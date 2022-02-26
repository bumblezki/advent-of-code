// Potential improvements:
//
use std::collections::HashSet;
use itertools::Itertools;
use regex::Regex;
use std::str::FromStr;
use std::time::Instant;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct SquareInch {
    x: i32,
    y: i32
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct FabricClaim {
    id: i32,
    top_left: SquareInch,
    top_right: SquareInch,
    bottom_left: SquareInch,
    width: i32,
    height: i32,
}

impl FabricClaim {
    fn new(id: i32, x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            id,
            top_left: SquareInch { x, y },
            top_right: SquareInch { x: x+width, y },
            bottom_left: SquareInch { x, y: y+height },
            width,
            height,
        }
    }

    fn all_sq_inches(&self) -> Vec<SquareInch> {
        let mut sq_inches = Vec::with_capacity(
            self.height as usize * self.width as usize
        );
        for x in self.top_left.x..self.top_right.x {
            for y in self.top_left.y..self.bottom_left.y {
                sq_inches.push( SquareInch { x, y } );
            }
        }
        sq_inches
    }

    fn from_input_line(input_line: &str) -> FabricClaim {
        let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        re.captures(input_line).map(|cap| {
            let id = FromStr::from_str(&cap[1]).unwrap();
            let x = FromStr::from_str(&cap[2]).unwrap();
            let y = FromStr::from_str(&cap[3]).unwrap();
            let width = FromStr::from_str(&cap[4]).unwrap();
            let height = FromStr::from_str(&cap[5]).unwrap();
            FabricClaim::new(id, x, y, width, height)
        }).unwrap()
    }

    fn overlaps(&self, other: &FabricClaim) -> bool {
        if self.top_left.x > other.top_right.x ||
        self.top_right.x < other.top_left.x ||
        self.top_left.y > other.bottom_left.y ||
        self.bottom_left.y < other.top_left.y {
            false
        } else {
            true
        }
    }
}

pub fn day03(input_lines: &[Vec<String>]) -> (String, String) {
    let now = Instant::now();

    let fabric_claims: HashSet<FabricClaim> = input_lines[0]
        .iter()
        .fold(HashSet::new(), |mut map, line| {
            let fabric_claim = FabricClaim::from_input_line(line);
            map.insert(fabric_claim);
            map
        }
    );

    println!("Finished parsing input lines after {}ms.", now.elapsed().as_millis());

    let (_, contested_inches) = fabric_claims
        .iter()
        .fold(
        (HashSet::<SquareInch>::new(), HashSet::<SquareInch>::new()),
            |(mut claimed, mut contested), claim| {
            let square_inches = claim.all_sq_inches();
            for square_inch in square_inches {
                if !claimed.insert(square_inch) {
                    contested.insert(square_inch);
                }
            }
            (claimed, contested)
        }
    );

    let all_claim_ids: HashSet<i32> = HashSet::from_iter(1..fabric_claims.len() as i32 + 1);
    let mut contested_claim_ids = HashSet::<i32>::new();
    for combination in fabric_claims.iter().combinations(2) {
        let this_claim = combination[0];
        let that_claim = combination[1];
        if this_claim.overlaps(that_claim) {
            contested_claim_ids.insert(this_claim.id);
            contested_claim_ids.insert(that_claim.id);
        }
    }

    let answer1 = contested_inches.len();
    let answer2: Vec<&i32> = all_claim_ids.difference(&contested_claim_ids).collect();
    (format!("{}", answer1), format!("{:?}", answer2[0]))
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