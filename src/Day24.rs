use std::fs;
use std::collections::HashSet;

pub fn day24() {
    let contents = fs::read_to_string("Input/Day24.txt").expect("Couldn't read the file");

    let mut flipped_tiles = HashSet::new();
    for line in contents.lines() {
        let mut iterator = line.chars().into_iter();
        let mut position = (0, 0);
        loop {
            match iterator.next() {
                None => { break; }
                Some(c) => {
                    match c {
                        'e' => position.0 += 1,
                        'w' => position.0 -= 1,
                        's' => {
                            position.1 -= 1;
                            match iterator.next().unwrap() {
                                'e' => if position.1 & 1 == 0 { position.0 += 1 },
                                'w' => if position.1 & 1 == 1 { position.0 -= 1 },
                                _ => panic!()
                            }
                        }
                        'n' => {
                            position.1 += 1;
                            match iterator.next().unwrap() {
                                'e' => if position.1 & 1 == 0 { position.0 += 1 },
                                'w' => if position.1 & 1 == 1 { position.0 -= 1 },
                                _ => panic!()
                            }
                        }
                        _ => panic!()
                    }
                }
            }
        }
        if flipped_tiles.contains(&position) {
            let _ = flipped_tiles.remove(&position);
        } else {
            flipped_tiles.insert(position.to_owned());
        }
    }
    println!("Part 1: {}", flipped_tiles.len());

    for _ in 0..100 {
        flipped_tiles = flip_floor(&flipped_tiles);
    }
    println!("Part 2: {}", flipped_tiles.len());
}

fn flip_floor(before: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut after = HashSet::new();
    let mut min = (0, 0);
    let mut max = (0, 0);
    for tile in before {
        if min.0 > tile.0 { min.0 = tile.0 }
        if min.1 > tile.1 { min.1 = tile.1 }
        if max.0 < tile.0 { max.0 = tile.0 }
        if max.1 < tile.1 { max.1 = tile.1 }
    }

    for x in min.0 - 1..=max.0 + 1 {
        for y in min.1 - 1..=max.1 + 1 {
            let tile = (x, y);
            if before.contains(&tile) {
                match count_neighbors(tile, before) {
                    1 | 2 => { after.insert(tile); },
                    _ => {}
                }
            } else {
                if count_neighbors(tile, before) == 2 {
                    after.insert(tile);
                }
            }
        }
    }

    after
}

fn count_neighbors(tile: (i32, i32), floor: &HashSet<(i32, i32)>) -> i32 {
    let mut count = 0;
    if floor.contains(&(tile.0 - 1, tile.1)) { count += 1 }
    if floor.contains(&(tile.0 + 1, tile.1)) { count += 1 }
    if floor.contains(&(tile.0, tile.1 - 1)) { count += 1 }
    if floor.contains(&(tile.0, tile.1 + 1)) { count += 1 }
    if tile.1 % 2 == 0 {
        if floor.contains(&(tile.0 - 1, tile.1 - 1)) { count += 1 }
        if floor.contains(&(tile.0 - 1, tile.1 + 1)) { count += 1 }
    } else {
        if floor.contains(&(tile.0 + 1, tile.1 - 1)) { count += 1 }
        if floor.contains(&(tile.0 + 1, tile.1 + 1)) { count += 1 }
    }
    count
}