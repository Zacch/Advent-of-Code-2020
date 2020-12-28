use std::fs;
use std::collections::HashSet;
use std::cmp::{min, max};

pub fn day17() {
    let contents = fs::read_to_string("Input/Day17.txt").expect("Couldn't read the file");

    let mut cubes:HashSet<(i32,i32,i32)> = HashSet::new();
    let mut hypercubes:HashSet<(i32,i32,i32,i32)> = HashSet::new();
    let mut minimum = (0, 0, 0);
    let mut maximum = (0, -1, 0);

    for line in contents.lines() {
        maximum.0 = line.len() as i32;
        maximum.1 += 1;
        let mut x = 0;
        for c in line.chars() {
            if c == '#' {
                cubes.insert((x, maximum.1, 0));
                hypercubes.insert((x, maximum.1, 0, 0));
            }
            x += 1;
        }
    }
    let mut hyperminimum = (minimum.0, minimum.1, 0, 0);
    let mut hypermaximum = (maximum.0, maximum.1, 0, 0);

    for _turn in 1..=6 {
        let mut cubes_after:HashSet<(i32,i32,i32)> = HashSet::new();
        for x in minimum.0 - 1..maximum.0 + 2 {
            for y in minimum.1 - 1..maximum.1 + 2 {
                for z in minimum.2 - 1..maximum.2 + 2 {
                    let cube = (x, y, z);
                    let neighbors = count_neighbors(cube, &cubes);
                    if cubes.contains(&cube) && (2..=3).contains(&neighbors) ||
                       !cubes.contains(&cube) && neighbors == 3 {
                        cubes_after.insert(cube);
                        minimum.0 = min(minimum.0, x);
                        minimum.1 = min(minimum.1, y);
                        minimum.2 = min(minimum.2, z);
                        maximum.0 = max(maximum.0, x);
                        maximum.1 = max(maximum.1, y);
                        maximum.2 = max(maximum.2, z);
                    }
                }
            }
        }
        cubes = cubes_after;
    }
    println!("Part 1: {}", cubes.len());
    println!("Part 2: {}", part2(&mut hypercubes, &mut hyperminimum, &mut hypermaximum));
}

fn count_neighbors(cube: (i32, i32, i32), cubes: &HashSet<(i32, i32, i32)>) -> i32 {
    let mut neighbors = 0;
    for x in cube.0 - 1..=cube.0 + 1 {
        for y in cube.1 - 1..=cube.1 + 1 {
            for z in cube.2 - 1..=cube.2 + 1 {
                if cubes.contains(&(x, y, z)) {
                    neighbors += 1;
                }
            }
        }
    }
    if cubes.contains(&cube) {
        neighbors - 1
    } else {
        neighbors
    }
}

fn part2(hypercubes: &mut HashSet<(i32, i32, i32, i32)>,  minimum: &mut (i32, i32, i32, i32),
         maximum: &mut (i32, i32, i32, i32)) -> usize {
    for _turn in 1..=6 {
        let mut cubes_after:HashSet<(i32, i32, i32, i32)> = HashSet::new();
        for x in minimum.0 - 1..maximum.0 + 2 {
            for y in minimum.1 - 1..maximum.1 + 2 {
                for z in minimum.2 - 1..maximum.2 + 2 {
                    for w in minimum.3 - 1..maximum.3 + 2 {
                        let cube = (x, y, z, w);
                        let neighbors = count_hyperneighbors(cube, &hypercubes);
                        if hypercubes.contains(&cube) && (2..=3).contains(&neighbors) ||
                            !hypercubes.contains(&cube) && neighbors == 3 {
                            cubes_after.insert(cube);
                            minimum.0 = min(minimum.0, x);
                            minimum.1 = min(minimum.1, y);
                            minimum.2 = min(minimum.2, z);
                            minimum.3 = min(minimum.3, z);
                            maximum.0 = max(maximum.0, x);
                            maximum.1 = max(maximum.1, y);
                            maximum.2 = max(maximum.2, z);
                            maximum.3 = max(maximum.3, z);
                        }
                    }
                }
            }
        }
        *hypercubes = cubes_after;
    }
    hypercubes.len()
}

fn count_hyperneighbors(cube: (i32, i32, i32, i32), hypercubes: &HashSet<(i32, i32, i32, i32)>) -> i32 {
    let mut neighbors = 0;
    for x in cube.0 - 1..=cube.0 + 1 {
        for y in cube.1 - 1..=cube.1 + 1 {
            for z in cube.2 - 1..=cube.2 + 1 {
                for w in cube.3 - 1..=cube.3 + 1 {
                    if hypercubes.contains(&(x, y, z, w)) {
                        neighbors += 1;
                    }
                }
            }
        }
    }
    if hypercubes.contains(&cube) {
        neighbors - 1
    } else {
        neighbors
    }
}
