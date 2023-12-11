// Potential improvements:
//
fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}

pub fn day03(input_lines: &[Vec<String>]) -> (String, String) {
    let num_rows = input_lines[0].len();
    let num_cols = input_lines[0][0].len();

    let mut total: u32 = 0;

    for (ii, row) in input_lines[0].iter().enumerate() {
        let mut current_num_str = Vec::new();
        let mut jj_chars = row.chars().enumerate();
        let mut start_col: usize = 0;
        while let Some((jj, cc)) = jj_chars.next() {
            if cc.is_digit(10) {
                if current_num_str.is_empty() {
                    start_col = jj;
                }
                current_num_str.push(cc);
            } else {
                if !current_num_str.is_empty() {
                    let current_num = current_num_str
                        .iter()
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap();
                    current_num_str.clear();

                    'search: for yy in ii.saturating_sub(1)..=ii + 1 {
                        for xx in start_col.saturating_sub(1)..=jj {
                            println!(
                                "input_lines: {:?}",
                                input_lines[clamp(yy, 0, num_rows - 1)][clamp(xx, 0, num_cols - 1)]
                            );
                            let neighbour = input_lines[clamp(yy, 0, num_rows - 1)]
                                [clamp(xx, 0, num_cols - 1)]
                            .chars()
                            .next()
                            .unwrap();
                            if !neighbour.is_digit(10) && neighbour != '.' {
                                total += current_num;
                                println!("{}: {}", current_num, neighbour);
                                break 'search;
                            }
                        }
                    }
                }
            }
        }
    }

    let answer1: u32 = total;
    let answer2 = 10;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day03;
    use crate::utils::load_input;

    #[test]
    fn check_day03_case01() {
        full_test(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..", // INPUT STRING
            "4361", // PART 1 RESULT
            "0",    // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day03(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
