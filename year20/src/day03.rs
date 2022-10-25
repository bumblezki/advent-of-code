fn count_trees_for_slope(down: usize, across: usize, lines: &[String]) -> u64 {
    lines
        .iter()
        .step_by(down)
        .enumerate()
        .fold(0, |mut tree_count, (counter, line)| {
            if line.chars().cycle().nth(across * counter).unwrap() == '#' {
                tree_count += 1;
            }
            tree_count
        })
}

pub fn day03(input_lines: &[Vec<String>]) -> (String, String) {
    let answer1 = count_trees_for_slope(1, 3, &input_lines[0]);
    let answer2: u64 = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .iter()
        .map(|&(down, across)| count_trees_for_slope(down, across, &input_lines[0]))
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
