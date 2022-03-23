use std::fmt;
use itertools::Itertools;

const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Clone, Debug)]
struct PolymerUnit {
    char: char,
}

impl PolymerUnit {

    fn _new(char: char) -> Self {
        Self { char }
    }

    fn _reacts_with(&self, other: &Self) -> bool {
        self.char.to_ascii_lowercase() == other.char.to_ascii_lowercase() && self.char.is_lowercase() != other.char.is_lowercase()
    }

    fn _is_type(&self, char: char) -> bool {
        self.char.to_ascii_lowercase() == char.to_ascii_lowercase()
    }
}

impl fmt::Display for PolymerUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.char)
    }
}

fn _react_polymer_chain(chain: &mut Vec<PolymerUnit>) -> usize {
    let mut restart_idx: usize = 0;
    'outer: loop {
        for (idx, pair) in chain
                .windows(2)
                .enumerate()
                .skip(restart_idx) {
            if pair[0]._reacts_with(&pair[1]) {
                chain.remove(idx);
                chain.remove(idx);
                if idx != 0 {
                    restart_idx = idx - 1;
                } else {
                    restart_idx = 0;
                }
                continue 'outer;
            }
        }
        return chain.len();
    }
}


fn reaction(a: &char, b: &char) -> bool {
    a.to_ascii_lowercase() == b.to_ascii_lowercase() && a.is_lowercase() != b.is_ascii_lowercase()
}

fn unreacted_chain_len(chain: &Vec<char>) -> usize {
    let mut unreacted = Vec::<&char>::new();
    for c in chain {
        match unreacted.last() {
            Some(&last) if reaction(last, c) => {
                unreacted.pop();
            },
            _ => unreacted.push(c)
        }
    }
    unreacted.len()
}

pub fn day05(input_lines: &[Vec<String>]) -> (String, String) {
    let original_polymer_chain = input_lines[0][0]
        .chars().collect_vec();
    let answer1 = unreacted_chain_len(&original_polymer_chain);

    let answer2 = ALPHABET
        .chars()
        .into_iter()
        .map(|letter| {
            unreacted_chain_len(&original_polymer_chain
                .clone()
                .into_iter()
                .filter(|&c| c.to_ascii_lowercase() != letter )
                .collect()
            )
        })
        .min()
        .unwrap();
    
    (format!("{:?}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day05;
    use crate::utils::load_input;

    #[test]
    fn check_day05_case01() {
        full_test(
"dabAcCaCBAcCcaDA", // INPUT STRING
"10", // PART 1 RESULT
"4" // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(day05(&input_lines), (part1_result.to_string(), part2_result.to_string()));
    }
}