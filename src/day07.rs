fn match_target(target: u64, operands: &mut Vec<u64>, allow_concat: bool) -> bool {
    assert!(!operands.is_empty());
    if operands.len() == 1 {
        return operands[0] == target;
    }
    if *operands.last().unwrap() > target {
        return false;
    }

    let a = operands.pop().unwrap();
    let b = operands.pop().unwrap();

    operands.push(a + b);
    if match_target(target, operands, allow_concat) {
        return true;
    }
    operands.pop();

    operands.push(a * b);
    if match_target(target, operands, allow_concat) {
        return true;
    }
    operands.pop();

    if allow_concat {
        let b_num_digits = b.checked_ilog10().unwrap_or(0) + 1;
        let concat = a * 10u64.pow(b_num_digits) + b;
        operands.push(concat);
        if match_target(target, operands, allow_concat) {
            return true;
        }
        operands.pop();
    }

    operands.push(b);
    operands.push(a);
    false
}

fn get_equations(input: &str) -> impl Iterator<Item = (u64, Vec<u64>)> + '_ {
    input.trim().lines().map(|line| {
        let (target, operands) = line.trim().split_once(": ").unwrap();
        let target = target.parse::<u64>().unwrap();
        let mut operands: Vec<_> = operands
            .split(" ")
            .map(|operand| operand.parse::<u64>().unwrap())
            .collect();
        operands.reverse();
        (target, operands)
    })
}

pub fn p1(input: &str) -> u64 {
    get_equations(input)
        .filter_map(|(target, mut operands)| {
            if match_target(target, &mut operands, false) {
                Some(target)
            } else {
                None
            }
        })
        .sum()
}

pub fn p2(input: &str) -> u64 {
    get_equations(input)
        .filter_map(|(target, mut operands)| {
            if match_target(target, &mut operands, true) {
                Some(target)
            } else {
                None
            }
        })
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
