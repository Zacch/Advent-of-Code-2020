use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::FromStr;

pub fn day07() {
    let contents = fs::read_to_string("Input/Day07.txt")
        .expect("Couldn't read the file");

    let mut children = HashMap::new();
    let mut parents = HashMap::new();

    for line in contents.lines() {
        let words: Vec<&str> = line.split(' ').collect();

        let parent = format!("{} {}", words[0], words[1]);
        let mut child_vector: Vec<(i32, String)> = vec![];
        let mut index = 4;
        while index + 3 < words.len() {
            let count = i32::from_str(words[index]).unwrap();
            let child =  format!("{} {}", words[index + 1], words[index + 2]);
            child_vector.push((count, child.to_owned()));
            if !parents.contains_key(&child) {
                parents.insert( child.to_owned(), vec![]);
            }
            parents.get_mut(&child).unwrap().push(parent.to_owned());
            index += 4;
        }
        children.insert(parent, child_vector);
    }

    let mut frontier = vec!["shiny gold".to_owned()];
    let mut reached = HashSet::new();
    reached.insert("shiny gold".to_owned());
    while !frontier.is_empty() {
        let bag = frontier.pop().unwrap();
        if !parents.contains_key(&bag) {
            continue;
        }
        let bag_parents = parents.get_mut(&bag).unwrap();
        for parent in bag_parents {
            if !reached.contains(parent) {
                frontier.push(parent.to_owned());
                reached.insert(parent.to_owned());
            }
        }
    }
    let part1 = reached.len() - 1;

    let mut part2 = 0;
    let mut frontier = vec![(1, "shiny gold".to_owned())];
    while !frontier.is_empty() {
        let tuple = frontier.pop().unwrap();
        let multiplier = tuple.0;
        let bag = tuple.1;

        let bag_children = children.get_mut(&bag).unwrap();
        for  child in bag_children {
            part2 += multiplier * child.0;
            frontier.push((child.0 * multiplier, child.1.to_owned()));
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}