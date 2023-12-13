// Potential improvements:
//

pub fn day06(input_lines: &[Vec<String>]) -> (String, String) {
    let output: Vec<(u64, u64)> = input_lines[0][0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .zip(
            input_lines[0][1]
                .split_whitespace()
                .skip(1)
                .map(|s| s.parse::<u64>().unwrap()),
        )
        .map(|(time, distance)| (time, distance))
        .collect();
    
    println!("{:?}", output);
    let answer1 = 0;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day06;
    use crate::utils::load_input;

    #[test]
    fn check_day06_case01() {
        full_test(
            "",  // INPUT STRING
            "0", // PART 1 RESULT
            "0", // PART 2 RESULT
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
