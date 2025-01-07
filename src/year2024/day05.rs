use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

type Input = (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>);

pub fn parse(input: &str) -> Input {
    let mut parts = input.split("\n\n");

    let mut rules_map = HashMap::new();
    for line in parts.next().unwrap().lines() {
        let mut line_parts = line.split('|').map(|n| n.parse::<u32>().unwrap());
        rules_map
            .entry(line_parts.next().unwrap())
            .or_insert_with(HashSet::new)
            .insert(line_parts.next().unwrap());
    }

    let updates: Vec<Vec<u32>> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(',').map(|n| n.parse::<u32>().unwrap()).collect())
        .collect();

    (rules_map, updates)
}

pub fn part1((rules, updates): &Input) -> u32 {
    updates
        .iter()
        .filter_map(|update| {
            if update.is_sorted_by(|a, b| match rules.get(a) {
                None => false,
                Some(h) => h.contains(b),
            }) {
                Some(update[update.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

pub fn part2((rules, updates): &Input) -> u32 {
    updates
        .iter()
        .filter_map(|update| {
            let mut update_copy = update.clone();
            update_copy.sort_by(|a, b| match rules.get(a) {
                None => Ordering::Greater,
                Some(h) => {
                    if h.contains(b) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
            });
            if &update_copy != update {
                Some(update_copy[update_copy.len() / 2])
            } else {
                None
            }
        })
        .sum()
}
