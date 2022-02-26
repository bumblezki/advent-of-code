// Potential improvements:
//
use std::collections::HashSet;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
struct FabricClaim {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl FabricClaim {
    fn into_coords(&self) -> Vec<(i32, i32)> {
        let mut coords = Vec::new();
        for i in self.x..self.x+self.width {
            for j in self.y..self.y+self.height {
                coords.push( (i, j) );
            }
        }
        coords
    }

    fn from_input_line(input_line: &str) -> FabricClaim {
        let re = Regex::new(r"(\d+),(\d+): (\d+)x(\d+)").unwrap();
        re.captures(&input_line).map(|cap| {
            FabricClaim {
                x: FromStr::from_str(&cap[1]).unwrap(),
                y: FromStr::from_str(&cap[2]).unwrap(),
                width: FromStr::from_str(&cap[3]).unwrap(),
                height: FromStr::from_str(&cap[4]).unwrap(),
            }
        }).unwrap()
    }
}

pub fn day03(input_lines: &[Vec<String>]) -> (String, String) {

    let (claimed_inches, contested_inches) = input_lines[0].iter().map(|line| {
        FabricClaim::from_input_line(line)
    }).fold((HashSet::<(i32, i32)>::new(), HashSet::<(i32, i32)>::new()), |(mut claimed, mut contested), claim| {
        let square_inches = claim.into_coords();
        for square_inch in square_inches {
            if !claimed.insert(square_inch) {
                contested.insert(square_inch);
            }
        }
        (claimed, contested)
    });
    let answer1 = contested_inches.len();
    let answer2 = 0;
    (format!("{:?}", answer1), format!("{:?}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day03;
    use crate::utils::load_input;

    #[test]
    fn check_day03_case01() {
        full_test(
"", // INPUT STRING
"0", // PART 1 RESULT
"0" // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(day03(&input_lines), (part1_result.to_string(), part2_result.to_string()));
    }
}