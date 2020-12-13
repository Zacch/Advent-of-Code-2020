use std::fs;
use std::cmp::{max, min};

pub fn day11() {
    let contents = fs::read_to_string("Input/Day11.txt")
        .expect("Couldn't read the file");

    let mut part1 = 0;
    let mut part2 = 0;

    let mut lines = vec![];
    for line in contents.lines() {
        lines.push(line.to_owned());
    }

    let width = lines[0].len();
    let height = lines.len();

    let mut grid = vec![vec![' '; 100]; 100];

    let mut y = 0;
    for line in lines {
        let mut x = 0;
        for char in line.trim().chars() {
            grid[x][y] = char.to_owned();
            x += 1;
        }
        y += 1;
    }

    while part1 == 0 {
        let new_grid = evolve(&grid, height, width);
        if new_grid == grid {
            part1 = count_occupied(grid);
        }
        grid = new_grid;
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

}

fn count_occupied(grid: Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    for line in grid {
        for c in line {
            if c == '#' { sum += 1; }
        }
    }
    sum
}

fn evolve(grid: &Vec<Vec<char>>, height: usize, width: usize) -> Vec<Vec<char>> {
    let mut after = vec![vec![' '; 100]; 100];
    for y in 0..height {
        for x in 0..width {
            after[x][y] =
                match grid[x][y] {
                    'L' => if occupied_neighbors(&grid, height, width, x, y) == 0 {'#'} else {'L'},
                    '#' => if occupied_neighbors(&grid, height, width, x, y) > 4 {'L'} else {'#'},
                    _ =>  '.'
                }
        }
    }
    after
}

fn occupied_neighbors(grid: &Vec<Vec<char>>, height: usize, width: usize, x:usize, y:usize) -> i8 {
    let mut count = 0;
    for y1 in max(y, 1) - 1..min(height, y + 2) {
        for x1 in max(x, 1) - 1..min(width, x + 2) {
            if grid[x1][y1] == '#' {
                count += 1;
            }
        }
    }
    count
}

fn print_grid(grid: &mut Vec<Vec<char>>, height: usize, width: usize) {
    for y in 0..height {
        for x in 0..width {
            print!("{}", grid[x][y]);
        }
        println!();
    }
}