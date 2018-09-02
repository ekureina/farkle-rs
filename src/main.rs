extern crate random;

use random::Source;
use std::time::SystemTime;

fn main() {
    let mut dice_source = get_rand();
    let dice_rolls = roll_dice(&mut dice_source, 6, 6);
    for i in 0..6 {
        println!("{}", dice_rolls[i]);
    }
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