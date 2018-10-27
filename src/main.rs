extern crate rand;

use rand::distributions::{Distribution, Uniform};
use std::io::{self, Write};

mod player;
use player::Player;

fn main() {
    let dice_distribution = get_distribution(6);
    println!("Welcome to Farkle!");
    let mut players = get_players(get_num_players());
    while !is_game_over(&players) {
        for player in players.iter_mut() {
            let mut dice_num: usize = 6;
            let mut round_score = 0;
            let mut re_roll = true;
            println!("{}'s Turn!\nCurrent Score: {}", player, player.score());
            while (!player.on_board() && round_score < 500) || re_roll == true {
                println!("Dice Roll:");
                let mut dice_rolls = roll_dice(&dice_distribution, dice_num);
                for die in dice_rolls.iter() {
                    print!("{} ", die);
                }
                let mut scored = score_roll(&dice_rolls, player.on_board(), round_score, dice_num as u8);
                if scored.0 == 0 {
                    println!("\nNo Score this roll. {}'s turn over.", player);
                    break;
                }
				print!("\nScore: {}\nDice Used: {}", scored.0, scored.1);
				if scored.1 != 1 {
					let mut unscore_dice = String::new();
					print!("\nDice to not Score: ");
					io::stdout().flush().unwrap();
					io::stdin()
						.read_line(&mut unscore_dice)
						.expect("Please Enter the Dice");
					if unscore_dice.trim() != "" {
						for die in unscore_dice.split_whitespace() {
							// Use remove_item when Available
							match dice_rolls.iter_mut().position(|&mut roll| roll == die.parse().expect("NaN")) {
								Some(position) => dice_rolls.remove(position),
								_ => 0,
							};
						}
					scored = score_roll(&dice_rolls, player.on_board(), round_score, dice_num as u8);
					}
				} else {
					print!("\n");
				}
				round_score = scored.0;
				dice_num -= scored.1;
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
            if re_roll == false { // Player picked to stop playing
				player.increment_score(round_score);
			}
        }
    }
    for player in players.iter() {
        println!("{}", player);
    }
}

fn get_distribution(die_sides: u8) -> Uniform<u8> {
    Uniform::new_inclusive(1, die_sides)
}

fn roll_dice(distribution: &Uniform<u8>, dice_num: usize) -> Vec<u8> {
    let mut dice = Vec::<u8>::with_capacity(dice_num);
    let mut rng = rand::thread_rng();
    for _ in 0..dice_num {
        dice.push(distribution.sample(&mut rng));
    }
    dice
}

fn score_roll(dice: &[u8], on_board: bool, current_score: u16, rolled_dice: u8) -> (u16, usize) {
    let mut score = current_score;
    let mut used_dice = 0;
    if on_board && rolled_dice == 2 {
        let mut added: u16 = 0;
        for die in dice.iter() {
            match die {
                1 => {
                        added += 100;
                        used_dice += 1;
                    },
                5 => {
                        added += 50;
                        used_dice += 1;
                    },
                _ => (),
            }
        }
        match added {
            0 => return (0, 0),
            _ => return (current_score * 2 + added, used_dice),
        }
    } else if on_board && rolled_dice == 1 {
        match dice[0] {
            1 => return (current_score * 3 + 100, 1),
            5 => return (current_score * 3 + 50, 1),
            _ => return (0, 0),
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
                4 => {
                        score += 1000;
                        used_dice += 4;
                },
                5 => {
                        score += 2000;
                        used_dice += 5;
                },
                6 => return (3000, 6),
                _ => (),
            }
        }
        if score == 0 && dice.len() == 6 {
            if doubles.len() == 3 {
                return (1500, 6);
            } else if triples.len() == 2 {
                return (2500, 6);
            } else if singles.len() == 6 {
                return (1500, 6);
            } else {
                for triple in triples.iter() {
                    score += 100 * (triple + 1) as u16;
                    used_dice += 3
                }
                if freq[0] <= 3 {
                    score += 100 * freq[0];
                    used_dice += freq[0] as usize;
                    if freq[0] == 3 {
                        score -= 100;
                        used_dice -= 3;
                    }
                }
                if freq[4] < 3 {
                    score += 50 * freq[4];
                    used_dice += freq[4] as usize;
                }
            }
        } else {
            for triple in triples.iter() {
                score += 100 * (triple + 1) as u16;
                used_dice += 3;
            }
            if freq[0] <= 3 {
                score += 100 * freq[0];
                used_dice += freq[0] as usize;
                if freq[0] == 3 {
                    score -= 100;
                    used_dice -= 3;
                }
            }
            if freq[4] < 3 {
                score += 50 * freq[4];
                used_dice += freq[4] as usize;
            }
        }
    }
    if score == current_score {
        return (0, 0);
    }
    (score, used_dice)
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
        if player.score() >= 10000 {
            return true;
        }
    }
    false
}
