// Potential improvements:
//
use std::collections::{HashMap, HashSet};

pub fn day02(input_lines: &[Vec<String>]) -> (String, String) {
    let char_maps: Vec<HashMap<char, i32>> = input_lines[0]
        .iter()
        .map(|id| {
            id.chars().fold(HashMap::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            })
        })
        .collect();
    let (twos, threes) = char_maps.iter().fold((0, 0), |(twos, threes), char_map| {
        (
            twos + char_map.values().any(|&val| val == 2) as i32,
            threes + char_map.values().any(|&val| val == 3) as i32,
        )
    });
    let answer1 = twos * threes;

    // For every ID in the list, cycle through it's characters, replacing each with a *
    // (resulting in permuted IDs) then add all these permutations of the IDs to a single vector.
    // E.g. The input file
    //   abc
    //   def
    //   ghi
    // goes to
    //   [ '*bc', 'a*c', 'ab*', '*ef', 'd*f', 'de*', '*hi', 'g*i', 'gh*' ]
    let permuted_ids: Vec<String> = input_lines[0]
        .iter()
        .map(|id| {
            id.chars()
                .enumerate()
                .fold(Vec::new(), |mut permutations, (i, _)| {
                    let mut string = id.clone();
                    string.replace_range(i..i + 1, "*");
                    permutations.push(string);
                    permutations
                })
        })
        .fold(Vec::new(), |mut all_permutations, permutations| {
            all_permutations.extend(permutations);
            all_permutations
        });

    // Add all the permuted IDs to a set until an entry already exists.
    // Remove the '*' placeholder before yielding the answer.
    let mut answer2 = String::new();
    let mut permutated_ids_set = HashSet::new();
    for id in permuted_ids.iter() {
        if !permutated_ids_set.insert(id) {
            answer2 = id.replace('*', "");
            break;
        }
    }

    (answer1.to_string(), answer2)
}

#[cfg(test)]
mod tests {
    use super::day02;
    use crate::utils::load_input;

    #[test]
    fn check_day02_case01() {
        full_test(
            "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab\n", // INPUT STRING
            "12",                                                       // PART 1 RESULT
            "abcde",                                                    // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day02(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
