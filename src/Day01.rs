use std::fs;

pub fn day01() {
    let contents = fs::read_to_string("Input/Day01.txt")
        .expect("Couldn't read the file");

    let mut numbers: Vec<i32> = Vec::new();
    for line in contents.lines() {
        let number: i32 = line.parse().expect(&*format!("{} is not a number!", line));
        numbers.push(number);
    }

    let mut part1: (i32, i32) = (0,0);
    let mut part2: (i32, i32, i32) = (0,0,0);
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                part1 = (numbers[i], numbers[j]);
            }
            for k in j + 1..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    part2 = (numbers[i], numbers[j], numbers[k]);
                }
            }
        }
    }
    println!("Part 1: {} * {} = {}", part1.0, part1.1, part1.0 * part1.1);
    println!("Part 2: {} * {} * {} = {}", part2.0, part2.1, part2.2, part2.0 * part2.1 * part2.2);
}
