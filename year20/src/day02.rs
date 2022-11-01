#![deny(clippy::pedantic)]

use counter::Counter;
use once_cell::sync::OnceCell;
use regex::Regex;
use std::convert::Infallible;
use std::str::FromStr;

struct PasswordPolicy {
    required_char: char,
    d1: usize,
    d2: usize,
}

struct PasswordChecker {
    pw: String,
    policy: PasswordPolicy,
}

impl FromStr for PasswordChecker {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RE: OnceCell<Regex> = OnceCell::new();

        RE.get_or_init(|| Regex::new(r"(\d+)-(\d+) ([a-z]): (\w+)").unwrap())
            .captures(s)
            .map(|cap| {
                Ok(PasswordChecker::new(
                    cap[4].parse().unwrap(),
                    cap[3].parse().unwrap(),
                    cap[1].parse().unwrap(),
                    cap[2].parse().unwrap(),
                ))
            })
            .expect("Failed to match regular expression against input.")
    }
}

impl PasswordChecker {
    fn new(pw: String, required_char: char, d1: usize, d2: usize) -> Self {
        Self {
            pw,
            policy: PasswordPolicy {
                required_char,
                d1,
                d2,
            },
        }
    }

    fn validate_occurrence_count(&self) -> bool {
        (self.policy.d1..=self.policy.d2)
            .contains(&self.pw.chars().collect::<Counter<char>>()[&self.policy.required_char])
    }

    fn validate_occurrence_positions(&self) -> bool {
        (self.pw.chars().nth(self.policy.d1 - 1).unwrap() == self.policy.required_char)
            ^ (self.pw.chars().nth(self.policy.d2 - 1).unwrap() == self.policy.required_char)
    }
}

pub fn day02(input_lines: &[Vec<String>]) -> (String, String) {
    let (answer1, answer2) = input_lines[0]
        .iter()
        .fold((0, 0), |(mut acc1, mut acc2), line| {
            let checker: PasswordChecker = line.parse().unwrap();
            if checker.validate_occurrence_count() {
                acc1 += 1;
            };
            if checker.validate_occurrence_positions() {
                acc2 += 1;
            };
            (acc1, acc2)
        });
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day02;
    use crate::utils::load_input;

    #[test]
    fn check_day02_case01() {
        full_test(
            "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc", // INPUT STRING
            "2", // PART 1 RESULT
            "1", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day02(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
