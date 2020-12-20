use std::fs;
use std::str::FromStr;
use std::iter::FromIterator;

#[derive(Debug)]
struct Tile {
    number: i32,
    grid: Vec<Vec<char>>,
    edges: Vec<i32>,
    connections: i32
}

pub fn day20() {
    let contents = fs::read_to_string("Input/Day20.txt").expect("Couldn't read the file");
    let mut tiles = vec![];
    let mut iterator = contents.lines().into_iter();

    loop {
        let mut line = match iterator.next() {
            None => {break}
            Some(l) => {l}
        };
        if line.is_empty() { continue }

        let number = i32::from_str(&line[5..9]).unwrap();

        let mut grid:Vec<Vec<char>> = vec![];
        for _ in 0..10 {
            line = iterator.next().unwrap();
            grid.push(line.chars().into_iter().collect());
        }
        let edges = calculate_edges(&grid);
        let tile = Tile { number, grid, edges, connections: 0 };
        tiles.push(tile);
    }

    for i in 0..tiles.len() - 1 {
        for j in i+1..tiles.len() {
            for edge in tiles[i].edges.clone() {
                if tiles[j].edges.contains(&edge) {
                    tiles[i].connections += 1;
                    tiles[j].connections += 1;
                }
            }
        }
    }

    let corners: Vec<Tile> = tiles.into_iter().filter(|t| t.connections == 4).collect();

    let mut part1:i64 = 1;
    for tile in corners {
        part1 *= tile.number as i64;
    }
    println!("Part 1: {}", part1);

    let mut part2 = 0;
    part2 += 0;
    println!("Part 2: {}", part2);
}

fn calculate_edges(grid: &Vec<Vec<char>>) -> Vec<i32> {
    let mut left = vec![];
    let mut right = vec![];
    for line in grid {
        left.push(line[0]);
        right.push(line[9]);
    }
    let mut edges = vec![];
    edges.push(encode_edge(&grid[0]));
    edges.push(encode_reverse_edge(&grid[0]));
    edges.push(encode_edge(&grid[9]));
    edges.push(encode_reverse_edge(&grid[9]));
    edges.push(encode_edge(&left));
    edges.push(encode_reverse_edge(&left));
    edges.push(encode_edge(&right));
    edges.push(encode_reverse_edge(&right));
    edges
}

fn encode_edge(edge: &Vec<char>) -> i32 {
    let top = String::from_iter(edge).replace('.', "0").replace('#', "1");
    i32::from_str_radix(&top, 2).unwrap()
}

fn encode_reverse_edge(edge: &Vec<char>) -> i32 {
    let mut flipped = edge.clone();
    flipped.reverse();
    encode_edge(&flipped)
}
