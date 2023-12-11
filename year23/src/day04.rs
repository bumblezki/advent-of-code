// Potential improvements:
//
use anyhow::Result;
use std::collections::{BTreeMap, HashSet};
use std::str::FromStr;

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
        let mut card_id = parts.next().unwrap().split(' ').filter(|s| s != &"");
        let id = card_id.nth(1).unwrap().parse::<u32>().unwrap();

        let mut card_numbers = parts.next().unwrap().split(" | ");
        let card_winners = card_numbers.next().unwrap().split(' ').filter(|s| s != &"");
        let card_numbers = card_numbers.next().unwrap().split(' ').filter(|s| s != &"");

        for winner in card_winners {
            winners.insert(winner.parse::<u32>().unwrap());
        }
        for number in card_numbers {
            numbers.push(number.parse::<u32>().unwrap());
        }

        Ok(Self {
            id,
            winners,
            numbers,
        })
    }
}

impl ScratchCard {
    fn win_count(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|n| self.winners.contains(n))
            .count() as u32
    }

    fn score(&self) -> Option<u32> {
        match self.win_count() {
            0 => None,
            wc => Some(u32::pow(2, wc - 1)),
        }
    }

    fn copies(&self) -> HashSet<u32> {
        (self.id + 1..=self.id + self.win_count()).collect::<HashSet<u32>>()
    }
}

fn num_copies(sum: &mut u32, copy_map: &BTreeMap<u32, HashSet<u32>>, id: u32) {
    let copies = copy_map.get(&id).unwrap();
    if !copies.is_empty() {
        copies.iter().for_each(|c| {
            *sum += 1;
            num_copies(sum, copy_map, *c);
        });
    }
}

pub fn day04(input_lines: &[Vec<String>]) -> (String, String) {
    let scratch_cards = input_lines[0]
        .iter()
        .map(|s| s.parse::<ScratchCard>().unwrap())
        .collect::<Vec<ScratchCard>>();

    let answer1 = scratch_cards
        .iter()
        .filter_map(|sc| sc.score())
        .sum::<u32>();

    let copy_map = scratch_cards.iter().fold(BTreeMap::new(), |mut map, sc| {
        map.insert(sc.id, sc.copies());
        map
    });

    let mut answer2 = 0;
    for id in 1..=copy_map.len() {
        answer2 += 1;
        num_copies(&mut answer2, &copy_map, id as u32)
    }

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
            "30", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day04(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
