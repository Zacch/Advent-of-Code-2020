use std::fs;
use std::collections::HashMap;
use std::str::FromStr;
use crate::day19::RuleType::{Char, Single, Double, Triple, TwoRules};

#[derive(Debug, Copy, Clone)]
enum RuleType { Char, Single, Double, Triple, TwoRules }

#[derive(Debug, Copy, Clone)]
struct Rule {
    nr: i32,
    kind: RuleType,
    c: char,
    rule_indexes: (i32, i32, i32),
    rules: Option<((i32, i32, i32), (i32, i32, i32))>
}

pub fn day19() {
    let contents = fs::read_to_string("Input/Day19.txt").expect("Couldn't read the file");

    let mut rules = HashMap::new();
    let mut messages = vec![];
    for line in contents.lines() {
        let words: Vec<&str> = line.split(' ').collect();
        if words.len() > 1 {
            let nr = i32::from_str(words[0].trim_end_matches(":")).unwrap();
            let rule;
            match words.len() {
                2 => {
                    match i32::from_str(words[1]) {
                        Ok(num) => {
                            rule = make_index_rule(nr, num, -1, -1);
                        }
                        Err(_) => {
                            rule = Rule {nr, kind: Char, rule_indexes: (-1, -1, -1), rules: None,
                                c: words[1].trim_start_matches("\"").chars().next().unwrap()};
                        }
                    }
                }
                3 => { rule = make_index_rule(nr, i32::from_str(words[1]).unwrap(),
                                              i32::from_str(words[2]).unwrap(), -1); }
                4 => if words[2] == "|" {
                    rule = Rule {nr, kind: TwoRules, c: ' ', rule_indexes: (-1, -1, -1), rules:
                        Some(((i32::from_str(words[1]).unwrap(), -1, -1),
                              (i32::from_str(words[3]).unwrap(), -1, -1)))};
                } else {
                    rule = make_index_rule(nr, i32::from_str(words[1]).unwrap(),
                                           i32::from_str(words[2]).unwrap(),
                                           i32::from_str(words[3]).unwrap());
                },
                6 => {
                    rule = Rule {nr, kind: TwoRules, c: ' ', rule_indexes: (-1, -1, -1),
                        rules: Some(((i32::from_str(words[1]).unwrap(),
                                      i32::from_str(words[2]).unwrap(), -1),
                                     (i32::from_str(words[4]).unwrap(),
                                      i32::from_str(words[5]).unwrap(), -1)))};
                }
                _ => panic!("unrecognized rule {}", line)
            }
            rules.insert(nr, rule);
        } else {
            if line.trim().len() > 0 {
                messages.push(words[0]);
            }
        }
    }

    // let mut part1 = 0;
    // for message in &messages {
    //     match matches_rule(*message, &rules[&0], &rules, 0) {
    //         None => {}
    //         Some(length) => {
    //             if length == message.len() {
    //                 part1 += 1;
    //             }
    //         }
    //     }
    // }

    rules.remove(&8);
    rules.insert(8, Rule {nr: 8, kind: RuleType::TwoRules, c: ' ', rule_indexes: (-1, -1, -1),
                                rules: Some(((42, 8, -1), (42, -1, -1)))});
    rules.remove(&11);
    rules.insert(11, Rule {nr: 11, kind: RuleType::TwoRules, c: ' ', rule_indexes: (-1, -1, -1),
                                 rules: Some(((42, 11, 31), (42, 31, -1)))});

    let mut part2 = 0;
    for message in &messages {
        match matches_rule(message, &rules[&0], &rules, 0) {
            None => {}
            Some(length) => {
                if length == message.len() {
                    part2 += 1;
                }
            }
        }
    }
   // println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    // for i in 0..50 {
    //     if rules.contains_key(&i) { println!("Rule {}: {:?}", i, rules[&i])}
    // }
}
// 134 is too low
// 235 is too high

fn make_index_rule(nr: i32, i0: i32, i1: i32, i2: i32) -> Rule {
    if i1 < 0 {
        Rule { nr, kind: Single, c: ' ', rule_indexes: (i0, i1, i2), rules: None }
    } else if i2 < 0 {
        Rule { nr, kind: Double, c: ' ', rule_indexes: (i0, i1, i2), rules: None }
    } else {
        Rule { nr, kind: Triple, c: ' ', rule_indexes: (i0, i1, i2), rules: None }
    }
}

fn matches_rule(message: &str, rule: &Rule, rules: &HashMap<i32, Rule>, level: i32) -> Option<usize> {
    let indent = " ".repeat(level as usize);
    println!("{}matches_rule({}, {:?})", indent, message, rule);
    if message.is_empty() { return None; }
    return match rule.kind {
        Char => {
            if message.len() > 0 && message.chars().next().unwrap() == rule.c {
                println!("{}-- Yes", indent);
                Some(1)
            } else {
                println!("{}-- No", indent);
                None
            }
        }
        Single => { matches_rule(message, &rules[&rule.rule_indexes.0], rules, level + 1) }
        Double => {
            match matches_rule(message, &rules[&rule.rule_indexes.0], rules, level + 1) {
                None => { None }
                Some(index) => {
                    println!("{}Message {} first {} chars matched rule {}", indent, message, index, &rule.rule_indexes.0);
                    match matches_rule(&message[index..],
                                       &rules[&rule.rule_indexes.1], rules, level + 1) {
                        None => { None }
                        Some(end_index) => {
                            println!("{}-- Yes Message {} {} chars matched rule {}", indent, message, index + end_index, &rule.nr);
                                Some(index + end_index)
                        }
                    }
                }
            }
        }
        Triple => {
            match matches_rule(message, &rules[&rule.rule_indexes.0], rules, level + 1) {
                None => { None }
                Some(index) => {
                    println!("{}Message {} first {} chars matched rule {}", indent, message, index, rule.rule_indexes.0);
                    match matches_rule(&message[index..],
                                       &rules[&rule.rule_indexes.1], rules, level + 1) {
                        None => { None }
                        Some(index2) => {
                            let sum = index + index2;
                            println!("{}Message {} {} more chars matched rule {}", indent, message, index2, rule.rule_indexes.1);
                            match matches_rule(&message[sum..],
                                               &rules[&rule.rule_indexes.2], rules, level + 1) {
                                None => { None }
                                Some(index3) => {
                                    let sum = sum + index3;
                                    println!("{}Message {} {} chars matched rule {}", indent, message, index3, rule.rule_indexes.2);
                                        println!("{}-- Yes {} {} chars matched Triple rule {:?}", indent, message, sum, rule);
                                        Some(sum)
                                }
                            }
                        }
                    }
                }
            }
        }
        TwoRules => {
            let rule_pair = rule.rules.unwrap();
            let rule1 = make_index_rule(rule.nr * 1000 + 1,rule_pair.0.0,rule_pair.0.1,rule_pair.0.2);
            match matches_rule(message, &rule1, rules, level + 1) {
                Some(index) => {
                    return Some(index)
                }
                None => {}
            }
            let rule2 = make_index_rule(rule.nr * 1000 + 2, rule_pair.1.0,rule_pair.1.1,rule_pair.1.2);
            matches_rule(message, &rule2, rules, level + 1)
        }
    }
}
