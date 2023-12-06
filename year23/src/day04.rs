// Potential improvements:
//
use std::collections::HashSet;
use std::str::FromStr;
use anyhow::Result;

#[derive(Debug)]
struct ScratchCard {
    id: u32,
    winners: HashSet<u32>,
    numbers: Vec<u32>,
}

// impl FromStr for ScratchCard  where the string looks like: "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
impl FromStr for ScratchCard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut winners = HashSet::new();
        let mut numbers = Vec::new();
        
        let mut parts = s.split(": ");
        let mut card_id = parts.next().unwrap().split(" ").filter(|s| s != &"");
        let id = card_id.nth(1).unwrap().parse::<u32>().unwrap();

        let mut card_numbers = parts.next().unwrap().split(" | ");
        let card_winners = card_numbers.next().unwrap().split(" ").filter(|s| s != &"");
        let card_numbers = card_numbers.next().unwrap().split(" ").filter(|s| s != &"");
        for winner in card_winners {
            winners.insert(winner.parse::<u32>().unwrap());
        }
        for number in card_numbers {
            numbers.push(number.parse::<u32>().unwrap());
        }

        Ok(Self { id, winners, numbers })
    }
}

impl ScratchCard {
    fn score(&self) -> Option<u32> {
        let win_count = self.numbers.iter().filter(|n| self.winners.contains(n)).count();
        if win_count > 0 {
            let score = u32::pow(2, win_count as u32 - 1) ;
            println!("id: {}, win_count: {}, score: {}", self.id, win_count, score);
            Some(score)
        } else {
            None
        }
    }

    fn copies(&self) -> Vec<ScratchCard> {
        let mut copies = Vec::new();
        for _ in 0..self.numbers.len() {
            copies.push(Self { id: self.id, winners: self.winners.clone(), numbers: self.numbers.clone() });
        }
        copies
    }
}

pub fn day04(input_lines: &[Vec<String>]) -> (String, String) {
    let scratch_cards = input_lines[0].iter().map(|s| s.parse::<ScratchCard>().unwrap()).collect::<Vec<ScratchCard>>();

    let answer1 = scratch_cards.iter().filter_map(|sc| sc.score()).sum::<u32>();
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day04;
    use crate::utils::load_input;

    #[test]
    fn check_day04_case01() {
        full_test(
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", // INPUT STRING
"13", // PART 1 RESULT
"0" // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(day04(&input_lines), (part1_result.to_string(), part2_result.to_string()));
    }
}
