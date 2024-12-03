use regex::Regex;

pub fn p1(input: &str) -> u32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    re.captures_iter(input)
        .map(|c| {
            let op1 = c[1].parse::<u32>().unwrap();
            let op2 = c[2].parse::<u32>().unwrap();
            op1 * op2
        })
        .sum()
}

pub fn p2(input: &str) -> u32 {
    let mut active = true;
    let re = Regex::new(r"do\(\)|don't\(\)|mul\(([0-9]+),([0-9]+)\)").unwrap();
    re.captures_iter(&input)
        .map(|c| {
            dbg!(&c);
            match &c[0] {
                "do()" => active = true,
                "don't()" => active = false,
                _ => {
                    if active {
                        let op1 = c[1].parse::<u32>().unwrap();
                        let op2 = c[2].parse::<u32>().unwrap();
                        return op1 * op2;
                    }
                }
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_p1() {
        let sample = r#"
            xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
        "#;
        assert_eq!(p1(sample), 161);
    }

    #[test]
    fn sample_p2() {
        let sample = r#"
            xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
        "#;
        assert_eq!(p2(sample), 48);
    }
}
