fn str_to_decimal(s: &str, one_char: char, zero_char: char) -> u64 {
    u64::from_str_radix(
        &s.chars()
            .map(|c| match c {
                one if one == one_char => '1',
                zero if zero == zero_char => '0',
                _ => panic!(
                    "Character in seat address was not {} or {}",
                    one_char, zero_char
                ),
            })
            .collect::<String>(),
        2,
    )
    .unwrap()
}

pub fn day05(input_lines: &[Vec<String>]) -> (String, String) {
    let mut seat_ids: Vec<u64> = input_lines[0]
        .iter()
        .map(|line| {
            let (row, col) = line.split_at(7);
            str_to_decimal(row, 'B', 'F') * 8 + str_to_decimal(col, 'R', 'L')
        })
        // I could have called .max().unwrap() here for part one rather than sorting the Vec and taking the last element.
        // However, for part two, I needed to have a sorted Vec anyway.
        .collect();

    seat_ids.sort();

    let answer1: &u64 = seat_ids.last().unwrap();

    let mut answer2: u64 = 0;
    for id_window in seat_ids.windows(2) {
        if id_window[0] + 1 < id_window[1] {
            answer2 = id_window[0] + 1;
        }
    }

    (format!("{:?}", answer1), format!("{}", answer2))
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
