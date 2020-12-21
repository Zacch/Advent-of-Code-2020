use std::fs;
use std::collections::{HashSet, HashMap, BTreeSet};
use std::iter::FromIterator;

pub fn day21() {
    let contents = fs::read_to_string("Input/Day21.txt").expect("Couldn't read the file");

    let mut allergen_map:HashMap<&str, Vec<HashSet<&str>>> = HashMap::new();
    let mut all_ingredients: HashSet<&str> = HashSet::new();
    let mut ingredient_count: HashMap<&str, i32> = HashMap::new();
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(" (contains ").collect();
        let ingredients: HashSet<&str> = parts[0].split(" ").collect();
        all_ingredients = all_ingredients.union(&ingredients).cloned().collect();
        for ingredient in &ingredients {
            *ingredient_count.entry(ingredient).or_insert(0) += 1;
        }

        let allergens: Vec<&str> = parts[1].split(" ").into_iter().map(|a|&a[0..a.len()-1]).collect();
        for allergen in &allergens {
            (*allergen_map.entry(allergen).or_insert(vec![])).push(ingredients.to_owned());
        }
    }

    let mut suspect_ingredients:Vec<&str> = vec![];
    let mut suspicions:HashMap<&str, Vec<&str>> = HashMap::new();
    for (allergen, ingredient_lists) in allergen_map.iter() {
        if ingredient_lists.len() == 1 {
            let mut suspects =
                 Vec::from_iter(ingredient_lists[0].iter().to_owned().map(|a| *a));
            suspect_ingredients.extend(suspects.iter());
            (*suspicions.entry(allergen).or_insert(vec![])).append(&mut suspects);
        } else {
            let mut suspects = ingredient_lists[0].to_owned();
            for ingredient_list in &ingredient_lists[1..ingredient_lists.len()] {
                suspects = suspects.intersection(&ingredient_list).map(|a|*a).collect();
            }
            let suspects_vector = &mut Vec::from_iter(suspects);
            suspect_ingredients.extend(suspects_vector.iter());
            (*suspicions.entry(allergen).or_insert(vec![])).append(suspects_vector);
        }
    }

    let suspect_set = HashSet::from_iter(suspect_ingredients.into_iter());
    let safe_ingredients = all_ingredients.difference(&suspect_set);
    let mut part1 = 0;
    for ingredient in safe_ingredients {
        part1 += ingredient_count[ingredient];
    }
    println!("Part 1: {}", part1);

    let mut confirmed = BTreeSet::new();
    while !suspicions.is_empty() {
        for (allergen, ingredients) in suspicions.clone() {
            if ingredients.len() == 1 {
                let ingredient = ingredients[0];
                confirmed.insert(format!("{},{}", allergen, ingredient));
                suspicions.remove(allergen);
                for (allergen, ingredients) in suspicions.clone() {
                    let new_list:Vec<&str> = ingredients.into_iter()
                        .filter(|i|*i != ingredient).collect();
                    suspicions.remove(allergen);
                    suspicions.insert(allergen, new_list);
                }
            }
        }
    }

    print!{"Part 2: "};
    let confirmed_vector = &mut Vec::from_iter(&confirmed);
    for i in 0..confirmed_vector.len() - 1 {
        print!("{},", confirmed_vector[i].split(',').collect::<Vec<&str>>()[1]);
    }
    println!("{}", confirmed_vector[confirmed_vector.len()-1].split(',').collect::<Vec<&str>>()[1]);
}
