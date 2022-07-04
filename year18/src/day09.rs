// Potential improvements:
//
use std::collections::VecDeque;
use regex::Regex;


pub fn day09(input_lines: &[Vec<String>]) -> (String, String) {
    let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    let caps = re.captures(&input_lines[0][0]).unwrap();
    let player_count = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let last_marble_value = caps.get(2).unwrap().as_str().parse::<u64>().unwrap() * 100;

    let mut circle  = VecDeque::new();
    let mut player_queue = VecDeque::new();
    for _ in 0..player_count {
        player_queue.push_back(0);
    }
    circle.push_back(0);
    circle.push_back(2);
    circle.push_back(1);
    for marble_value in 3..=last_marble_value {
        let mut player = player_queue.pop_front().unwrap();

        if marble_value % 23 == 0 {
            player += marble_value;
            circle.rotate_right(8);
            player += circle.pop_back().unwrap();
            circle.rotate_left(2);

        } else {
            circle.push_back(marble_value);
            circle.rotate_left(1);
        }
        // println!("{:?}", circle);
        
        player_queue.push_back(player);
    }

    let answer1 = player_queue.iter().max().unwrap();
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day09;
    use crate::utils::load_input;

    #[test]
    fn check_day09_case01() {
        full_test(
            "9 players; last marble is worth 25 points",  // INPUT STRING
            "3sf2", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day09(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
