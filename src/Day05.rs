use std::fs;

pub fn day05() {
    let contents = fs::read_to_string("Input/Day05.txt")
        .expect("Couldn't read the file");

    let mut part1 = 0;
    let mut part2 = 0;
    let mut seats: Vec<i32> = vec![];
    for line in contents.lines() {

        let binary:String = line.chars().map(|x| match x {
                'B' => '1', 'R' => '1', _ => '0'
            }).collect();
        let seat_id = i32::from_str_radix(&binary, 2).unwrap();
        part1 = i32::max(part1, seat_id);
        seats.push(seat_id.to_owned());
    }

    for i in 0..part1 {
        if !seats.contains(&i) && seats.contains(&(i-1)) && seats.contains(&(i+1)){
            part2 = i;
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}