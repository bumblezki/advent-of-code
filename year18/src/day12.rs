// Potential improvements:
//
use std::str::{FromStr, ParseBoolError};

use itertools::Itertools;

const RULE_SIZE: usize = 5;

#[derive(Clone, Debug)]
struct SpreadingRule {
    slice: Vec<bool>,
    output: bool,
}

impl SpreadingRule {
    fn slice_to_output(&self, other_slice: &[bool]) -> Option<bool> {
        if &self.slice == other_slice {
            Some(self.output)
        } else {
            None
        }
    }
}

impl FromStr for SpreadingRule {
    type Err = ParseBoolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" => ").collect::<Vec<&str>>();
        Ok(SpreadingRule {
            slice: parts[0].chars().map(|c| c == '#').collect_vec(),
            output: parts[1] == "#",
        })
    }
}

struct Generation {
    plants: Vec<bool>,
}

impl FromStr for Generation {
    type Err = ParseBoolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let plants: Vec<bool> = s
            .split("initial state: ")
            .collect::<Vec<&str>>()[1]
            .chars()
            .map(|c| c == '#')
            .collect();
        Ok(Generation { plants })
    }
}

impl Generation {
    fn next_generation(&self, rules: Vec<SpreadingRule>) -> Generation {
        let next_gen_plants = self.plants
            .clone()
            .windows(RULE_SIZE)
            .map(|window| {
                let mut output: bool = window[2];
                for rule in &rules {
                    if let Some(value) = rule.slice_to_output(window) {
                        output = value;
                        break
                    }
                }
                output
            }).collect_vec();
        Generation { plants: next_gen_plants }
    }
}

impl std::fmt::Display for Generation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self.plants.iter().fold(String::new(), |s, &b| {
            if b {
                format!("{s}#")
            } else {
                format!("{s}.")
            }
        });
        write!(f, "{}", out)
    }
}


pub fn day12(input_lines: &[Vec<String>]) -> (String, String) {
    let gen_zero: Generation = input_lines[0][0].parse::<Generation>().expect("Could not parse first line of input into `Generation`.");
    let rules: Vec<SpreadingRule> = input_lines[0][1..]
        .iter()
        .map(|rule| rule.parse::<SpreadingRule>().unwrap())
        .collect();
    
    let gen_one = gen_zero.next_generation(rules);

    println!("{}", gen_zero);
    println!("{}", gen_one);

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
            "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #",  // INPUT STRING
            "1", // PART 1 RESULT
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
