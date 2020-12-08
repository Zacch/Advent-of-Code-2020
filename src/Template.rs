use std::fs;

pub fn day08() {
    let contents = fs::read_to_string("Input/Day08.txt")
        .expect("Couldn't read the file");

    let mut part1 = 0;
    let mut part2 = 0;
    for line in contents.lines() {
        let words: Vec<&str> = line.split(' ').collect();
        println!("{:?}", &words);
        part1 += 1;
        part2 += 1;
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}