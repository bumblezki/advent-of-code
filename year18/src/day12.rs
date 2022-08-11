// Potential improvements:
//
use std::str::{FromStr, ParseBoolError};

#[derive(Debug)]
struct SpreadingRule {
    slice: Vec<bool>,
    output: bool,
}

impl FromStr for SpreadingRule {
    type Err = ParseBoolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" => ").collect::<Vec<&str>>();
        Ok(SpreadingRule {
            slice: parts[0].chars().map(|c| c == '#').collect(),
            output: parts[1] == "#",
        })
    }
}

fn parse(input_lines: &[Vec<String>]) -> (Vec<bool>, Vec<SpreadingRule>) {
    let gen_zero: Vec<bool> = input_lines[0][0]
        .split("initial state: ")
        .collect::<Vec<&str>>()[1]
        .chars()
        .map(|c| c == '#')
        .collect();

    let rules: Vec<SpreadingRule> = input_lines[0][1..]
        .iter()
        .map(|rule| rule.parse::<SpreadingRule>().unwrap())
        .collect();
    (gen_zero, rules)
}

pub fn day12(input_lines: &[Vec<String>]) -> (String, String) {
    let (gen_zero, rules) = parse(input_lines);

    let answer1 = 0;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day12;
    use crate::utils::load_input;

    #[test]
    fn check_day12_case01() {
        full_test(
            "",  // INPUT STRING
            "0", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day12(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
