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
    let mut grid2 = vec![vec![' '; 100]; 100];

    let mut y = 0;
    for line in lines {
        let mut x = 0;
        for char in line.trim().chars() {
            grid[x][y] = char.to_owned();
            grid2[x][y] = char.to_owned();
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

    while part2 == 0 {
        let new_grid = evolve2(&grid2, height, width);
        if new_grid == grid2 {
            part2 = count_occupied(grid2);
        }
        grid2 = new_grid;
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
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

// NOTE: This method counts the seat at (x, y) also!
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

fn evolve2(grid: &Vec<Vec<char>>, height: usize, width: usize) -> Vec<Vec<char>> {
    let mut after = vec![vec![' '; 100]; 100];
    for y in 0..height {
        for x in 0..width {
            after[x][y] =
                match grid[x][y] {
                    'L' => if occupied_line_of_sight(&grid, height, width, x, y) == 0 {'#'} else {'L'},
                    '#' => if occupied_line_of_sight(&grid, height, width, x, y) > 4 {'L'} else {'#'},
                    _ =>  '.'
                }
        }
    }
    after
}

fn occupied_line_of_sight(grid: &Vec<Vec<char>>, height: usize, width: usize, x:usize, y:usize) -> i8 {
    let mut count = 0;
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0),
                                    (-1, 1), (-1, -1), (1, -1), (1, 1)];
    for direction in directions {
        let mut point = (x as i32 + direction.0, y as i32 + direction.1);
        let mut found_seat = false;
        while !found_seat && point.0 >= 0 && point.1 >= 0 &&
            point.0 < width as i32 && point.1 < height as i32 {
            match grid[point.0 as usize][point.1 as usize] {
                'L' => found_seat = true,
                '#' => {found_seat = true; count += 1; },
                _ => ()
            }
            point = (point.0 + direction.0, point.1 + direction.1);
        }
    }
    count
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

/*
fn print_grid(grid: &mut Vec<Vec<char>>, height: usize, width: usize) {
    for y in 0..height {
        for x in 0..width {
            print!("{}", grid[x][y]);
        }
        println!();
    }
}
*/