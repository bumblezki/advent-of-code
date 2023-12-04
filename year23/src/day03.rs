// Potential improvements:
//

use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct PartNumber { uuid: Uuid, number: u32 }

impl PartNumber {
    fn new(number: u32) -> Self {
        Self { uuid: Uuid::new_v4(), number }
    }
}

pub fn day03(input_lines: &[Vec<String>]) -> (String, String) {

    let mut part_number_map: HashMap<(usize, usize), HashSet<PartNumber>> = HashMap::new();
    let mut symbol_coords: Vec<(usize, usize)> = Vec::new();

    let mut current_num_str = Vec::new();

    for (ii, row) in input_lines[0].iter().enumerate() {
        let mut jj_chars = row.chars().enumerate();
        let mut start_col: usize = 0;
        while let Some((jj, c)) = jj_chars.next() {
            
            if c.is_digit(10) {
                if current_num_str.is_empty() {
                    start_col = jj;
                }
                current_num_str.push(c);
            } else {

                if c != '.' {
                    symbol_coords.push((ii, jj));
                }

                if !current_num_str.is_empty() {
                    let current_num = current_num_str.iter().collect::<String>().parse::<u32>().unwrap();
                    current_num_str.clear();

                    for yy in ii.saturating_sub(1)..=ii+1 {
                        for xx in start_col.saturating_sub(1)..=jj {
                            part_number_map.entry((yy, xx))
                                .or_insert(HashSet::new())
                                .insert(PartNumber::new(current_num));
                        }
                    }
                }
            }
        }
    }

    let mut part_numbers: HashSet<&PartNumber> = HashSet::new();
    for (ii, jj) in &symbol_coords {
        if let Some(pns) = part_number_map.get(&(*ii, *jj)) {
            for pn in pns {
                part_numbers.insert(pn);
            }
        }
    }
    // println!("part_numbers: {:?}", part_numbers.iter().map(|pns| pns.iter().map(|pn| pn.number).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>());
    // println!("symbol_coords: {:?}", symbol_coords);
    // for (coord, pn) in part_number_map.iter() {
    //     if pn.number == 755 {println!("{:?}: {}", coord, pn.number)}
        
    // }

    let answer1: u32 = part_numbers.iter().map(|pn| pn.number).sum();
    let answer2 = 0;
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
"0" // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(day03(&input_lines), (part1_result.to_string(), part2_result.to_string()));
    }
}