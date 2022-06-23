pub fn day08(input_lines: &[Vec<String>]) -> (String, String) {
    let mut queue = input_lines[0][0]
        .split(' ')
        .map(|val| val.parse::<i32>().unwrap());

    let mut stack = vec![(queue.next().unwrap(), queue.next().unwrap())];
    let mut total = 0;
    loop {
        let (mut child_count, metadata_count) = stack.pop().unwrap();

        if child_count == 0 {
            for _ in 1..=metadata_count {
                total += queue.next().unwrap();
            }
            if stack.is_empty() {
                total += queue.sum::<i32>();
                break;
            }
        } else {
            child_count -= 1;
            stack.push((child_count, metadata_count));
            stack.push((queue.next().unwrap(), queue.next().unwrap()));
        }
    }

    let answer1 = total;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day08;
    use crate::utils::load_input;

    #[test]
    fn check_day08_case01() {
        full_test(
            "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2", // INPUT STRING
            "138",                                 // PART 1 RESULT
            "0",                                   // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day08(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
