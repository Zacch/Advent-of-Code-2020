use std::fs;
use std::str::FromStr;
use std::iter::FromIterator;
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use std::panic::resume_unwind;

#[derive(Debug,Eq, Clone)]
struct Tile {
    number: i32,
    grid: Vec<Vec<char>>,
    edges: Vec<i32>,
    connections: i32,
    connecting_tiles: Vec<usize>,
    position: (usize, usize)
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.number.partial_cmp(&other.number)
    }
}
impl Ord for Tile {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number.cmp(&other.number)
    }
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
        let tile = Tile { number, grid, edges, connections: 0, connecting_tiles: vec![], position: (0,0) };
        tiles.push(tile);
    }

    tiles.sort();
    for i in 0..tiles.len() - 1 {
        for j in i+1..tiles.len() {
            for edge in tiles[i].edges.clone() {
                if tiles[j].edges.contains(&edge) {
                    tiles[i].connections += 1;
                    tiles[j].connections += 1;
                    if !tiles[i].connecting_tiles.contains(&j) { tiles[i].connecting_tiles.push(j)}
                    if !tiles[j].connecting_tiles.contains(&i) { tiles[j].connecting_tiles.push(i)}
                }
            }
        }
    }

    let corners: Vec<&Tile> = tiles.iter().filter(|t| t.connecting_tiles.len() == 2).collect();
    let mut part1:i64 = corners.iter().fold(1, |i, t| i * (t.number as i64));
    println!("Part 1: {}", part1);


   //  for tile in &tiles {
   //      println!("Tile {}: edges {:?}, connections {:?}", tile.number, tile.edges, tile.connecting_tiles);
   //  }
   //  println!();
    let flipped_tiles = tiles; // same_side_up(&tiles);

    for tile in &flipped_tiles {
        println!("Tile {}: edges {:?}, connections {:?}", tile.number, tile.edges, tile.connecting_tiles);
    }
    println!();

    let sorted_tiles = assemble_image(&flipped_tiles);

    print_image(&sorted_tiles);

    // print(&sorted_tiles[0]);
    // print(&rotate(&sorted_tiles[0]));
    // let mut part2 = 0;
    // part2 += 0;
    // println!("Part 2: {}", part2);
}

fn print(tile: &Tile) {
    println!("Tile {}: edges {:?}, connections {:?}", tile.number, tile.edges, tile.connecting_tiles);
    for row in &tile.grid {
        for square in row {
            print!(" {}", square);
        }
        println!();
    }
}

fn same_side_up(tiles: &Vec<Tile>) -> Vec<Tile> {
    let mut flipped_tiles: Vec<Tile> = vec![tiles[0].clone()];

    let mut frontier: Vec<usize> = vec![];
    frontier.push(0);
    let mut reached = HashSet::new();
    reached.insert(0 as usize);
    while !frontier.is_empty() {
        let mut current = frontier.remove(0);
        let current_number = tiles[current].number;
        println!("Current: {}", current_number);
        let current_flipped = flipped_tiles.iter().find(|t|t.number == current_number).unwrap().clone();
        for next in &tiles[current].connecting_tiles {
            if reached.contains(&next) { continue }
            let mut flip_tile = false;
            for edge in 0..4 {
                if tiles[*next].edges[4..8].contains(&current_flipped.edges[edge]) {
                    flip_tile = true;
                    break;
                }
            }
            flipped_tiles.push(
                if flip_tile { flip(&tiles[*next]) } else {
                    println!("Storing {}", tiles[*next].number);
                    tiles[*next].clone() }
            );
            frontier.push(*next);
            reached.insert(*next);
        }
    }
    flipped_tiles.sort();
    flipped_tiles
}

fn assemble_image(tiles: &Vec<Tile>) -> Vec<Tile> {
    let tile_columns = (tiles.len() as f32) as usize;

    let mut reached: Vec<Tile> = vec![tiles[0].clone()];
    reached[0].position = (tile_columns, tile_columns);
    let mut frontier: Vec<Tile> = vec![reached[0].clone()];

    while !frontier.is_empty() {
        let mut current = frontier.remove(0);
        println!("current is {} at position {:?}", current.number, current.position);

        for next_index in &current.connecting_tiles {
            let mut next = tiles[*next_index].clone();
            if reached.contains(&next) { continue; }
            let rotated_next = rotate_and_move(&mut next, &current);
            println!("rotated_next is {} at position {:?}", rotated_next.number, rotated_next.position);
            frontier.push(rotated_next.clone());
            reached.push(rotated_next);
        }
    }

    reached.sort_by(|a, b|
        (a.position.1 * 100 + a.position.0).cmp(&(b.position.1 * 100 + b.position.0)));
    reached
}


