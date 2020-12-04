use std::fs;
use std::collections::HashMap;
use regex::Regex;

pub fn day04() {
    let contents = fs::read_to_string("Input/Day04.txt")
        .expect("Couldn't read the file");

    let mut part1 = 0;
    let mut part2 = 0;
    let mut passports = Vec::new();
    let mut current = HashMap::new();
    for line in contents.lines() {
        if line.len() < 3 {
            passports.push(current.clone());
            current = HashMap::new();
            continue;
        }
        let words: Vec<&str> = line.split(' ').collect();
        for word in words {
            let parts: Vec<&str> = word.split(':').collect();
            current.insert(parts[0], parts[1]);
        }
    }
    passports.push(current);

    let color_regex = Regex::new("#[0-9a-f]{6}").unwrap();
    let colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    for passport in passports {
        if passport.values().count() == 8 ||
            (passport.values().count() == 7 && !passport.contains_key("cid")) {
            part1 += 1;
        }
        if part_2_valid(&passport, &color_regex, &colors) {
            part2 += 1;
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn part_2_valid(passport: &HashMap<&str, &str>, color_regex: &Regex, colors: &Vec<&str>) -> bool {

    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    if !passport.contains_key("byr") { return false }
    let byr = passport["byr"].parse::<i32>();
    match byr {
        Ok(ok) => { if ok < 1920 || ok > 2002 { return false }},
        Err(_e) => {return false},
    }

    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    if !passport.contains_key("iyr") { return false }
    let iyr = passport["iyr"].parse::<i32>();
    match iyr {
        Ok(ok) => { if ok < 2010 || ok > 2020 { return false }},
        Err(_e) => {return false},
    }

    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    if !passport.contains_key("eyr") { return false }
    let eyr = passport["eyr"].parse::<i32>();
    match eyr {
        Ok(ok) => { if ok < 2020 || ok > 2030 { return false }},
        Err(_e) => {return false},
    }

    // hgt (Height) - a number followed by either cm or in:
    //     If cm, the number must be at least 150 and at most 193.
    //     If in, the number must be at least 59 and at most 76.
    if !passport.contains_key("hgt") { return false }
    let hgt = passport["hgt"];
    if hgt.ends_with("cm") {
        let h = hgt.strip_suffix("cm").unwrap().parse::<i32>();
        match h {
            Ok(ok) => { if ok < 150 || ok > 193 { return false }},
            Err(_e) => {return false},
        }
    } else if hgt.ends_with("in") {
        let h = hgt.strip_suffix("in").unwrap().parse::<i32>();
        match h {
            Ok(ok) => { if ok < 59 || ok > 76 { return false }},
            Err(_e) => {return false},
        }
    } else {
        return false
    }

    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    if !passport.contains_key("hcl") { return false }
    let hcl = passport["hcl"];
    if !color_regex.is_match(hcl) { return false; }

    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    if !passport.contains_key("ecl") { return false }
    let ecl = passport["ecl"];
    if !colors.contains(&ecl) { return false; }

    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    if !passport.contains_key("pid") { return false }
    let pid = passport["pid"].parse::<i32>();
    match pid {
        Ok(_ok) => { if passport["pid"].chars().count() != 9 { return false }},
        Err(_e) => {return false},
    }

    // cid (Country ID) - ignored, missing or not.
    true
}