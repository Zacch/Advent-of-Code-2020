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

    let mut part1 = 0;
    for message in &messages {
        match matches_rule(*message, &rules[&0], &rules, 0) {
            None => {}
            Some(lengths) => {
                if lengths.contains(&message.len()) {
                    part1 += 1;
                }
            }
        }
    }
    println!("Part 1: {}", part1);

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
            Some(lengths) => {
                if lengths.contains(&message.len()) {
                    part2 += 1;
                }
            }
        }
    }
    println!("Part 2: {}", part2);
}

fn make_index_rule(nr: i32, i0: i32, i1: i32, i2: i32) -> Rule {
    if i1 < 0 {
        Rule { nr, kind: Single, c: ' ', rule_indexes: (i0, i1, i2), rules: None }
    } else if i2 < 0 {
        Rule { nr, kind: Double, c: ' ', rule_indexes: (i0, i1, i2), rules: None }
    } else {
        Rule { nr, kind: Triple, c: ' ', rule_indexes: (i0, i1, i2), rules: None }
    }
}

fn matches_rule(message: &str, rule: &Rule, rules: &HashMap<i32, Rule>, level: i32) -> Option<Vec<usize>> {
    if message.is_empty() { return None; }
    return match rule.kind {
        Char => {
            if message.len() > 0 && message.chars().next().unwrap() == rule.c {
                Some(vec![1])
            } else {
                None
            }
        }
        Single => { matches_rule(message, &rules[&rule.rule_indexes.0], rules, level + 1) }
        Double => {
            match matches_rule(message, &rules[&rule.rule_indexes.0], rules, level + 1) {
                None => { None }
                Some(indices) => {
                    let mut result = vec![];
                    for index in indices {
                        match matches_rule(&message[index..], &rules[&rule.rule_indexes.1], rules, level + 1) {
                            None => {}
                            Some(indices2) => {
                                for index2 in indices2 {
                                    result.push(index + index2);
                                }
                            }
                        }
                    }
                    if result.is_empty() {
                        None
                    } else {
                        Some(result)
                    }
                }
            }
        }
        Triple => {
            match matches_rule(message, &rules[&rule.rule_indexes.0], rules, level + 1) {
                None => { None }
                Some(indices) => {
                    let mut result = vec![];
                    for index in indices {
                        match matches_rule(&message[index..], &rules[&rule.rule_indexes.1], rules, level + 1) {
                            None => {}
                            Some(indices2) => {
                                for index2 in indices2 {
                                    match matches_rule(&message[index + index2..], &rules[&rule.rule_indexes.2], rules, level + 1) {
                                        None => {}
                                        Some(indices3) => {
                                            for index3 in indices3 {
                                                result.push(index + index2 + index3);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if result.is_empty() {
                        None
                    } else {
                        Some(result)
                    }
                }
            }
        }
        TwoRules => {
            let mut result = vec![];
            let rule_pair = rule.rules.unwrap();
            let rule1 = make_index_rule(rule.nr * 1000 + 1,rule_pair.0.0,rule_pair.0.1,rule_pair.0.2);
            match matches_rule(message, &rule1, rules, level + 1) {
                Some(indices) => {
                    result.append(&mut indices.clone());
                }
                None => {}
            }
            let rule2 = make_index_rule(rule.nr * 1000 + 2, rule_pair.1.0,rule_pair.1.1,rule_pair.1.2);
            match matches_rule(message, &rule2, rules, level + 1) {
                Some(indices) => {
                    result.append(&mut indices.clone());
                }
                None => {}
            }
            if result.is_empty() {
                None
            } else {
                Some(result)
            }
        }
    }
}