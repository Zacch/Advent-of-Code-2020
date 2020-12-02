use std::fs;

pub fn day02() {
    let contents = fs::read_to_string("Input/Day02.txt")
        .expect("Couldn't read the file");

    let mut part1 = 0;
    let mut part2 = 0;
    for line in contents.lines() {
        let words: Vec<&str> = line.split(' ').collect();
        let numbers: Vec<&str> = words[0].split('-').collect();

        let min: usize = numbers[0].parse().expect(&*format!("{} is not a number!", line));
        let max: usize = numbers[1].parse().expect(&*format!("{} is not a number!", line));
        let char = words[1].chars().next().unwrap();

        let count  = words[2].matches(char).count();
        if count >= min && count <= max {
            part1 += 1;
        }

        if (char == words[2].chars().nth(min - 1).unwrap()) ^
            (char == words[2].chars().nth(max - 1).unwrap()) {
            part2 += 1;
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}