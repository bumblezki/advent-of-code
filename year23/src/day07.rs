// Potential improvements:
//
use counter::Counter;
use std::cmp::{Ord, Ordering, PartialOrd};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: char) -> Self {
        match c {
            'A' => Card::Ace,
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            _ => panic!("Invalid card character: {}", c),
        }
    }

    fn from_char_with_jokers(c: char) -> Self {
        match c {
            'A' => Card::Ace,
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Joker,
            'Q' => Card::Queen,
            'K' => Card::King,
            _ => panic!("Invalid card character: {}", c),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Hand {
    bid: u64,
    cards: [Card; 5],
    hand_type: HandType,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                let card_ordering = self_card.cmp(other_card);
                if card_ordering != Ordering::Equal {
                    return card_ordering;
                }
            }
            Ordering::Equal
        } else {
            self.hand_type.cmp(&other.hand_type)
        }
    }
}

impl Hand {
    fn from_str(s: &str, with_jokers: bool) -> Self {
        let mut cards = [Card::Two; 5];
        let split = s.split_once(' ').unwrap();

        let mut card_iter = split.0.chars();
        for card in cards.iter_mut() {
            if with_jokers {
                *card = Card::from_char_with_jokers(card_iter.next().unwrap());
            } else {
                *card = Card::from_char(card_iter.next().unwrap());
            }
        }

        let card_counts = cards.iter().collect::<Counter<&Card>>();
        let joker_count = *card_counts.get(&Card::Joker).unwrap_or(&0);

        let hand_type = match (&card_counts, card_counts.len()) {
            (_, 1) => HandType::FiveOfAKind,
            (ccs, 2) => match joker_count {
                0 => {
                    if ccs.values().any(|&count| count == 4) {
                        HandType::FourOfAKind
                    } else {
                        HandType::FullHouse
                    }
                }
                _ => HandType::FiveOfAKind,
            },
            (ccs, 3) => {
                if ccs.values().any(|&count| count == 3) {
                    match joker_count {
                        3 => HandType::FourOfAKind,
                        1 => HandType::FourOfAKind,
                        _ => HandType::ThreeOfAKind,
                    }
                } else {
                    match joker_count {
                        2 => HandType::FourOfAKind,
                        1 => HandType::ThreeOfAKind,
                        _ => HandType::TwoPair,
                    }
                }
            }
            (_, 4) => match joker_count {
                1 | 2 => HandType::ThreeOfAKind,
                _ => HandType::OnePair,
            },
            (_, 5) => HandType::HighCard,
            _ => panic!("Invalid number of cards: {}", cards.len()),
        };

        Hand {
            bid: split.1.parse::<u64>().unwrap(),
            cards,
            hand_type,
        }
    }
}

pub fn day07(input_lines: &[Vec<String>]) -> (String, String) {
    let mut hands = input_lines[0]
        .iter()
        .map(|line| Hand::from_str(line, false))
        .collect::<Vec<Hand>>();
    hands.sort();

    let answer1 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u64 + 1))
        .sum::<u64>();

    let mut joker_hands = input_lines[0]
        .iter()
        .map(|line| Hand::from_str(line, true))
        .collect::<Vec<Hand>>();
    joker_hands.sort();
    let answer2 = joker_hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u64 + 1))
        .sum::<u64>();
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day07;
    use crate::utils::load_input;

    #[test]
    fn check_day07_case01() {
        full_test(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483", // INPUT STRING
            "6440", // PART 1 RESULT
            "5905", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day07(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
