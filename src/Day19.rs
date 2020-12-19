use std::fs;
use std::collections::HashMap;
use std::str::FromStr;
use crate::day19::RuleType::{Char, Single, Double, TwoSingles, TwoDoubles, Triple};

#[derive(Debug, Copy, Clone)]
enum RuleType { Char, Single, Double, Triple, TwoSingles, TwoDoubles }

#[derive(Debug, Copy, Clone)]
struct Rule {
    kind: RuleType,
    c: char,
    first: (i32, i32),
    second: (i32, i32)
}

pub fn day19() {
    let contents = fs::read_to_string("Input/Day19.txt").expect("Couldn't read the file");
    let mut part1 = 0;
    let  part2 = 0;

    let mut rules = HashMap::new();
    let mut messages = vec![];
    for line in contents.lines() {
        let words: Vec<&str> = line.split(' ').collect();
        if words.len() > 1 {
//          println!("{:?} {}", &words, words.len());
            let nr = i32::from_str(words[0].trim_end_matches(":")).unwrap();
            let rule;
            match words.len() {
                2 => {
                    match i32::from_str(words[1]) {
                        Ok(num) => {
                            rule = Rule {kind: Single, c: ' ', first: (num, -1), second: (-1, -1) };
                        }
                        Err(_) => {
                            rule = Rule {kind: Char, c: words[1].trim_start_matches("\"").chars().next().unwrap(),
                                first: (0, 0), second: (0, 0) };
                        }
                    }
                }
                3 => {
                    rule = Rule {kind: Double, c: ' ',
                        first: (i32::from_str(words[1]).unwrap(), i32::from_str(words[2]).unwrap()),
                        second: (-1, -1) };
                }
                4 => {
                    if words[2] == "|" {
                        rule = Rule {kind: TwoSingles, c: ' ',
                            first: (i32::from_str(words[1]).unwrap(), -1),
                            second: (i32::from_str(words[3]).unwrap(), -1) };
                    } else {
                        rule = Rule {kind: Triple, c: ' ',
                            first: (i32::from_str(words[1]).unwrap(), i32::from_str(words[2]).unwrap()),
                            second: (i32::from_str(words[3]).unwrap(), -1) };
                    }
                }
                6 => {
                    rule = Rule {kind: TwoDoubles, c: ' ',
                        first: (i32::from_str(words[1]).unwrap(), i32::from_str(words[2]).unwrap()),
                        second: (i32::from_str(words[4]).unwrap(), i32::from_str(words[5]).unwrap()) };
                }
                _ => panic!("unrecognized rule {}", line)
            }
            rules.insert(nr, rule);
 //           println!("Rule {} {:?}", nr, rule);
        } else {
            messages.push(words[0]);
        }
    }
//    println!("{:?}", rules);

    for message in messages {
        match matches_rule(message, &rules[&0], &rules) {
            None => {}
            Some(length) => {
                if length == message.len() {
                    part1 += 1;
                }
            }
        }
    }
    //8: 42 | 42 8
    // 11: 42 31 | 42 11 31
    //rules[8] = Rule { kind: RuleType::TwoDoubles, c: ' ', first: (42, -1), second: (42, 8) };
    //rules[11] = Rule { kind: RuleType::TwoDoubles, c: ' ', first: (42, 31), second: (42, 8) };

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
/*
fn matches(message: &str, rules: &HashMap<i32, Rule>) -> bool {
    if message.len() == 0 { return false; }
    for rule in rules.values() {
        match matches_rule(message, rule, rules) {
            None => {}
            Some(length) => {
                if length == message.len() {
                    return true;
                }
            }
        }
    }
    false
}
*/
fn matches_rule(message: &str, rule: &Rule, rules: &HashMap<i32, Rule>) -> Option<usize> {
//    println!("matches_rule({}, {:?}", message, rule);
    return match rule.kind {
        Char => {
            if message.len() > 0 && message.chars().next().unwrap() == rule.c {
//                println!("Message {} matches {:?}!", message, rule);
                Some(1)
            } else {
//                println!("Message {} doesn't match {:?}.", message, rule);
                None
            }
        }
        Single => { matches_rule(message, &rules[&rule.first.0], rules) }
        Double => {
//            println!("Message {} {:?}", message, rule);
            match matches_rule(message, &rules[&rule.first.0], rules) {
                None => { None }
                Some(index) => {
//                    println!("Message {} first {} chars matched rule {}", message, index, rule.first.0);
                    match matches_rule(&message[index..],
                                       &rules[&rule.first.1], rules) {
                        None => { None }
                        Some(end_index) => {
//                            println!("Message {} first {} chars matched rule {}", message, end_index, rule.first.1);
//                                println!("Message {} matched Double rule {:?}", message, rule);
                                Some(index + end_index)

                        }
                    }
                }
            }
        }
        Triple => {
            match matches_rule(message, &rules[&rule.first.0], rules) {
                None => { None }
                Some(index) => {
//                    println!("Message {} first {} chars matched rule {}", message, index, rule.first.0);
                    match matches_rule(&message[index..],
                                       &rules[&rule.first.1], rules) {
                        None => { None }
                        Some(index2) => {
                            let sum = index + index2;
//                            println!("Message {} {} more chars matched rule {}", message, index2, rule.first.1);
                            match matches_rule(&message[sum..],
                                               &rules[&rule.second.0], rules) {
                                None => { None }
                                Some(index3) => {
                                    let sum = sum + index3;
//                             println!("Message {} {} chars matched rule {}", message, index3, rule.first.1);
                                    if sum == message.len() {
//                                println!("Message {} matched Double rule {:?}", message, rule);
                                        Some(sum)
                                    } else {
//                                println!("Message {} didn't match. {} + {} != {}", message, index, end_index, message.len());
                                        None
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        TwoSingles => {
            match matches_rule(message, &rules[&rule.first.0], rules) {
                Some(index) => {
                        return Some(index)
                }
                None => {}
            }
            matches_rule(message, &rules[&rule.second.0], rules)
        }
        TwoDoubles => {
            let rule1 = Rule { kind: Double, c: ' ', first: rule.first, second: (-1, -1) };
            match matches_rule(message, &rule1, rules) {
                Some(index) => {
                    return Some(index)
                }
                None => {}
            }
            let rule2 = Rule { kind: Double, c: ' ', first: rule.second, second: (-1, -1) };
            matches_rule(message, &rule2, rules)
        }
    }
}
