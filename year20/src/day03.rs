use std::num::TryFromIntError;

fn count_trees_for_slope(v: (usize, usize), lines: &[String]) -> Result<u64, TryFromIntError> {
    lines
        .iter()
        .step_by(v.1)
        .enumerate()
        .filter(|(counter, line)| line.chars().cycle().nth(v.0 * counter).unwrap() == '#')
        .count()
        .try_into()
}

pub fn day03(input_lines: &[Vec<String>]) -> (String, String) {
    let answer1 = count_trees_for_slope((3, 1), &input_lines[0]).unwrap();
    let answer2: u64 = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&v| count_trees_for_slope(v, &input_lines[0]).unwrap())
        .product();
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day03;
    use crate::utils::load_input;

    #[test]
    fn check_day03_case01() {
        full_test(
            "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#", // INPUT STRING
            "7",   // PART 1 RESULT
            "336", // PART 2 RESULT
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