fn rotate_and_move(next: &mut Tile, current: &Tile) -> Tile {
    let mut result = next.clone();

    let mut current_edge = 42;
    for i in 0..4 {
        if next.edges.contains(&current.edges[i]) {
            current_edge = i;
            break;
        }
    }
    let mut next_edge = 42;
    for i in 0..8 {
        if next.edges[i] == current.edges[current_edge] {
            next_edge = i;
            break;
        }
    }
    match current_edge {
        0 => {
            result.position = (current.position.0, current.position.1 - 1);
            match next_edge {
                0 => { result = flip(&result); }
                1 => { result = rotate(&flip(&result)); }
                2 => { result = rotate(&rotate(&flip(&result))); }
                3 => { result = flip(&rotate(&result)); }
                4 => { result = rotate(&rotate(&result)); }
                5 => { result = rotate(&result); }
                6 => { }
                7 => { result =  rotate(&rotate(&rotate(&result))); }
                _ => { panic!(); }
            }
        }
        1 => {
            result.position = (current.position.0 + 1, current.position.1);
            match next_edge {
                0 => { result = rotate(&flip(&result)); }
                1 => { result = rotate(&rotate(&flip(&result))); }
                2 => { result = flip(&rotate(&result)); }
                3 => { result = flip(&result); }
                4 => { result =  rotate(&rotate(&rotate(&result))); }
                5 => { result = rotate(&rotate(&result)); }
                6 => { result = rotate(&result); }
                7 => { }
                _ => { panic!(); }
            }
        }
        2 => {
            result.position = (current.position.0, current.position.1 + 1);
            match next_edge {
                0 => { result = rotate(&rotate(&flip(&result))); }
                1 => { result = flip(&rotate(&result)); }
                2 => { result = flip(&result); }
                3 => { result = rotate(&flip(&result)); }
                4 => { }
                5 => { result =  rotate(&rotate(&rotate(&result))); }
                6 => { result = rotate(&rotate(&result)); }
                7 => { result = rotate(&result); }
                _ => { panic!(); }
            }
        }
        3 => {
            result.position = (current.position.0 - 1, current.position.1);
            match next_edge {
                0 => { result = flip(&rotate(&result)); }
                1 => { result = flip(&result); }
                2 => { result = rotate(&flip(&result)); }
                3 => { result = rotate(&rotate(&flip(&result))); }
                5 => { }
                6 => { result = rotate(&rotate(&rotate(&result))); }
                7 => { result = rotate(&rotate(&result)); }
                4 => { result = rotate(&result); }
                _ => { panic!(); }
            }
        }
        _ => { panic!(); }
    }

    // println!("Current {}, edge {} matches {}, edge {} -> rotate {} times ",
    //          current.number, current_edge, next.number, next_edge, (next_edge + 6 - current_edge).rem_euclid(4));
    result
}
/* Orientations
 Normal     Flipped
  0            4
3   1        7   5
  2            6

 */

fn rotate(tile: &Tile) -> Tile {
    println!("Rotating {}", tile.number);
    let size = tile.grid.len();
    let mut result = tile.clone();
    for y in 0..size {
        for x in 0..size {
            result.grid[y][size - 1 - x] = tile.grid[x][y];
        }
    }
    let e = &tile.edges;
    result.edges = vec![e[3], e[0], e[1], e[2],
                        e[7], e[4], e[5], e[6]];

    result
}

fn flip(tile: &Tile) -> Tile {
    println!("Flipping {}", tile.number);
    let mut result = tile.clone();
    result.grid.reverse();
    let e = &tile.edges;
    result.edges = vec![e[6], e[5], e[4], e[7],
                        e[2], e[1], e[0], e[3]];
    result
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
    edges.push(encode_edge(&right));
    edges.push(encode_reverse_edge(&grid[9]));
    edges.push(encode_reverse_edge(&left));
    edges.push(encode_reverse_edge(&grid[0]));
    edges.push(encode_reverse_edge(&right));
    edges.push(encode_edge(&grid[9]));
    edges.push(encode_edge(&left));
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


fn print_image(tiles: &Vec<Tile>) {
    let tile_columns = (tiles.len() as f32).sqrt() as usize;

    for row in 0..tile_columns {
        for line in 0..10 {
            for tile_column in 0..tile_columns {
                for i in 0..10 { print!(" {}", tiles[row * tile_columns + tile_column].grid[line][i]); }
                print!("|");
            }
            println!();
        }
        println!("---------------------------------------------------------------");
    }
    for row in 0..tile_columns {
        for tile_column in 0..tile_columns {
            print!("{} ", tiles[row * tile_columns + tile_column].number);
        }
        println!();
    }
}
