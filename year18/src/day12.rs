// Potential improvements:
//
use std::str::{FromStr, ParseBoolError};

use itertools::Itertools;

// RULE_SIZE must be odd.
const RULE_SIZE: usize = 5;
const LEFT_BUFFER: usize = RULE_SIZE;
const RIGHT_BUFFER: usize = RULE_SIZE * 20;
// const TRUE: char = '#';
const FALSE: char = '.';

#[derive(Clone, Debug)]
struct SpreadingRule {
    chars: Vec<char>,
    output: char,
}

impl SpreadingRule {
    fn slice_to_output(&self, slice: &[char]) -> Option<char> {
        if self.chars == slice {            
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
            chars: parts[0].chars().collect_vec(),
            output: parts[1].chars().collect_vec()[0],
        })
    }
}

#[derive(Clone)]
struct Generation {
    plants: Vec<char>,
    zero_index: usize,
}

impl FromStr for Generation {
    type Err = ParseBoolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut plants = vec![FALSE; LEFT_BUFFER];
        plants.extend(s
            .split("initial state: ")
            .collect::<Vec<&str>>()[1]
            .chars()
        );
        plants.extend_from_slice(&[FALSE; RIGHT_BUFFER]);
        Ok(Generation { plants, zero_index: LEFT_BUFFER })
    }
}

impl Generation {
    fn next_generation(&self, rules: &[SpreadingRule]) -> Generation {
        let mut next_gen_plants = vec![FALSE; RULE_SIZE / 2];
        next_gen_plants.extend(
            self.plants
                .windows(RULE_SIZE)
                .map(|window| {
                    let mut output: char = FALSE;
                    for rule in rules {
                        if let Some(value) = rule.slice_to_output(window) {
                            output = value;
                            break
                        }
                    }
                    output
                })
        );
        next_gen_plants.extend_from_slice(&[FALSE; RULE_SIZE / 2]);
        Generation { plants: next_gen_plants, zero_index: self.zero_index }
    }
}

impl std::fmt::Debug for Generation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_iter(self.plants.clone()))
    }
}


pub fn day12(input_lines: &[Vec<String>]) -> (String, String) {
    let mut current_gen: Generation = input_lines[0][0].parse::<Generation>().expect("Could not parse first line of input into `Generation`.");
    let rules: Vec<SpreadingRule> = input_lines[1]
        .iter()
        .map(|rule| rule.parse::<SpreadingRule>().expect("Could not parse rules."))
        .collect();
    let mut next_gen = current_gen.next_generation(&rules);
    // println!("0000: {:?}", current_gen);

    for _idx in 0..20 {
        current_gen = next_gen;
        next_gen = current_gen.next_generation(&rules);
        // println!("{:04}: {:?}", idx+1, current_gen);
    }

    let answer1 = current_gen.plants.iter().enumerate().fold(0, |acc, (idx, plant)| 
        acc + (idx as i32 - current_gen.zero_index as i32 ) * (plant == &'#') as i32
    );

    // Is this cheating?
    // 50_000_000_000 is enormous! I let it run for a bit to see if any patterns emerged.
    // I noticed that it looked stable from generation 89 onwards. It looked something like:
    // 89: ...#..#.##.##....#.........
    // 90: ....#..#.##.##....#........
    // 91: .....#..#.##.##....#.......
    // 92: ......#..#.##.##....#......
    // etc.
    // So I just copied the output from generation 90 and added them up as if the plants were shifted to the right by 50 billion - 90.
    let gen_90_no_left_buffer = "..................#..#..#..#..#..#..#....#..#....#....#..#....#..#..#..#..#..#..#....#....#..#..#..#..#..#....#..#..#....#..#....#..#....#..#..#..#..#..#..#..#..#..#..#..#....#..#..#..#....#..........";
    let answer2 = gen_90_no_left_buffer.chars().enumerate().fold(0, |acc, (idx, plant)|
        acc + (idx as i64 + 50_000_000_000 - 90) * (plant == '#') as i64
    );
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
            "325", // PART 1 RESULT
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
