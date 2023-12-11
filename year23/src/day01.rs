use std::collections::HashMap;

fn calibration_value(
    line: &String,
    digit_maps: Option<(&HashMap<&str, &str>, &HashMap<&str, &str>)>,
) -> u32 {
    let mut mut_line = line.clone();
    if digit_maps.is_some() {
        let (special_case_digit_map, digit_map) = digit_maps.unwrap();
        for (key, value) in special_case_digit_map {
            mut_line = mut_line.replace(key, value);
        }
        for (key, value) in digit_map {
            mut_line = mut_line.replace(key, value);
        }
    }
    mut_line.retain(|c| c.is_numeric());
    let digits: Vec<u32> = mut_line.chars().map(|c| c.to_digit(10).unwrap()).collect();
    digits[0] * 10 + digits[digits.len() - 1]
}

pub fn day01(input_lines: &[Vec<String>]) -> (String, String) {
    let answer1 = input_lines[0]
        .iter()
        .fold(0, |acc, line| acc + calibration_value(line, None));
    let special_case_digit_map = HashMap::from([
        ("oneight", "18"),
        ("twone", "21"),
        ("threeight", "38"),
        ("fiveight", "58"),
        ("sevenine", "79"),
        ("eightwo", "82"),
        ("eighthree", "83"),
        ("nineight", "98"),
    ]);
    let digit_map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    let answer2 = input_lines[0].iter().fold(0, |acc, line| {
        acc + calibration_value(line, Some((&special_case_digit_map, &digit_map)))
    });
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day01;
    use crate::utils::load_input;

    #[test]
    fn check_day01_case01() {
        full_test(
            "two1nine
eigh1twothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen", // INPUT STRING
            "220", // PART 1 RESULT
            "211", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day01(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
