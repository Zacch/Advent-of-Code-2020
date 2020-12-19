use std::fs;
use std::str::FromStr;
use std::collections::HashMap;
use std::iter::FromIterator;

pub fn day14() {
    let contents = fs::read_to_string("Input/Day14.txt").expect("Couldn't read the file");

    let mut and_mask:i64 = 0xfffffffff;
    let mut or_mask:i64 = 0;
    let mut memory: HashMap<i64, i64> = HashMap::new();
    for line in contents.lines() {
        let words: Vec<&str> = line.split(' ').collect();
        if words[0] == "mask" {
            let and_str = words[2].replace('X', "1");
            let or_str = words[2].replace('X', "0");
            and_mask = i64::from_str_radix(&*and_str, 2).unwrap();
            or_mask = i64::from_str_radix(&*or_str, 2).unwrap();
        } else {
            let address = i64::from_str(&(words[0])[4..words[0].len() - 1]).unwrap();
            let argument = i64::from_str(words[2]).unwrap();
            let value = (argument & and_mask) | or_mask;
            store(&mut memory, &address, value);
        }
    }
    let part1:i64 = memory.values().sum();

    memory.clear();
    let mut mask_bits: Vec<char> = vec![];
    let mut number_of_addresses = 0;
    for line in contents.lines() {
        let words: Vec<&str> = line.split(' ').collect();
        if words[0] == "mask" {
            let mask = words[2];
            mask_bits = mask.chars().collect();
            or_mask = i64::from_str_radix(&*mask.replace('X', "0"), 2).unwrap();
            number_of_addresses = i64::pow(2, mask.matches("X").count() as u32);
        } else {
            let base_address = i64::from_str(&(words[0])[4..words[0].len() - 1]).unwrap() | or_mask;
            let base_bits: Vec<char> = format!("{:036b}", base_address).chars().collect();
            let value = i64::from_str(words[2]).unwrap();
            for i in 0..number_of_addresses {
                let bits: Vec<char> = format!("{:036b}", i).chars().collect();
                let mut address_bits = base_bits.clone();
                let mut bits_index = 35;
                for pos in (0..36).rev() {
                    if mask_bits[pos] == 'X' {
                        address_bits[pos] = bits[bits_index];
                        bits_index -= 1;
                    }
                }
                let address = i64::from_str_radix(&String::from_iter(address_bits), 2).unwrap();
                store(&mut memory, &address, value)
            }
        }
    }
    let part2: i64 = memory.values().sum();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn store(memory: &mut HashMap<i64, i64>, address: &i64, value: i64) {
    if memory.contains_key(&address) {
        *memory.get_mut(&address).unwrap() = value;
    } else {
        memory.insert(*address, value);
    }
}