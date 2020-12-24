use std::fs;
use std::collections::HashSet;

pub fn day24() {
    let contents = fs::read_to_string("Input/Day24.txt").expect("Couldn't read the file");

    let mut part1 = 0;
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
    // let mut part2 = 0;
    // println!("Part 2: {}", part2);
}