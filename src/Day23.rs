use std::time::Instant;
use std::cmp::min;

const ONE_MILLION: usize = 1_000_000;

pub fn day23() {
    let input = "394618527";

    let mut cups: Vec<i32> = input.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
    let cup_count = cups.len() as i32;
    let mut current_index = 0;

    for _ in 1..=100 {
        let current_label = cups[current_index];
        let mut picked_up = vec![];
        let mut pickup_index = (current_index + 1).rem_euclid(cups.len());
        for _ in 0..3 {
            if pickup_index == cups.len() { pickup_index = 0; }
            picked_up.push(cups.remove(pickup_index))
        }
        let mut destination_label = (current_label - 2).rem_euclid(cup_count) + 1;
        while !cups.contains(&destination_label) {
            destination_label = (destination_label - 2).rem_euclid(cup_count) + 1;
        }

        let destination_index = index_of(destination_label, &cups);
        for i in (0..3).rev() {
            cups.insert(destination_index + 1, picked_up.remove(i));
        }

        current_index = (index_of(current_label, &cups) + 1).rem_euclid(cups.len());
    }

    let one_index = index_of(1, &cups);
    let mut i = (one_index + 1).rem_euclid(cups.len());
    let mut part1 = String::new();
    while i != one_index {
        part1 += &cups[i].to_string();
        i = (i + 1).rem_euclid(cups.len());
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2(input));
    println!("Control: {}", part2("389125467"));
    println!("(Should be 934001 * 159792 = 149245887792)");
}


fn part2(input: &str) -> i128 {
    let mut cups = [0; ONE_MILLION];
    let mut cups2 = [0; ONE_MILLION];
    let mut i = 0;
    for char in input.chars() {
        cups[i] = char.to_digit(10).unwrap() as i32;
        i += 1;
    }
    while i < ONE_MILLION {
        cups[i] = (i + 1) as i32;
        i += 1;
    }

    let mut current_index = 2;
    let mut current_label = cups[current_index];
    let mut now = Instant::now();

    for turn in 1..=ONE_MILLION * 5 {
        if turn % 100000 == 0 {
            println!("turn:   {:?}", turn);
            println!("Execution time: {:?}", Instant::now().checked_duration_since(now).unwrap());
            now = Instant::now();
        }
        make_move(&cups, &mut cups2, current_index);
//        println!("result:   {:?}", &cups2[0..20]);

        current_index = (index_of2(current_label, &cups2) + 1).rem_euclid(ONE_MILLION);
        current_label = cups2[current_index];
        make_move(&cups2, &mut cups, current_index);
//        println!("result 2: {:?}", &cups[0..20]);
        current_index = (index_of2(current_label, &cups) + 1).rem_euclid(ONE_MILLION);
        current_label = cups[current_index];
    }
    let cup_1_index = index_of2(1, &cups);
    println!("result:   {:?}", &cups);
    let cup1 = cups[(cup_1_index + 1).rem_euclid(ONE_MILLION)] as i128;
    let cup2 = cups[(cup_1_index + 2).rem_euclid(ONE_MILLION)] as i128;
    println!("Part 2: {} * {} == {}", cup1, cup2, cup1 * cup2);
    cup1 * cup2
}

fn make_move(source: &[i32; ONE_MILLION], result: &mut [i32; ONE_MILLION], current_index: usize) {
    let current_label: i32 = source[current_index];
//    println!("Current cup {}", current_label);

    let mut picked_up:[i32; 3] = [0; 3];
    let mut pickup_index = current_index + 1;
    for i in 0..3 {
        if pickup_index == ONE_MILLION { pickup_index = 0; }
        picked_up[i] = source[pickup_index];
        pickup_index = pickup_index + 1;
    }

    let mut destination_label = (current_label - 2).rem_euclid(ONE_MILLION as i32) + 1;
    while picked_up.contains(&destination_label) {
        destination_label = (destination_label - 2).rem_euclid(ONE_MILLION as i32) + 1;
    }
    let destination_index = index_of2(destination_label, &source);
    // println!("source:      {:?} at index {}", current_label, current_index);
    // println!("destination: {:?} at index {}", destination_label, destination_index);

    if current_index < destination_index {
        // ....1xxx,,,,2____
        // ....1,,,,2xxx____
        result[..=current_index].copy_from_slice(&source[..=current_index]);
        result[current_index + 1..=destination_index - 3].copy_from_slice(&source[current_index + 4..=destination_index]);
        result[destination_index - 2..=destination_index].copy_from_slice(&source[current_index + 1..current_index + 4]);
        result[destination_index + 1..].copy_from_slice(&source[destination_index + 1..]);
    } else {
        // ....2,,,,1xxx____
        // ....2xxx,,,,1____
        if current_index >= ONE_MILLION - 4 {
            let overlap = (current_index + 4).rem_euclid(ONE_MILLION);
            result[..=destination_index - overlap].copy_from_slice(&source[overlap..=destination_index]);

            let mut source_index = current_index + 1;
            for i in 0..3 {
                if source_index == ONE_MILLION { source_index = 0; }
                result[destination_index - overlap + 1 + i] = source[source_index];
                source_index += 1
            }
            result[destination_index - overlap + 4..ONE_MILLION].copy_from_slice(&source[destination_index + 1..=current_index]);
        } else {
            result[..=destination_index].copy_from_slice(&source[..=destination_index]);
            result[destination_index + 1..destination_index + 4].copy_from_slice(&source[current_index + 1..current_index + 4]);
            result[destination_index + 4..=current_index + 3].copy_from_slice(&source[destination_index + 1..=current_index]);
            result[current_index + 4..].copy_from_slice(&source[current_index + 4..]);
        }
    }
}




fn index_of(label: i32, cups: &Vec<i32>) -> usize {
    for i in 0..cups.len() {
        if cups[i] == label {
            return i;
        }
    }
    panic!("Couldn't find {} in {:?}", label, cups)
}

fn index_of2(label: i32, cup_array: &[i32; ONE_MILLION]) -> usize {
    for i in 0..ONE_MILLION {
        if cup_array[i] == label {
            return i;
        }
    }
    panic!("Couldn't find {} in {:?}", label, &cup_array[0..20])
}

// -- move 1000 --
// Execution time: 90.936737205s

// Execution time: 1.709578022s
// Part 2: 382911 * 455139 == -1815929507
// 1815929507 is too low
// 174277729629 is too low
// 470976377428 is too low
//(Should be 934001 * 159792 = 149245887792)
