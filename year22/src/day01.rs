use std::collections::BinaryHeap;

pub fn day01(input_lines: &[Vec<String>]) -> (String, String) {
    let sums: Vec<i32> = input_lines
        .iter()
        .map(|v| v.iter().map(|x| x.parse::<i32>().unwrap()).sum())
        .collect();
    let max_heap: BinaryHeap<i32> = BinaryHeap::from(sums);

    let answer1: i32 = *max_heap.peek().unwrap();
    let answer2: i32 = max_heap.into_iter_sorted().take(3).sum();
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day01;
    use crate::utils::load_input;

    #[test]
    fn check_day01_case01() {
        full_test(
            "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000", // INPUT STRING
            "24000", // PART 1 RESULT
            "45000", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day01(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
