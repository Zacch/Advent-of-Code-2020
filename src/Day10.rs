use std::fs;
use std::collections::BTreeSet;
use std::str::FromStr;

pub fn day10() {
    let contents = fs::read_to_string("Input/Day10.txt")
        .expect("Couldn't read the file");

    let mut input = BTreeSet::new();
    for line in contents.lines() {
        input.insert(i32::from_str(line).unwrap());
    }

    let mut differences = vec![0, 0, 0, 0];
    let mut runs = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut last_v = 0;
    let mut run = 0;
    for v in input {
        let difference = (v - last_v) as usize;
        differences[difference] += 1;
        if difference == 1 {
            run += 1;
        } else {
            runs[run] += 1;
            run = 0;
        }
        last_v = v;
    }
    runs[run] += 1;
    differences[3] += 1;

    println!("Part 1: {}", differences[1] * differences[3]);
    println!("Part 2: {}", 2i128.pow(runs[2]) * 4i128.pow(runs[3]) * 7i128.pow(runs[4]));
}