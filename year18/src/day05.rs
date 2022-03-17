use std::fmt;

const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Clone, Debug, PartialEq)]
enum Polarity {
    Upper,
    Lower
}

#[derive(Clone, Debug)]
struct PolymerUnit {
    raw_char: char,
    lowercase_char: char,
    polarity: Polarity,
}

impl PolymerUnit {

    fn new(char: char) -> Self {
        let polarity: Polarity = if char.is_lowercase() {
            Polarity::Lower
        } else {
            Polarity::Upper
        };
        Self {
            raw_char: char,
            lowercase_char: char.to_ascii_lowercase(),
            polarity,
        }
    }

    fn reacts_with(&self, other: &Self) -> bool {
        self.lowercase_char == other.lowercase_char && self.polarity != other.polarity
    }

    fn is_type(&self, char: char) -> bool {
        self.lowercase_char == char.to_ascii_lowercase()
    }
}

impl fmt::Display for PolymerUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw_char)
    }
}

fn react_polymer_chain(chain: &mut Vec<PolymerUnit>) -> &mut Vec<PolymerUnit> {
    let mut restart_idx: usize = 0;
    'outer: loop {
        for (idx, pair) in chain.windows(2).enumerate().skip(restart_idx) {
            if pair[0].reacts_with(&pair[1]) {
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
        return chain;
    }
    
}

pub fn day05(input_lines: &[Vec<String>]) -> (String, String) {
    let original_polymer_chain = input_lines[0][0]
        .chars()
        .map(|char| PolymerUnit::new(char))
        .collect::<Vec<PolymerUnit>>();

    let mut polymer_chain = original_polymer_chain.clone();
    let reacted_polymer_chain = react_polymer_chain(&mut polymer_chain);
    let answer1 = reacted_polymer_chain.len();

    let mut shortest_reacted_polymer_chain_len = original_polymer_chain.len();
    for char in ALPHABET.chars() {
        let mut polymer_chain = original_polymer_chain
            .clone()
            .into_iter()
            .filter(|unit| !unit.is_type(char))
            .collect::<Vec<PolymerUnit>>();
        let reacted_polymer_chain = react_polymer_chain(&mut polymer_chain);
        if reacted_polymer_chain.len() < shortest_reacted_polymer_chain_len {
            shortest_reacted_polymer_chain_len = reacted_polymer_chain.len();
        }
    }

    let answer2 = shortest_reacted_polymer_chain_len;
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