// Potential improvements:
//

use std::{str::FromStr, num::ParseIntError};

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(", ").collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point { x: x_fromstr, y: y_fromstr })
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn manhattan_distance(&self, other: &Self) -> i32 {
        return (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn get_shortest_manhattan_distance(&self, others: &Vec<Point>) -> Option<(Point, i32)> {
        let shortest_distances: Vec<(Point, i32)> =  others
            .clone()
            .iter()
            .map(|other| (other.clone(), self.manhattan_distance(&other)))
            .collect();
        let shortest_distance: Option<(Point, i32)> = shortest_distances
            .clone()
            .into_iter()
            .min_by(|&(_, d1), &(_, d2)| d1.cmp(&d2));
        match shortest_distance {
            Some((point, d)) => {
                if shortest_distances.iter().filter(|&(_, distance)| distance == &d).count() > 1 {
                    return None
                } else {
                    return Some((point.clone(), d))
                }
            }
            None => return None
        }
    }
}

// Modelled off https://davidburn.github.io/advent-2018/day6/
pub fn day06(input_lines: &[Vec<String>]) -> (String, String) {
    let points: Vec<Point> = input_lines[0].iter().map(|line| line.parse::<Point>().unwrap()).collect();
    println!("{:?}", points);

    let answer1 = 0;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day06;
    use crate::utils::load_input;

    #[test]
    fn check_day06_case01() {
        full_test(
"", // INPUT STRING
"0", // PART 1 RESULT
"0" // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(day06(&input_lines), (part1_result.to_string(), part2_result.to_string()));
    }
}