// Potential improvements:
//
use std::collections::HashSet;

pub fn day06(input_lines: &[Vec<String>]) -> (String, String) {
    // Collect the answers into a list of lists of sets of characters. E.g.,
    // 
    // abc
    //
    // a
    // b
    // c
    //
    // ab
    // ac
    //
    // a
    // a
    // a
    // a
    //
    // b
    //
    // goes to
    //
    // [
    //     [
    //         {'a', 'b', 'c'}
    //     ],
    //     [
    //         {'a'}, {'b'}, {'c'}
    //     ],
    //     [
    //         {'a', 'b'}, {'a', 'c'}
    //     ],
    //     [
    //         {'a'}, {'a'}, {'a'}, {'a'}
    //     ],
    //     [
    //         {'b'}
    //     ]
    // ]
    let groups: Vec<Vec<HashSet<char>>> = input_lines
        .iter()
        .map(|group_lines| {
            group_lines
                .iter()
                .map(|line| HashSet::from_iter(line.chars()))
                .collect()
        })
        .collect();

    // Then for each group we find the size of the union of all the sets in the group and sum them...
    let answer1: usize = groups
        .iter()
        .map(|group| {
            group.iter().fold(HashSet::new(), |acc, set| {
                acc.union(&set).copied().collect::<HashSet<_>>()
            })
        })
        .map(|set| set.len())
        .sum();
    // Then do the same for the intersection...
    let answer2: usize = groups
        .into_iter()
        .map(|group| {
            group
                .into_iter()
                .reduce(|left, right| left.intersection(&right).copied().collect::<HashSet<_>>())
                .unwrap_or(HashSet::new())
        })
        .map(|set| set.len())
        .sum();
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day06;
    use crate::utils::load_input;

    #[test]
    fn check_day06_case01() {
        full_test(
            "abc

a
b
c

ab
ac

a
a
a
a

b",  // INPUT STRING
            "11", // PART 1 RESULT
            "6", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day06(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
