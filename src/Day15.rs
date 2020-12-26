use num_format::{Locale, ToFormattedString};
use std::collections::HashMap;

pub fn day15() {

    let input:Vec<usize> = vec![0,3,1,6,7,5];

    let mut history = HashMap::new();
    for turn in 0..input.len() - 1 {
        history.insert(input[turn], turn + 1);
    }
    let mut last_spoken = *input.last().unwrap();

    for turn in input.len() + 1..30000001 {
        let next = match history.remove(&last_spoken) {
            None => {0}
            Some(last_turn) => {turn - 1 - last_turn }
        };
        if turn == 2020 { println!("Part 1: {}", next); }
        if turn % 1_000_000 == 0 {
            println!("Calculating... Now at turn {}", turn.to_formatted_string(&Locale::en));
        }
        history.insert(last_spoken, turn - 1);
        last_spoken = next.to_owned();
    }
    println!("Part 2: {}", last_spoken);
}