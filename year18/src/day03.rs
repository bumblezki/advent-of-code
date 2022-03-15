use std::collections::HashSet;
use std::hash::Hash;
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
    width: i32,
    height: i32,
}

impl FabricClaim {
    fn new(id: i32, x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            id,
            top_left: SquareInch { x, y },
            width,
            height,
        }
    }

    fn right_edge_x(&self) -> i32 {
        self.top_left.x + self.width - 1
    }

    fn bottom_edge_y(&self) -> i32 {
        self.top_left.y + self.height - 1
    }

    fn all_sq_inches(&self) -> Vec<SquareInch> {
        let mut sq_inches = Vec::with_capacity(
            self.height as usize * self.width as usize
        );
        for x in self.top_left.x..self.right_edge_x() + 1 {
            for y in self.top_left.y..self.bottom_edge_y() + 1 {
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
        }).expect("Failed to match regular expression against input.")
    }

    fn overlaps(&self, other: &FabricClaim) -> bool {
        if self.top_left.x > other.right_edge_x() ||
        self.right_edge_x() < other.top_left.x ||
        self.top_left.y > other.bottom_edge_y() ||
        self.bottom_edge_y() < other.top_left.y {
            false
        } else {
            true
        }
    }
}

pub fn day03(input_lines: &[Vec<String>]) -> (String, String) {
    let now = Instant::now();

    let fabric_claims: Vec<FabricClaim> = input_lines[0]
        .iter()
        .map(|line| 
            FabricClaim::from_input_line(line)
        )
        .collect();
    
    println!("Finished parsing input lines after {}ms.", now.elapsed().as_millis());

    let mut claimed_sq_inches = HashSet::<SquareInch>::new();
    let mut contested_sq_inches = HashSet::<SquareInch>::new();
    for claim in &fabric_claims {
        for sq_inch in claim.all_sq_inches() {
            if !claimed_sq_inches.insert(sq_inch) {
                contested_sq_inches.insert(sq_inch);
            }
        }
    }

    let mut uncontested_claim_ids: HashSet<i32> = HashSet::from_iter(1..fabric_claims.len() as i32 + 1);
    for combination in fabric_claims.iter().combinations(2) {
        let this_claim = combination[0];
        let that_claim = combination[1];
        if this_claim.overlaps(that_claim) {
            uncontested_claim_ids.remove(&this_claim.id);
            uncontested_claim_ids.remove(&that_claim.id);
        }
    }

    let answer1 = contested_sq_inches.len();
    let answer2 = uncontested_claim_ids;
    (format!("{}", answer1), format!("{:?}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day03;
    use crate::utils::load_input;

    #[test]
    fn check_day03_case01() {
        full_test(
            "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2", // INPUT STRING
            "4", // PART 1 RESULT
            "{3}" // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(day03(&input_lines), (part1_result.to_string(), part2_result.to_string()));
    }
}