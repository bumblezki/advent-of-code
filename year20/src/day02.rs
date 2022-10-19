#![deny(clippy::pedantic)]

use std::str::FromStr;
use std::convert::Infallible;
use regex::Regex;
use counter::Counter;

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
        let re = Regex::new(r"(\d+)-(\d+) ([a-z]): (\w+)").unwrap();
        re.captures(s)
            .map(|cap| {
                let d1: usize = FromStr::from_str(&cap[1]).unwrap();
                let d2: usize = FromStr::from_str(&cap[2]).unwrap();
                let required_char: char = FromStr::from_str(&cap[3]).unwrap();
                let pw: String = FromStr::from_str(&cap[4]).unwrap();
                Ok(
                    PasswordChecker::new(
                        pw,
                        required_char,
                        d1,
                        d2,
                    )
                )
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
            }
        }
    }

    fn validate_occurance_count(&self) -> bool {
        let pw_chars_counter = &self.pw.chars().collect::<Counter<char>>();
        self.policy.d1 <= pw_chars_counter[&self.policy.required_char] && pw_chars_counter[&self.policy.required_char] <= self.policy.d2 
    }

    fn validate_occurance_positions(&self) -> bool {
        let pw_chars: Vec<char> = self.pw.chars().collect();
        (pw_chars[self.policy.d1-1] == self.policy.required_char && pw_chars[self.policy.d2-1] != self.policy.required_char) ||
        (pw_chars[self.policy.d1-1] != self.policy.required_char && pw_chars[self.policy.d2-1] == self.policy.required_char)
    }
}



pub fn day02(input_lines: &[Vec<String>]) -> (String, String) {
    let (answer1, answer2) = input_lines[0].iter().fold((0, 0), |(mut acc1, mut acc2), line| {
        let checker: PasswordChecker = line.parse().unwrap();
        if checker.validate_occurance_count() {
            acc1 += 1;
        };
        if checker.validate_occurance_positions() {
            acc2 += 1;
        }
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
"", // INPUT STRING
"0", // PART 1 RESULT
"0" // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(day02(&input_lines), (part1_result.to_string(), part2_result.to_string()));
    }
}