use std::cmp::Ordering;
use std::fs;
use std::iter::FromIterator;
use std::str::FromStr;

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
    let part1:i64 = corners.iter().fold(1, |i, t| i * (t.number as i64));
    println!("Part 1: {}", part1);


    let mut image = assemble_image(&tiles);

    for _ in 0..4 {
        let monsters = count_monsters(&mut image);
        if monsters > 0 {
            // println!("Found {} monsters!", monsters);
            // for line in &image {
            //     println!("{}", String::from_iter(line.clone()));
            // }
            println!("Part 2: {}", count_waves(&image));
        }
        image = rotate_image(&image);
    }

    image.reverse();
    for _ in 0..4 {
        let monsters = count_monsters(&mut image);
        if monsters > 0 {
            // println!("Found {} monsters!", monsters);
            // for line in &image {
            //     println!("{}", String::from_iter(line.clone()));
            // }
            println!("Part 2: {}", count_waves(&image));
        }
        image = rotate_image(&image);
    }
}

fn count_waves(image: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    for line in image {
        result += line.iter().filter(|c|**c=='#').count();
    }
    result
}

//   01234567890123456789
// 0                   #
// 1 #    ##    ##    ###
// 2  #  #  #  #  #  #
const MONSTER_WIDTH: usize = 20;
const MONSTER_HEIGHT: usize = 3;
fn count_monsters(image: &mut Vec<Vec<char>>) -> i32 {
    let monster_pixels: Vec<(usize, usize)> =
        vec![(18, 0),
             (0,1), (5,1), (6,1), (11,1), (12,1), (17,1), (18,1), (19,1),
             (1,2), (4,2), (7,2), (10,2), (13,2), (16,2)];
    let mut result = 0;
    for x in 0..image.len() - MONSTER_WIDTH {
        for y in 0..image.len() - MONSTER_HEIGHT {
            let mut monster = true;
            for pixel in &monster_pixels {
                if image[y + pixel.1][x + pixel.0] == '.' {
                    monster = false;
                }
            }
            if monster {
                result += 1;
                for pixel in &monster_pixels {
                    image[y + pixel.1][x + pixel.0] = 'O';
                }
            }
        }
    }
    result
}

fn assemble_image(tiles: &Vec<Tile>) -> Vec<Vec<char>> {
    let tile_columns = (tiles.len() as f32) as usize;

    let mut reached: Vec<Tile> = vec![tiles[0].clone()];
    reached[0].position = (tile_columns, tile_columns);
    let mut frontier: Vec<Tile> = vec![reached[0].clone()];

    while !frontier.is_empty() {
        let current = frontier.remove(0);

        for next_index in &current.connecting_tiles {
            let mut next = tiles[*next_index].clone();
            if reached.contains(&next) { continue; }
            let rotated_next = rotate_and_move(&mut next, &current);
            frontier.push(rotated_next.clone());
            reached.push(rotated_next);
        }
    }

    reached.sort_by(|a, b|
        (a.position.1 * 100 + a.position.0).cmp(&(b.position.1 * 100 + b.position.0)));
    // print_tiled_image(&reached);

    render_image(&reached)
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
    result
}

fn rotate(tile: &Tile) -> Tile {
    let mut result = tile.clone();
    result.grid = rotate_image(&tile.grid);
    let e = &tile.edges;
    result.edges = vec![e[3], e[0], e[1], e[2],
                        e[7], e[4], e[5], e[6]];
    result
}

fn rotate_image(image: &Vec<Vec<char>>) -> Vec<Vec<char>>{
    let size = image.len();
    let mut result = image.clone();
    for y in 0..size {
        for x in 0..size {
            result[y][size - 1 - x] = image[x][y];
        }
    }
    result
}


fn flip(tile: &Tile) -> Tile {
    let mut result = tile.clone();
    result.grid.reverse();
    let e = &tile.edges;
    result.edges = vec![e[6], e[5], e[4], e[7],
                        e[2], e[1], e[0], e[3]];
    result
}

/* Edges (going clockwise)
 Normal     Flipped
  0            4
3   1        7   5
  2            6
 */

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

#[allow(dead_code)]
fn print_tiled_image(tiles: &Vec<Tile>) {
    let tile_columns = (tiles.len() as f32).sqrt() as usize;

    for row in 0..tile_columns {
        for line in 0..10 {
            for tile_column in 0..tile_columns {
                for i in 0..10 { print!(" {}", tiles[row * tile_columns + tile_column].grid[line][i]); }
                print!("  ");
            }
            println!();
        }
        println!();
    }
    for row in 0..tile_columns {
        for tile_column in 0..tile_columns {
            print!("{} ", tiles[row * tile_columns + tile_column].number);
        }
        println!();
    }
}

fn render_image(tiles: &Vec<Tile>) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = vec![];

    let tile_columns = (tiles.len() as f32).sqrt() as usize;

    for row in 0..tile_columns {
        for line in 1..9 {
            let mut result_line: Vec<char> = vec![];
            for tile_column in 0..tile_columns {
                for i in 1..9 {
                    result_line.push(tiles[row * tile_columns + tile_column].grid[line][i]);
                }
            }
            result.push(result_line);
        }
    }
    result
}
