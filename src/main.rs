extern crate random;

use random::Source;
use std::fmt;
use std::io::{self, Write};
use std::time::SystemTime;

fn main() {
    let mut dice_source = get_rand();
    println!("Welcome to Farkle!");
    let mut players = get_players(get_num_players());
    while !is_game_over(&players) {
        for player in players.iter_mut() {
            let mut dice_num: usize = 6;
            let mut round_score = 0;
            let mut re_roll = true;
            println!("{}'s Turn!\nCurrent Score: {}", player, player.score);
            while (!player.on_board() && round_score < 500) || re_roll == true {
                println!("Dice Roll:");
                let dice_rolls = roll_dice(&mut dice_source, 6, dice_num);
                for die in dice_rolls.iter() {
                    print!("{} ", die);
                }
                let mut score_dice = String::new();
                print!("\nWhich Dice to Use: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut score_dice)
                    .expect("Please Enter the Dice");
                if score_dice.trim() == "" {
                    round_score = 0;
                    break;
                }
                let mut scoring_dice = Vec::<u8>::with_capacity(dice_num);
                for die in score_dice.split_whitespace() {
                    scoring_dice.push(die.parse().expect("NaN"));
                }
                round_score = score_roll(
                    &scoring_dice,
                    player.on_board(),
                    round_score,
                    dice_rolls.len() as u8,
                );
                if round_score == 0 {
                    break;
                }
                dice_num -= scoring_dice.len();
                println!("Score this round: {}", round_score);
                if player.on_board() || round_score >= 500 {
                    let mut should_re_roll = String::new();
                    print!("Roll Again?: ");
                    io::stdout().flush().unwrap();
                    io::stdin()
                        .read_line(&mut should_re_roll)
                        .expect("Bad Answer");
                    re_roll = should_re_roll.trim().to_lowercase() == "y";
                }
                if dice_num == 0 {
                    dice_num = 6;
                }
            }
            player.score += round_score;
        }
    }
    for player in players.iter() {
        println!("{}", player);
    }
}

fn get_rand() -> random::Default {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    random::default().seed([now.as_secs(), now.subsec_millis() as u64])
}

fn roll_dice(source: &mut random::Default, side_n: u8, dice_num: usize) -> Vec<u8> {
    let mut dice = source.iter().take(dice_num).collect::<Vec<u8>>();
    for i in 0..dice_num {
        dice[i] = dice[i] % side_n + 1;
    }
    dice
}

fn score_roll(dice: &[u8], on_board: bool, current_score: u16, rolled_dice: u8) -> u16 {
    let mut score = current_score;
    if on_board && rolled_dice == 2 {
        let mut added: u16 = 0;
        for die in dice.iter() {
            match die {
                1 => added += 100,
                5 => added += 50,
                _ => (),
            }
        }
        match added {
            0 => return 0,
            _ => return current_score * 2 + added,
        }
    } else if on_board && rolled_dice == 1 {
        match dice[0] {
            1 => return current_score * 3 + 100,
            5 => return current_score * 3 + 50,
            _ => return 0,
        }
    } else {
        let mut freq = vec![0; 6];
        for die in dice.iter() {
            freq[(die - 1) as usize] += 1;
        }
        let mut doubles: Vec<u8> = Vec::with_capacity(3);
        let mut triples: Vec<u8> = Vec::with_capacity(2);
        let mut singles: Vec<u8> = Vec::with_capacity(6);
        for dots in 0..6 {
            match freq[dots] {
                1 => singles.push(dots as u8),
                2 => doubles.push(dots as u8),
                3 => triples.push(dots as u8),
                4 => score += 1000,
                5 => score += 2000,
                6 => score += 3000,
                _ => (),
            }
        }
        if score == 0 && dice.len() == 6 {
            if doubles.len() == 3 {
                score = 1500;
            } else if triples.len() == 2 {
                score = 2500;
            } else if singles.len() == 6 {
                score = 1500;
            } else {
                for triple in triples.iter() {
                    score += 100 * (triple + 1) as u16;
                }
                if freq[0] <= 3 {
                    score += 100 * freq[0];
                    if freq[0] == 3 {
                        score -= 100
                    }
                }
                if freq[4] < 3 {
                    score += 50 * freq[4];
                }
            }
        } else {
            for triple in triples.iter() {
                score += 100 * (triple + 1) as u16;
            }
            if freq[0] <= 3 {
                score += 100 * freq[0];
                if freq[0] == 3 {
                    score -= 100
                }
            }
            if freq[4] < 3 {
                score += 50 * freq[4];
            }
        }
    }
    score
}

fn get_num_players() -> usize {
    let mut player_num = String::new();
    print!("How many players?: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut player_num).expect("Bad input!");
    player_num.trim().parse().expect("Not a number!")
}

fn get_players(player_number: usize) -> Vec<Player> {
    let mut players = Vec::<Player>::with_capacity(player_number);
    for num in 0..player_number {
        let mut player_name = String::new();
        print!("What is Player #{}'s Name?: ", (num + 1));
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut player_name)
            .expect("Please input a player name.");
        players.push(Player::new(player_name.trim()));
    }
    players
}

fn is_game_over(players: &[Player]) -> bool {
    for player in players.iter() {
        if player.score >= 10000 {
            return true;
        }
    }
    false
}

struct Player {
    name: String,
    score: u16,
}

impl Player {
    pub fn new(player_name: &str) -> Player {
        Player {
            name: player_name.to_string(),
            score: 0,
        }
    }

    pub fn on_board(&self) -> bool {
        self.score >= 500
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
