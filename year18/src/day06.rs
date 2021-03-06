// Change a comment so that this file shows up in the PR

use std::{
    collections::{HashMap, HashSet},
    num::ParseIntError,
    str::FromStr,
};

const MAX_PROXIMITY: i32 = 10000;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// SCC Implementing this so we can just parse::<Point>() is nice; I should use that pattern more often.
impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(", ").collect();

        let x_from_str = coords[0].parse::<i32>()?;
        let y_from_str = coords[1].parse::<i32>()?;

        Ok(Point {
            x: x_from_str,
            y: y_from_str,
        })
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn manhattan_distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn get_closest_destination(&self, destinations: &[Point]) -> Option<Point> {
        // SCC The processing with collects and clones here is slightly overcomplicated. You could do one of the following:
        // 1. Not collect the destination_distances, and keep it as an Iterator.  Then you clone for the first use in shortest_distance, and use directly in the if test later, without having to call [into_]iter() each time.
        // 2. Collect into a Vec of references to the duples, and then just call .iter() when getting the shortest_distance without having to clone() or [into_] it by handling the references instead (some small amount of other & additions and removals, plus a .clone for the return value, required).
        let destination_distances: Vec<(Point, i32)> = destinations
            .iter()
            .map(|dest| (dest.clone(), self.manhattan_distance(dest)))
            .collect();
        let shortest_distance: Option<(Point, i32)> = destination_distances
            .clone()
            .into_iter()
            .min_by(|&(_, d1), &(_, d2)| d1.cmp(&d2));
        match shortest_distance {
            Some((point, d)) => {
                if destination_distances
                    .iter()
                    .filter(|&(_, distance)| distance == &d)
                    .count()
                    > 1
                {
                    None
                } else {
                    Some(point)
                }
            }
            None => None,
        }
    }

    fn get_cumulative_distances(&self, destinations: &[Point]) -> i32 {
        destinations.iter().fold(0, |accumulator, destination| {
            accumulator + self.manhattan_distance(destination)
        })
    }
}

fn get_max_x_and_y(destinations: &[Point]) -> (i32, i32) {
    (
        destinations
            .iter()
            .max_by(|a, b| a.x.cmp(&b.x))
            .expect("Failed to find max X value.")
            .x,
        destinations
            .iter()
            .max_by(|a, b| a.y.cmp(&b.y))
            .expect("Failed to find max Y value.")
            .y,
    )
}

fn get_all_points(max_x: &i32, max_y: &i32) -> Vec<Point> {
    let mut all_points: Vec<Point> = Vec::new();
    for x in 0..=*max_x {
        for y in 0..=*max_y {
            all_points.push(Point::new(x, y))
        }
    }
    all_points
}

fn get_edges(max_x: &i32, max_y: &i32) -> Vec<Point> {
    let mut edges: Vec<Point> = Vec::new();
    for x in 0..=*max_x {
        edges.push(Point::new(x, 0));
        edges.push(Point::new(x, *max_y));
    }
    for y in 0..=*max_y {
        edges.push(Point::new(0, y));
        edges.push(Point::new(*max_x, y));
    }
    // SCC this has added each of the corners twice.  Probably not a concern but something that you'd want to be aware of if you're not already!
    edges
}

// Modelled off https://davidburn.github.io/advent-2018/day6/
pub fn day06(input_lines: &[Vec<String>]) -> (String, String) {
    let destinations: Vec<Point> = input_lines[0]
        .iter()
        .map(|line| line.parse::<Point>().unwrap())
        .collect();
    let (max_x, max_y): (i32, i32) = get_max_x_and_y(&destinations);
    let edges: Vec<Point> = get_edges(&max_x, &max_y);
    // SCC feels a little inefficient to start from 0,0 when your data-set may be a considerably long way away from here.
    // SCC In fact, I also think this would break if any of the co-ordinates were negative but might not do so in an obvious way i.e. by hitting an error; it might just give you the wrong answer.
    let all_points: Vec<Point> = get_all_points(&max_x, &max_y);

    let mut infinite_destinations: HashSet<Point> = HashSet::new();
    let mut destination_map: HashMap<Point, i32> = HashMap::new();
    // SCC This might be cloning a pretty large Vec. We don't actually need the object itself - you could just do `for point in &all_points` and it just works.
    for point in all_points.clone() {
        if let Some(closest_destination) = point.get_closest_destination(&destinations) {
            if edges.contains(&point) {
                infinite_destinations.insert(closest_destination.clone());
            }
            *destination_map.entry(closest_destination).or_insert(0) += 1;
        }
    }

    // SCC This use of the duple has a bit of a code smell to me: we're relying on remembering which bit of the duple is which over and over.  I'd suggest creating another
    // struct of this duple and then we can refer to things with the field names.
    let (_, area): (&Point, &i32) = destination_map
        .iter()
        .filter(|&(dest, _)| !infinite_destinations.contains(dest))
        .max_by(|&(_, &a), &(_, &b)| a.cmp(&b))
        .expect("Failed to find maximum area.");

    let answer1 = area;

    let mut in_region_count = 0;
    for point in all_points {
        if point.get_cumulative_distances(&destinations) < MAX_PROXIMITY {
            in_region_count += 1;
        }
    }

    let answer2 = in_region_count;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day06;
    use crate::utils::load_input;

    #[test]
    fn check_day06_case01() {
        full_test(
            "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9", // INPUT STRING
            "17", // PART 1 RESULT
            "16", // PART 2 RESULT, MUST CHANGE MAX_PROXIMITY CONSTANT TO 32 FOR THIS TO PASS
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
