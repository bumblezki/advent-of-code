// Potential improvements:
//
use std::collections::{HashMap, HashSet};

pub fn day02(input_lines: &[Vec<String>]) -> (String, String) { 
    let char_maps: Vec<HashMap<char, i32>> = input_lines[0].iter().map(
        |id| id.chars().fold(
                HashMap::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            }
        )
    ).collect();
    let twos = char_maps.iter().fold(0, |accumulator, char_map| 
        accumulator + char_map.values().any(|&val| val == 2) as i32
    );
    let threes = char_maps.iter().fold(0, |accumulator, char_map| 
        accumulator + char_map.values().any(|&val| val == 3) as i32
    );
    let answer1 = twos * threes;
    
    let vec_of_subid_sets: Vec<HashSet<String>> = input_lines[0].iter().map(
        |id| id.chars().enumerate().fold(HashSet::new(), |mut sub_ids_set, (i, _)| {
                let mut string = id.clone();
                string.replace_range(i..i+1, "*");
                sub_ids_set.insert(string);
                sub_ids_set
            }
        )
    ).collect();
    let _ = vec_of_subid_sets.iter().fold(HashSet::new(), |mut unique_sub_ids_set, sub_ids_set| {
        let intersection = unique_sub_ids_set.intersection(&sub_ids_set);
        
        unique_sub_ids_set.extend(sub_ids_set.clone());
        unique_sub_ids_set
    });
    let answer2 = 0;
    (format!("{}", answer1), format!("{:?}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day02;
    use crate::utils::load_input;

    #[test]
    fn check_day02_case01() {
        full_test(
            "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab\n", // INPUT STRING
            "12", // PART 1 RESULT
            "abcde" // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(day02(&input_lines), (part1_result.to_string(), part2_result.to_string()));
    }
}