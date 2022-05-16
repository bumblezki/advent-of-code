// Potential improvements:
//

use std::{str::FromStr, num::ParseIntError, collections::{HashMap, HashSet}};

const MAX_PROXIMITY: i32 = 10000;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

    fn get_closest_destination(&self, destinations: &Vec<Point>) -> Option<Point> {
        let destination_distances: Vec<(Point, i32)> = destinations
            .iter()
            .map(|dest| (dest.clone(), self.manhattan_distance(&dest)))
            .collect();
        let shortest_distance: Option<(Point, i32)> = destination_distances
            .clone()
            .into_iter()
            .min_by(|&(_, d1), &(_, d2)| d1.cmp(&d2));
        match shortest_distance {
            Some((point, d)) => {
                if destination_distances.iter().filter(|&(_, distance)| distance == &d).count() > 1 {
                    return None
                } else {
                    return Some(point)
                }
            }
            None => return None
        }
    }

    fn get_cumulative_distances(&self, destinations: &Vec<Point>) -> i32 {
        destinations.iter().fold(0, |accumulator, destination| accumulator + self.manhattan_distance(destination))
    }
}

fn get_max_x_and_y(destinations: &Vec<Point>) -> (i32, i32) {
    (
        destinations.iter().max_by(|a, b| a.x.cmp(&b.x)).expect("Failed to find max X value.").x,
        destinations.iter().max_by(|a, b| a.y.cmp(&b.y)).expect("Failed to find max Y value.").y
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
    edges
}

// Modelled off https://davidburn.github.io/advent-2018/day6/
pub fn day06(input_lines: &[Vec<String>]) -> (String, String) {
    let destinations: Vec<Point> = input_lines[0].iter().map(|line| line.parse::<Point>().unwrap()).collect();
    let (max_x, max_y): (i32, i32) = get_max_x_and_y(&destinations);
    let edges: Vec<Point> = get_edges(&max_x, &max_y);
    let all_points: Vec<Point> = get_all_points(&max_x, &max_y);

    let mut infinite_destinations: HashSet<Point> = HashSet::new();
    let mut destination_map: HashMap<Point, i32> = HashMap::new();
    for point in all_points.clone() {
        if let Some(closest_destination) = point.get_closest_destination(&destinations) {
            if edges.contains(&point) {
                infinite_destinations.insert(closest_destination.clone());
            }
            *destination_map.entry(closest_destination).or_insert(0) += 1;
        }
    }

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
"16" // PART 2 RESULT, MUST CHANGE MAX_PROXIMITY CONSTANT TO 32 FOR THIS TO PASS
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(day06(&input_lines), (part1_result.to_string(), part2_result.to_string()));
    }
}