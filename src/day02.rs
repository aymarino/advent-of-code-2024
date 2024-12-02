use itertools::Itertools;

fn get_reports(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect()
        })
        .filter(|v: &Vec<_>| !v.is_empty())
        .collect()
}

fn is_safe(report: &Vec<u32>) -> bool {
    let increasing = report.iter().is_sorted();
    let decreasing = report.iter().rev().is_sorted();
    let low_diff = report
        .iter()
        .tuple_windows()
        .all(|(a, b)| (1..=3).contains(&a.abs_diff(*b)));
    (increasing || decreasing) && low_diff
}

pub fn p1(input: &str) -> u32 {
    let reports = get_reports(input);
    reports.into_iter().filter(is_safe).count() as u32
}

pub fn p2(input: &str) -> u32 {
    let reports = get_reports(input);
    reports
        .into_iter()
        .filter(|report| {
            is_safe(report)
                || (0..report.len()).any(|i| {
                    let mut report = report.clone();
                    report.remove(i);
                    is_safe(&report)
                })
        })
        .count() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE: &str = r#"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
    "#;

    #[test]
    fn sample_p1() {
        assert_eq!(p1(SAMPLE), 2);
    }

    #[test]
    fn sample_p2() {
        assert_eq!(p2(SAMPLE), 4);
    }
}
