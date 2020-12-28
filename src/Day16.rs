use std::fs;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, Eq)]
struct Field {
    name: String,
    range1: RangeInclusive<i32>,
    range2: RangeInclusive<i32>,
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Field {
    fn contains(&self, number: &i32) -> bool {
        self.range1.contains(number) || self.range2.contains(number)
    }
}

pub fn day16() {
    let contents = fs::read_to_string("Input/Day16.txt").expect("Couldn't read the file");
    let mut iterator = contents.lines().into_iter();
    let mut line = iterator.next().unwrap();
    let mut fields = vec![];
    while line.len() > 0 {
        let parts: Vec<&str> = line.split(':').collect();
        let ranges: Vec<&str> = parts[1].split(' ').collect();
        fields.push(Field {
            name: parts[0].parse().unwrap(),
            range1: make_range(ranges[1]),
            range2: make_range(ranges[3])
        });
        line = iterator.next().unwrap();
    }
    iterator.next();
    let my_ticket: Vec<i32> = iterator.next().unwrap().split(',')
        .map(|s|i32::from_str(s).unwrap()).collect();
    iterator.next();
    iterator.next();

    let mut valid_tickets = vec![];
    let mut part1 = 0;
    line = iterator.next().unwrap();
    while line.len() > 0 {
        let mut ticket_valid = true;
        let numbers: Vec<i32> = line.split(',')
            .map(|s|i32::from_str(s).unwrap()).collect();
        for number in &numbers {
            let mut number_valid = false;
            for field in &fields {
                if field.contains(number) {
                    number_valid = true;
                    break;
                }
            }
            if !number_valid {
                part1 += number;
                ticket_valid = false;
            }
        }
        if ticket_valid {
            valid_tickets.push(numbers);
        }
        line = match iterator.next() {
            None => {""}
            Some(s) => { s }
        }
    }
    println!("Part 1: {}", part1);

    let mut possible_fields:HashMap<&Field, HashSet<usize>> = HashMap::new();
    for field in &fields {
        possible_fields.insert(field, HashSet::new());
    }
    for index in 0..my_ticket.len() {
        for field in &fields {
            if field.contains(&my_ticket[index]) {
                possible_fields.get_mut(&field).unwrap().insert(index);
            }
        }
    }

    for ticket in &valid_tickets {
        for index in 0..ticket.len() {
            for field in &fields {
                if !field.contains(&ticket[index]) {
                    let indices = possible_fields.get_mut(&field).unwrap();
                    if indices.contains(&index) {
                        indices.remove(&index);
                    }
                }
            }
        }
    }

    let mut field_order:HashMap<&Field, usize> = HashMap::new();
    while !&possible_fields.is_empty() {
        let mut updated_fields:HashMap<&Field, HashSet<usize>> = HashMap::new();
        for (field, indices) in &possible_fields {
            if indices.len() == 1 {
                let index = indices.iter().next().unwrap();
                field_order.insert(field, index.to_owned());
                for (possible_field, indices) in &possible_fields {
                    let mut new_indices: HashSet<usize> = HashSet::new();
                    for possible_index in indices {
                        if *possible_index != *index {
                            new_indices.insert(*possible_index);
                        }
                    }
                    if new_indices.len() > 0 {
                        updated_fields.insert(possible_field, new_indices);
                    }
                }
                possible_fields = updated_fields;
                break;
            }
        }
    }

    let departure_fields:HashMap<&Field, usize> = field_order.into_iter()
        .filter(|(f,_)| f.name.starts_with("departure")).collect();
    let part2 = departure_fields.iter()
        .fold(1, |acc:i128, (_, index) | acc * (my_ticket[*index] as i128));
    println!("Part 2: {}", part2);
}

fn make_range(a_dash_b: &str) -> RangeInclusive<i32> {
    let numbers: Vec<&str> = a_dash_b.split('-').collect();
    RangeInclusive::new(i32::from_str(numbers[0]).unwrap(),i32::from_str(numbers[1]).unwrap())
}