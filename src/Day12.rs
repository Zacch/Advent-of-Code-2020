use std::fs;
use std::str::FromStr;

pub fn day12() {
    let contents = fs::read_to_string("Input/Day12.txt")
        .expect("Couldn't read the file");

    // Y axis is north(+)/south(-), X is east(+)/west(-)
    let mut pos = (0, 0);
    let mut dir = (1,0);

    for line in contents.lines() {
        navigate(&mut pos, &mut dir, line);
    }
    let part1 = i32::abs(pos.0) + i32::abs(pos.1);

    pos = (0, 0);
    let mut waypoint = (10, 1);
    for line in contents.lines() {
        navigate2(&mut pos, &mut waypoint, line);
    }
    let part2 = i32::abs(pos.0) + i32::abs(pos.1);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn navigate(pos: &mut (i32, i32), dir: &mut (i32, i32), instr: &str) {
    let parts = instr.split_at(1);
    let argument = i32::from_str(parts.1).unwrap();
    match parts.0 {
        "N" => { pos.1 += argument },
        "S" => { pos.1 -= argument },
        "E" => { pos.0 += argument },
        "W" => { pos.0 -= argument },
        "F" => {
            pos.0 += dir.0 * argument;
            pos.1 += dir.1 * argument;
        },
        "L" => {
            match argument {
                90 => { let old0 = dir.0; dir.0 = -dir.1; dir.1 = old0 }
                180 => { dir.0 = -dir.0; dir.1 = -dir.1 }
                270 => { let old0 = dir.0; dir.0 = dir.1; dir.1 = -old0 }
                _ => { println!("Unknown instruction: {:?}", parts) }
            }
        },
        "R" => {
            match argument {
                90 => { let old0 = dir.0; dir.0 = dir.1; dir.1 = -old0 }
                180 => { dir.0 = -dir.0; dir.1 = -dir.1 }
                270 => { let old0 = dir.0; dir.0 = -dir.1; dir.1 = old0 }
                _ => { println!("Unknown instruction: {:?}", parts) }
            }
        },
        &_ => { println!("Unknown instruction: {:?}", parts)}
    }
}

fn navigate2(pos: &mut (i32, i32), way: &mut (i32, i32), instr: &str) {
    let parts = instr.split_at(1);
    let argument = i32::from_str(parts.1).unwrap();
    match parts.0 {
        "N" => { way.1 += argument },
        "S" => { way.1 -= argument },
        "E" => { way.0 += argument },
        "W" => { way.0 -= argument },
        "F" => {
            pos.0 += way.0 * argument;
            pos.1 += way.1 * argument;
        },
        "L" => {
            match argument {
                90 => {
                    let old0 = way.0;
                    way.0 = -way.1;
                    way.1 = old0
                }
                180 => {
                    way.0 = -way.0;
                    way.1 = -way.1
                }
                270 => {
                    let old0 = way.0;
                    way.0 = way.1;
                    way.1 = -old0
                }
                _ => { println!("Unknown instruction: {:?}", parts) }
            }
        },
        "R" => {
            match argument {
                90 => {
                    let old0 = way.0;
                    way.0 = way.1;
                    way.1 = -old0
                }
                180 => {
                    way.0 = -way.0;
                    way.1 = -way.1
                }
                270 => {
                    let old0 = way.0;
                    way.0 = -way.1;
                    way.1 = old0
                }
                _ => { println!("Unknown instruction: {:?}", parts) }
            }
        },
        &_ => { println!("Unknown instruction: {:?}", parts) }
    }
}