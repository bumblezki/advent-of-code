// Potential improvements:
//
pub fn day01(input_lines: &[Vec<String>]) -> (String, String) {
    let answer1 = input_lines[0]
                        .iter()
                        .fold(0, |accumulator, line| accumulator + line.parse::<i32>().unwrap());
    let mut frequencies = vec![0];
    let mut accumulator = 0;
    'outer: loop {
        for line in &input_lines[0] {
            accumulator += line.parse::<i32>().unwrap();
            
            if frequencies.contains(&accumulator) {
                break 'outer
            }
            frequencies.push(accumulator);
        }
    }
    let answer2 = accumulator;
    
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day01;
    use crate::utils::load_input;

    #[test]
    fn check_day01_case01() {
        full_test(
"+1\n-2\n+3\n+1\n+1\n-2\n-2", // INPUT STRING
"0", // PART 1 RESULT
"2" // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(day01(&input_lines), (part1_result.to_string(), part2_result.to_string()));
    }
}