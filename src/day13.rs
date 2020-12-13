use std::fs;
use std::str::FromStr;

pub fn day13() {
    let contents = fs::read_to_string("Input/Day13.txt")
        .expect("Couldn't read the file");
    let lines: Vec<&str> = contents.split("\n").collect();

    let start_time = i128::from_str(lines[0]).unwrap();
    let bus_list: Vec<&str> = lines[1].split(',').collect();
    let mut buses: Vec<(i128, i128)> = vec![];
    for i in 0..bus_list.len() {
        if bus_list[i] != "x" {
            buses.push((i128::from_str(bus_list[i]).unwrap(), i as i128));
        }
    }

    let mut earliest_bus = 0;
    let mut shortest_wait = 9999999;
    for bus in &buses {
        let wait = bus.0 - (start_time % bus.0);
        if wait < shortest_wait {
            earliest_bus = bus.0;
            shortest_wait = wait;
        }
    }
    println!("Part 1: {}", earliest_bus * shortest_wait);

    let first_bus = buses.remove(0);
    let mut t = first_bus.0;
    let mut step = first_bus.0;
    for bus in &buses {
        for i in 0..bus.0 {
            if bus.0 - ((t + step * i) % bus.0) == (bus.1 % bus.0) {
                t += step * i;
                step *= bus.0;
                break;
            }
        }
    }
    println!("Part 2: {}", t);
}