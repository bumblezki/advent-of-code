fn str_to_decimal(s: &str) -> u64 {
    // Map the string slice to a binary number as a String.
    let binary_string = s
        .chars()
        .map(|c| match c {
            'L' | 'F' => '0',
            'R' | 'B' => '1',
            _ => panic!(),
        })
        .collect::<String>();
    // Then convert this binary string to an integer.
    u64::from_str_radix(&binary_string, 2).unwrap()
}

pub fn day05(input_lines: &[Vec<String>]) -> (String, String) {
    let seat_ids: Vec<u64> = input_lines[0]
        .iter()
        .map(|line| str_to_decimal(line))
        .collect();

    let highest: &u64 = seat_ids.iter().max().unwrap();
    let lowest: &u64 = seat_ids.iter().min().unwrap();

    let answer1 = highest;
    // The sum of consecutive integers from 1 to n (inclusive) is
    //   n * ( n + 1 ) / 2
    // The sum of consecutive integers between l and h (inclusive, where l < h) is the sum of consecutive integers
    // from 1 to h (inclusive) minus the sum of consecutive integers from 1 to l-1 so:
    //   h * ( h + 1 ) / 2 - ( l - 1 ) * l / 2
    // The difference between this value and sum of the seat IDs that have been accounted for is our missing seat ID.
    let answer2: u64 =
        highest * (highest + 1) / 2 - (lowest - 1) * lowest / 2 - &seat_ids.iter().sum();

    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day05;
    use crate::utils::load_input;

    #[test]
    fn check_day05_case01() {
        full_test(
            "FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL", // INPUT STRING
            "820", // PART 1 RESULT
            "0",   // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day05(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
