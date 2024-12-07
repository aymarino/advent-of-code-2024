fn match_target(target: u64, operands: &[u64], accum: u64, allow_concat: bool) -> bool {
    if operands.is_empty() {
        return accum == target;
    }
    if accum > target {
        return false;
    }

    let next = operands.first().unwrap();
    let rest = &operands[1..];
    if match_target(target, rest, accum + next, allow_concat) {
        return true;
    }

    if match_target(target, rest, accum * next, allow_concat) {
        return true;
    }

    if allow_concat {
        let next_num_digits = next.checked_ilog10().unwrap_or(0) + 1;
        let concat = accum * 10u64.pow(next_num_digits) + next;
        if match_target(target, rest, concat, allow_concat) {
            return true;
        }
    }

    false
}

fn get_equations(input: &str) -> impl Iterator<Item = (u64, Vec<u64>)> + '_ {
    input.trim().lines().map(|line| {
        let (target, operands) = line.trim().split_once(": ").unwrap();
        let target = target.parse::<u64>().unwrap();
        let operands: Vec<_> = operands
            .split(" ")
            .map(|operand| operand.parse::<u64>().unwrap())
            .collect();
        (target, operands)
    })
}

pub fn p1(input: &str) -> u64 {
    get_equations(input)
        .filter(|(target, operands)| {
            match_target(*target, &operands[1..], *operands.first().unwrap(), false)
        })
        .map(|(target, _)| target)
        .sum()
}

pub fn p2(input: &str) -> u64 {
    get_equations(input)
        .filter(|(target, operands)| {
            match_target(*target, &operands[1..], *operands.first().unwrap(), true)
        })
        .map(|(target, _)| target)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static SAMPLE: &str = r#"
    190: 10 19
    3267: 81 40 27
    83: 17 5
    156: 15 6
    7290: 6 8 6 15
    161011: 16 10 13
    192: 17 8 14
    21037: 9 7 18 13
    292: 11 6 16 20
    "#;

    #[test]
    fn sample_p1() {
        assert_eq!(p1(SAMPLE), 3749);
    }

    #[test]
    fn sample_p2() {
        assert_eq!(p2(SAMPLE), 11387);
    }
}
