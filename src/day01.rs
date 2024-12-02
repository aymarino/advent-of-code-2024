use std::collections::HashMap;

use itertools::Itertools;

fn get_inputs(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .filter_map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect_tuple()
        })
        .unzip()
}

pub fn p1(input: &str) -> u32 {
    let (mut left, mut right) = get_inputs(input);
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

pub fn p2(input: &str) -> u32 {
    let (left, right) = get_inputs(input);
    let mut right_counts = HashMap::new();
    right
        .into_iter()
        .for_each(|n| *right_counts.entry(n).or_insert(0) += 1);
    left.into_iter()
        .map(|n| n * right_counts.get(&n).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE: &str = r#"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "#;

    #[test]
    fn sample_p1() {
        assert_eq!(p1(SAMPLE), 11);
    }

    #[test]
    fn sample_p2() {
        assert_eq!(p2(SAMPLE), 31);
    }
}
