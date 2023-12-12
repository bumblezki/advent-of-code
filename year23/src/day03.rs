use std::collections::HashMap;

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

    let height = input_lines[0].len();

    let mut gear_map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    let mut answer1: u32 = 0;

    for (ii, row) in input_lines[0].iter().enumerate() {

        let mut num_builder = Vec::new();
        let mut start_col: usize = 0;
        for (jj, cc) in row.char_indices() {
            if cc.is_ascii_digit() {
                if num_builder.is_empty() {
                    start_col = jj;
                }
                num_builder.push(cc);
            }

            if !num_builder.is_empty() && (!cc.is_ascii_digit() || jj == row.len() - 1) {
                let num = num_builder.iter().collect::<String>().parse::<u32>().unwrap();
                num_builder.clear();
                let mut touches = false;
                for yy in clamp(ii.saturating_sub(1), 0, height - 1)..=clamp(ii+1, 0, height - 1) {
                    for xx in clamp(start_col.saturating_sub(1), 0, row.len() - 1)..clamp(jj+1, 0, row.len() - 1) {
                        gear_map.entry((xx, yy)).or_default().push(num);
                        let this_char = input_lines[0][yy].chars().nth(xx).unwrap();
                        if this_char != '.' && !this_char.is_ascii_digit() {
                            touches = true;
                        } 
                    }
                }
                if touches {
                    answer1 += num;
                }
            } 
        }
    }
    
    let mut answer2 = 0;
    for (ii, row) in input_lines[0].iter().enumerate() {
        for (jj, cc) in row.char_indices() {
            if cc == '*' {
                if let Some(nums) = gear_map.get(&(jj, ii)) {
                    if nums.len() == 2 {
                        answer2 += nums[0] * nums[1];
                    }
                }
            }
        }
    }
    
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
"467835" // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(day03(&input_lines), (part1_result.to_string(), part2_result.to_string()));
    }
}