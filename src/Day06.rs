use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn day06() {
    let contents = fs::read_to_string("Input/Day06.txt")
        .expect("Couldn't read the file");

    let mut part1 = 0;
    let mut part2 = 0;
    let mut yeses = HashSet::new();
    let mut all_yes =
        HashSet::from(HashSet::from_iter("abcdefghijklmnopqrstuvwxyz".chars()));

    for line in contents.lines() {
        if line.is_empty() {
            part1 += yeses.len();
            part2 += all_yes.len();
            yeses.clear();
            all_yes = HashSet::from_iter("abcdefghijklmnopqrstuvwxyz".chars());
        } else {
            let chars_in_line = &HashSet::from_iter(line.chars());
            yeses = HashSet::from_iter(yeses.union(chars_in_line).copied());
            all_yes = HashSet::from_iter(all_yes.intersection(chars_in_line).copied());
        }
    }
    part1 += yeses.len();
    part2 += all_yes.len();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}