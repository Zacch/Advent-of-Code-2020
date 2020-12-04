use std::fs;

pub fn day03() {
    let contents = fs::read_to_string("Input/Day03.txt")
        .expect("Couldn't read the file");

    let part1 = count_trees(&contents, 3, 1);
    let part2 =
            count_trees(&contents, 1, 1) *
            count_trees(&contents, 3, 1) *
            count_trees(&contents, 5, 1) *
            count_trees(&contents, 7, 1) *
            count_trees(&contents, 1, 2);


    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn count_trees(contents: &String, right: usize, down: usize) -> u32 {
    let mut result: u32 = 0;

    let mut x = 0;
    let mut y = 0;
    while y < contents.lines().count() {
        let line = contents.lines().nth(y).unwrap();
        if line.chars().nth(x).unwrap() == '#' {
            result += 1;
        }
        x = (x + right) % line.chars().count();
        y += down;
    }
    result
}
