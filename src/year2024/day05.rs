use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

fn parse(input: &str) -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
    let mut parts = input.split("\n\n");

    let mut rules_map = HashMap::new();
    for line in parts.next().unwrap().lines() {
        let mut line_parts = line.split("|").map(|n| n.parse::<u32>().unwrap());
        rules_map
            .entry(line_parts.next().unwrap())
            .or_insert_with(HashSet::new)
            .insert(line_parts.next().unwrap());
    }

    let updates: Vec<Vec<u32>> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(",").map(|n| n.parse::<u32>().unwrap()).collect())
        .collect();

    (rules_map, updates)
}

pub fn part1(input: &str) -> u32 {
    let (rules, updates) = parse(input);
    updates
        .iter()
        .map(|update| {
            if update.is_sorted_by(|a, b| rules.get(a).unwrap().contains(b)) {
                update[update.len() / 2]
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let (rules, updates) = parse(input);
    updates
        .iter()
        .map(|update| {
            let mut update_copy = update.clone();
            update_copy.sort_by(|a, b| {
                if rules.get(a).unwrap().contains(b) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            if &update_copy != update {
                update_copy[update_copy.len() / 2]
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use super::*;

    const EXAMPLE: &str = indoc! {"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    "};

    #[test]
    fn part1_test() {
        assert_eq!(part1(EXAMPLE), 143);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(EXAMPLE), 123);
    }
}
