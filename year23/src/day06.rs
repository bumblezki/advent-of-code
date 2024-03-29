// Potential improvements:
//

struct Race {
    /// The length of the race, T.
    time: u64,
    /// The incumbent record for distance covered in the race.
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    fn distance(&self, button_time: u64) -> Option<u64> {
        // Length of button press, B, is the speed because you gain
        // 1 mm/ms for each ms you hold the button.
        // s = B
        // The time remaining to travel at speed B is T - B, where T
        // is the (time) length of the race.
        // t = T - B
        // d = st = B(T - B)
        (self.time > button_time).then_some(button_time * (self.time - button_time))
    }

    fn winning_distance_count(&self) -> u64 {
        (0..self.time)
            .filter_map(|button_time| {
                (self.distance(button_time).unwrap() > self.distance).then_some(1)
            })
            .sum()
    }
}

pub fn day06(input_lines: &[Vec<String>]) -> (String, String) {
    let answer1 = input_lines[0][0]
        // Parse the input into an iterator of u64 tuples.
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .zip(
            input_lines[0][1]
                .split_whitespace()
                .skip(1)
                .map(|s| s.parse::<u64>().unwrap()),
        )
        // Now do the logic.
        .map(|(time, distance)| Race::new(time, distance).winning_distance_count())
        .product::<u64>();

    let answer2 = Race::new(
        input_lines[0][0]
            .split_once(':')
            .unwrap()
            .1
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u64>()
            .unwrap(),
        input_lines[0][1]
            .split_once(':')
            .unwrap()
            .1
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u64>()
            .unwrap(),
    )
    .winning_distance_count();
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day06;
    use crate::utils::load_input;

    #[test]
    fn check_day06_case01() {
        full_test(
            "Time:      7  15   30
Distance:  9  40  200", // INPUT STRING
            "288",   // PART 1 RESULT
            "71503", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day06(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
