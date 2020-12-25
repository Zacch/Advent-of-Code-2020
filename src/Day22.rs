use std::collections::{VecDeque, HashSet};
use std::fs;
use std::str::FromStr;

pub fn day22() {
    let contents = fs::read_to_string("Input/Day22.txt").expect("Couldn't read the file");

    let mut p1: VecDeque<i32> = VecDeque::new();
    let mut p2: VecDeque<i32> = VecDeque::new();

    let mut iterator = contents.lines().into_iter();
    iterator.next();
    let mut line = iterator.next().unwrap();
    while line.len() > 0 {
        p1.push_back(i32::from_str(line).unwrap());
        line = iterator.next().unwrap();
    }
    iterator.next();
    line = iterator.next().unwrap();
    while line.len() > 0 {
        p2.push_back(i32::from_str(line).unwrap());
        line = match iterator.next() {
            None => { "" }
            Some(s) => { s }
        }
    }
    let mut p1_clone = p1.clone();
    let mut p2_clone = p2.clone();

    while p1.len() > 0 && p2.len() > 0 {
        play_round(p1.front() > p2.front(), &mut p1, &mut p2);
    }

    let mut winner = if p1.len() == 0 {p2} else {p1};
    println!("Part 1: {}", calculate_score(&mut winner));

    winner = recursive_combat(&mut p1_clone, &mut p2_clone);
    println!("Part 2: {}", calculate_score(&mut winner));
}

// Part 2: 36621
// Execution time: 422.725022975s


fn play_round(p1_wins: bool, p1: &mut VecDeque<i32>, p2: &mut VecDeque<i32>) {
    if p1_wins {
        let top_card = p1.pop_front().unwrap();
        p1.push_back(top_card);
        p1.push_back(p2.pop_front().unwrap());
    } else {
        let top_card = p2.pop_front().unwrap();
        p2.push_back(top_card);
        p2.push_back(p1.pop_front().unwrap());
    }
}

fn calculate_score(winner: &mut VecDeque<i32>) -> i32 {
    let mut score = 0;
    let mut multiplier = 1;
    while !winner.is_empty() {
        score += multiplier * winner.pop_back().unwrap();
        multiplier += 1;
    }
    score
}

fn recursive_combat(mut p1: &mut VecDeque<i32>, mut p2: &mut VecDeque<i32>) -> VecDeque<i32> {
    let mut history = HashSet::new();
    while p1.len() > 0 && p2.len() > 0 {
        let state = format!("{:?}{:?}", p1, p2);
        if history.contains(&state) {
            return p1.clone();
        }
        history.insert(state);
        play_recursive_round(&mut p1, &mut p2);
    }
    if p2.len() == 0 {p1.clone()} else {p2.clone()}
}

fn play_recursive_round(mut p1: &mut VecDeque<i32>, mut p2: &mut VecDeque<i32>) {
    if *p1.front().unwrap() < p1.len() as i32 && *p2.front().unwrap() < p2.len() as i32 {
        play_round(sub_game(p1, p2), &mut p1, &mut p2);
    } else {
        play_round(p1.front() > p2.front(), &mut p1, &mut p2);
    }
}

fn sub_game(p1_super_game: &VecDeque<i32>, p2_super_game: &VecDeque<i32>) -> bool {
    let mut history = HashSet::new();
    let mut p1 = VecDeque::new();
    for i in 1..=*p1_super_game.front().unwrap() as usize {
        p1.push_back(p1_super_game[i]);
    }
    let mut p2 = VecDeque::new();
    for i in 1..=*p2_super_game.front().unwrap() as usize {
        p2.push_back(p2_super_game[i]);
    }
    while p1.len() > 0 && p2.len() > 0 {
        let state = format!("{:?}{:?}", p1, p2);
        if history.contains(&state) {
            return true;
        }
        history.insert(state);
        play_recursive_round(&mut p1, &mut p2);
    }
    return p2.len() == 0
}
