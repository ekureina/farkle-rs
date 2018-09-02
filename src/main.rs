extern crate random;

use random::Source;
use std::time::SystemTime;

fn main() {
    let mut dice_source = get_rand();
    let dice_rolls = roll_dice(&mut dice_source, 6, 6);
    for die in dice_rolls.iter() {
        println!("{}", die);
    }

    println!("{}", get_points(dice_rolls));

}

fn get_rand() -> random::Default {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    random::default().seed([now.as_secs(), now.subsec_millis() as u64])
}

fn roll_dice(source: &mut random::Default, side_n: u8, dice_num: usize) -> Vec<u8> {
    let mut dice = source.iter().take(dice_num).collect::<Vec<u8>>();
    for i in 0..dice_num {
        dice[i] = dice[i] % side_n + 1;
    }
    dice
}

fn get_points(dice: Vec<u8>) -> u16 {
    let mut freq = Vec::with_capacity(6);
    for _ in 0..6 {
        freq.push(0);
    }
    for element in dice.iter() {
        freq[(element-1) as usize] += 1;
    }
    let mut score: u16 = 0;
    for dots in 0..6 {
        if dots == 0 && freq[0] < 4 {
            score += 100 * freq[0];
            continue;
        }
        else if dots == 4 && freq[4] < 3 {
            score += 50 * freq[4];
            continue;
        }
        else if freq[dots] == 3 {
            score += 100 * (dots+1) as u16;
            continue;
        }
        else if freq[dots] == 4 {
            score += 1000;
            continue;
        }
        else if freq[dots] == 5 {
            score += 2000;
            continue;
        }
        else if freq[dots] == 6 {
            score += 3000;
            break;
        }
    }
    score
}