use itertools::Itertools;

pub fn day05(input_lines: &[Vec<String>]) -> (String, String) {
    
    let mut original_chain = input_lines[0][0].chars();
    loop {
        let (current, next) = match original_chain.next_tuple() {
            Some((char1, char2)) => (char1, char2),
            None => break
        };
        if current == next {
            break
        }
    }
    let answer1 = 0;
    let answer2 = 0;
    (format!("{:?}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day05;
    use crate::utils::load_input;

    #[test]
    fn check_day05_case01() {
        full_test(
"", // INPUT STRING
"0", // PART 1 RESULT
"0" // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(day05(&input_lines), (part1_result.to_string(), part2_result.to_string()));
    }
}