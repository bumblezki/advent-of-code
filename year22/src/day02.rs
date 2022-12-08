use once_cell::sync::OnceCell;
use regex::Regex;
use rotate_enum::RotateEnum;

// Rotate enum allows me to call .next() and .prev() on the enum.
#[derive(Copy, Clone, RotateEnum, PartialEq)]
enum Hand {
    Rock,
    Scissors,
    Paper,
}

impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if other == &self.next() {
            Some(std::cmp::Ordering::Greater)
        } else if other == self {
            Some(std::cmp::Ordering::Equal)
        } else if other == &self.prev() {
            Some(std::cmp::Ordering::Less)
        } else {
            unreachable!()
        }
    }
}

struct Game {
    my_hand: Hand,
    their_hand: Hand,
}

impl Game {
    fn compare(&self) -> u32 {
        let mut score = match &self.my_hand {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        };
        if self.my_hand > self.their_hand {
            score += 6
        } else if self.my_hand == self.their_hand {
            score += 3
        };
        score
    }

    fn from_str_part_1(s: &str) -> Self {
        static RE: OnceCell<Regex> = OnceCell::new();

        RE.get_or_init(|| Regex::new(r"([A-C]) ([X-Z])").unwrap())
            .captures(s)
            .map(|cap| Game {
                my_hand: match cap[2].parse().unwrap() {
                    'X' => Hand::Rock,
                    'Y' => Hand::Paper,
                    'Z' => Hand::Scissors,
                    _ => unreachable!(),
                },
                their_hand: match cap[1].parse().unwrap() {
                    'A' => Hand::Rock,
                    'B' => Hand::Paper,
                    'C' => Hand::Scissors,
                    _ => unreachable!(),
                },
            })
            .expect("Failed to match regular expression against input.")
    }

    fn from_str_part_2(s: &str) -> Self {
        static RE: OnceCell<Regex> = OnceCell::new();

        RE.get_or_init(|| Regex::new(r"([A-C]) ([X-Z])").unwrap())
            .captures(s)
            .map(|cap| {
                let their_hand = match cap[1].parse().unwrap() {
                    'A' => Hand::Rock,
                    'B' => Hand::Paper,
                    'C' => Hand::Scissors,
                    _ => unreachable!(),
                };
                Game {
                    my_hand: match cap[2].parse().unwrap() {
                        'X' => their_hand.next(),
                        'Y' => their_hand,
                        'Z' => their_hand.prev(),
                        _ => unreachable!(),
                    },
                    their_hand,
                }
            })
            .expect("Failed to match regular expression against input.")
    }
}

pub fn day02(input_lines: &[Vec<String>]) -> (String, String) {
    let answer1: &u32 = &input_lines[0]
        .iter()
        .map(|s| Game::from_str_part_1(s).compare())
        .sum();
    let answer2: &u32 = &input_lines[0]
        .iter()
        .map(|s| Game::from_str_part_2(s).compare())
        .sum();
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::{day02, Hand};
    use crate::utils::load_input;

    #[test]
    fn check_day02_case01() {
        full_test(
            "A Y
B X
C Z", // INPUT STRING
            "15", // PART 1 RESULT
            "12", // PART 2 RESULT
        )
    }

    #[test]
    fn check_hand_partial_ord() {
        assert!(Hand::Rock > Hand::Scissors);
        assert!(Hand::Scissors > Hand::Paper);
        assert!(Hand::Paper > Hand::Rock);
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day02(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
