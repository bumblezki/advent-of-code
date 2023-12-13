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
        if self.time < button_time {
            return None;
        }
        // Length of button press, B, is the speed because you gain
        // 1 mm/ms for each ms you hold the button.
        // s = B
        // The time remaining to travel at speed B is T - B, where T
        // is the (time) length of the race.
        // t = T - B
        // d = st = B(T - B)
        Some(button_time * (self.time - button_time))
    }

    fn winning_distance_count(&self) -> u64 {
        let mut win_count = 0;
        let mut time = 0;
        while let Some(distance) = self.distance(time) {
            // println!("{}: {} | {}: {}", self.time, self.distance, time, distance);
            if distance > self.distance {
                win_count += 1;
            }
            time += 1;
        }
        win_count
    }
}

pub fn day06(input_lines: &[Vec<String>]) -> (String, String) {
    let races: Vec<Race> = input_lines[0][0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .zip(
            input_lines[0][1]
                .split_whitespace()
                .skip(1)
                .map(|s| s.parse::<u64>().unwrap()),
        )
        .map(|(time, distance)| Race::new(time, distance))
        .collect();

    let answer1 = races.iter().map(|r| r.winning_distance_count()).product::<u64>();

    let long_race = Race::new(
        input_lines[0][0]
            .split_once(':')
            .unwrap().1
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u64>()
            .unwrap(),
        input_lines[0][1]
            .split_once(':')
            .unwrap().1
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse::<u64>()
            .unwrap(),
    );
    let answer2 = long_race.winning_distance_count();
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
Distance:  9  40  200",  // INPUT STRING
            "288", // PART 1 RESULT
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
