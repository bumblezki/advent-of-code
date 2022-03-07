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
            // SCC There's a subtlty here: the top_right and bottom_left squares aren't included in the fabric claim, while top_left is.  This works out with the all_sq_inches() implementation because of how for iterators are bounded but leads to your overlap() function being wrong.
            // SCC We're also storing the same state multiple times.  I'd recommend making top_right and bottom_left (or slight variants on them) functions of the FabricClaim rather than saved values, if you even need to use them at all.
            top_right: SquareInch { x: x+width, y },
            bottom_left: SquareInch { x, y: y+height },
            width,
            height,
        }
    }

    fn all_sq_inches(&self) -> Vec<SquareInch> {
        // SCC ::with_capacity() is a neat touch that I fail to make use of, nice!
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
        // SCC This seems like a solid way of getting the values from the string (and probably better than my method of using repeated .split()s!)  I've heard of people using Nom but I haven't looked into that, but if you're finding regex matching slow that might be something to look into.
        re.captures(input_line).map(|cap| {
            let id = FromStr::from_str(&cap[1]).unwrap();
            let x = FromStr::from_str(&cap[2]).unwrap();
            let y = FromStr::from_str(&cap[3]).unwrap();
            let width = FromStr::from_str(&cap[4]).unwrap();
            let height = FromStr::from_str(&cap[5]).unwrap();
            FabricClaim::new(id, x, y, width, height)
        }).unwrap()
        // SCC Personal preference: I like to use .expect("Error message") in my AoC code where I know it will work because of the input/problem statement but it's not enforced by the code, and only use .unwrap() if something about the code itself guarantees the unwrap is safe. I'd recommend it partly for debugging purposes and partly to make sure you're thinking "why am I sure this is safe" whenever you do unwrap something.
        // In this case, it's dependent on the unput matching your expectations, so I'd go with "expect".  In the first line of this function, it's guaranteed to work so long as it's worked once and Regex::new() doesn't change under your feet, so unwrap is probably fine.
    }

    fn overlaps(&self, other: &FabricClaim) -> bool {
        // SCC This looks bugged - it'll return false positives when one claim starts immediately after the end of the other one (if self.top_left.x == other.top_right.x, the two pieces don't overlap, since top_right isn't in the claim).
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

    // SCC Why are you storing this as a HashSet?  There are two concerns with doing this:
    // 1. (Hypothetical) You'll drop duplicate claims, but the puzzle doesn't say anything about that: if you had two identical claims, you should consider them to fully overlap and you'd want to know you had two.  This doesn't apply here because each claim has a unique ID, but it's worth bearing in mind.
    // 2. (Actual) HashSets are more expensive to add and remove entries from than Vectors, but give you the benefit of being easier to find if a given element is in it or not (and guaranteeing no duplicate entries).  You're not using those beneifts here, so I think you'd be strictly better off with a Vector.
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
    // SCC This might be personal preference, but I think using a .fold() to insert values into mutable HashSets feels a bit awkward.  .fold() is great when you're building up something that's naturally an accumulator but for this I'd just declare the maps you're building up front (as mut) and then do a `for claim in claims` loop.
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

    // SCC This could be more efficient by starting from the all_claim_ids and calling .remove() during the loop, rather than building a second set and calling .difference().
    // SCC Additionally, if you were keeping this code like this, I'd recommend moving this line down to just before calculation answer2, since we don't use it until then.
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
    // SCC It'd be worth asserting that answer2.len() == 1 here, since the puzzle has claimed this and if that doesn't hold, you've got a bug - which you might catch before entering a wrong answer.
    (format!("{}", answer1), format!("{:?}", answer2[0]))
}

#[cfg(test)]
mod tests {
    use super::day03;
    use crate::utils::load_input;

    #[test]
    // SCC Ah, you haven't filed in the UT framework I provided!  This would have caught the overlap() bug above if you'd used the example 3 lines from the puzzle page :)
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
