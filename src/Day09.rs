use std::fs;
use std::str::FromStr;

pub fn day09() {
    let contents = fs::read_to_string("Input/Day09.txt").expect("Couldn't read the file");
    let mut input = vec![];
    for line in contents.lines() {
        input.push(i64::from_str(line).unwrap());
    }

    let mut part1 = 0;
    for index in 25..=input.len() - 1 {
        let current = input[index];
        let mut found = false;
        for x in index - 25..index - 1 {
            for y in x + 1..index {
                if input[x] + input[y] == current {
                    found = true;
                }
            }
        }
        if !found {
            part1 = current;
        }
    }

    let mut interval = (0,0);
    for start in 0..=input.len() - 2 {
        let mut sum = input[start];
        if sum == part1 {continue}
        let mut end = start + 1;
        while sum < part1 && end < input.len() - 1 {
            sum += input[end];
            end += 1
        }
        if sum == part1 {
            interval = (start, end);
            println!("Found interval: {:?}", interval);
            break;
        }
    }
    let v = &input[interval.0..=interval.1 - 1];
    let part2 = v.iter().min().unwrap() + v.iter().max().unwrap();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}