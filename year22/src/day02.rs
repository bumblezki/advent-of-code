use rotate_enum::RotateEnum;
use std::cmp::{Ordering, PartialEq, PartialOrd};


// Rotate enum allows me to call .next() and .prev() on the enum.
#[derive(Copy, Clone, RotateEnum, PartialEq)]
enum Hand {
    Rock,
    Scissors,
    Paper,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if other == &self.next() {
            Some(Ordering::Greater)
        } else if other == self {
            Some(Ordering::Equal)
        } else if other == &self.prev() {
            Some(Ordering::Less)
        } else {
            unreachable!()
        }
    }
}


pub fn day02(_input_lines: &[Vec<String>]) -> (String, String) {
    println!("Paper beats Rock: {:?}", Hand::Paper > Hand::Rock);
    println!("Paper beats Scissors: {:?}", Hand::Paper > Hand::Scissors);
    
    println!("Rock beats Scissors: {:?}", Hand::Rock > Hand::Scissors);
    println!("Rock beats Paper: {:?}", Hand::Rock > Hand::Paper);

    let answer1 = 0;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::{day02, Hand};
    use crate::utils::load_input;

    #[test]
    fn check_day02_case01() {
        full_test(
            "",  // INPUT STRING
            "0", // PART 1 RESULT
            "0", // PART 2 RESULT
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
