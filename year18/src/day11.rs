// Potential improvements:
//
use std::i32::MIN;
use nalgebra::DMatrix;

const POWER_GRID_SIZE: usize = 300;

pub fn day11(input_lines: &[Vec<String>]) -> (String, String) {
    let serial_number = input_lines[0][0].parse::<i32>().unwrap();

    // The question is 1-indexed, but the matrix is 0-indexed.
    let power_grid = DMatrix::<i32>::from_fn(
        POWER_GRID_SIZE, 
        POWER_GRID_SIZE, 
        |y, x| ((x as i32 + 11) * (y as i32 + 1) + serial_number) * (x as i32 + 11) / 100 % 10 - 5
    );
    
    let mut max_power: i32 = MIN;
    let mut max_indices= (0 ,0);
    let subgrid_size = 3;
    for jj in 0..=POWER_GRID_SIZE-subgrid_size {
        for ii in 0..=POWER_GRID_SIZE-subgrid_size {
            let new_power = power_grid.slice((jj,ii),(subgrid_size,subgrid_size)).sum();
            if new_power > max_power {
                max_power = new_power;
                max_indices = (ii+1, jj+1);
            }
        }
    }

    let answer1 = max_indices;

    let mut max_power: i32 = MIN;
    let mut max_indices= (0 ,0);
    let mut max_subgrid_size = 0;
    for subgrid_size in 1..=POWER_GRID_SIZE {
        for jj in 0..=POWER_GRID_SIZE-subgrid_size {
            for ii in 0..=POWER_GRID_SIZE-subgrid_size {
                let new_power = power_grid.slice((jj,ii),(subgrid_size,subgrid_size)).sum();
                if new_power > max_power {
                    max_power = new_power;
                    max_indices = (ii+1, jj+1);
                    max_subgrid_size = subgrid_size;
                }
            }
        }
    }

    let answer2 = (max_indices, max_subgrid_size);
    (format!("{:?}", answer1), format!("{:?}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day11;
    use crate::utils::load_input;

    #[test]
    fn check_day11_case01() {
        full_test(
            "18",  // INPUT STRING
            "(33, 45)", // PART 1 RESULT
            "((90, 269), 16)", // PART 2 RESULT
        )
    }

    #[test]
    fn check_day11_case02() {
        full_test(
            "42",
            "(21, 61)",
            "((232, 251), 12)"
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day11(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
