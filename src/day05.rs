use std::cmp::Ordering;
use std::collections::HashMap;

fn get_input(input: &str) -> (HashMap<u32, Vec<u32>>, impl Iterator<Item = Vec<u32>> + '_) {
    let (pairs, updates) = input.trim().split_once("\n\n").unwrap();
    let mut rules = HashMap::new();
    pairs
        .lines()
        .map(|line| line.trim().split_once("|").unwrap())
        .map(|(p1, p2)| (p1.parse::<u32>().unwrap(), p2.parse::<u32>().unwrap()))
        .for_each(|(p1, p2)| {
            rules.entry(p1).or_insert(Vec::new()).push(p2);
        });
    let updates = updates.lines().map(|line| {
        line.trim()
            .split(",")
            .map(|page| page.parse::<u32>().unwrap())
            .collect()
    });
    (rules, updates)
}

fn is_ordered(rules: &HashMap<u32, Vec<u32>>, p1: &u32, p2: &u32) -> bool {
    let mut ok = rules.get(p1).map_or(true, |v| v.contains(p2));
    ok &= rules.get(p2).map_or(true, |v| !v.contains(p1));
    ok
}

pub fn p1(input: &str) -> u32 {
    let (rules, updates) = get_input(input);
    updates
        .filter(|update| update.is_sorted_by(|p1, p2| is_ordered(&rules, p1, p2)))
        .map(|update| update[update.len() / 2])
        .sum()
}

pub fn p2(input: &str) -> u32 {
    let (rules, updates) = get_input(input);
    updates
        .filter(|update| !update.is_sorted_by(|p1, p2| is_ordered(&rules, p1, p2)))
        .map(|mut update| {
            update.sort_by(|p1, p2| {
                if is_ordered(&rules, p1, p2) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            update[update.len() / 2]
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE: &str = r#"
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
    "#;

    #[test]
    fn sample_p1() {
        assert_eq!(p1(SAMPLE), 143);
    }

    #[test]
    fn sample_p2() {
        assert_eq!(p2(SAMPLE), 123);
    }
}
