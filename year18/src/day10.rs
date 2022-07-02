// Potential improvements:
//
use std::str::FromStr;
use std::num::ParseIntError;
use regex::Regex;
use std::fmt;
use std::ops::AddAssign;

#[derive(Clone, Copy)]
struct Vec2d {
    x: i32,
    y: i32,
}

impl Vec2d {
    fn new(x: i32, y: i32) -> Vec2d {
        Vec2d { x, y }
    }
}

impl AddAssign for Vec2d {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl fmt::Display for Vec2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone, Copy)]
struct Star {
    p: Vec2d,
    v: Vec2d,
}

impl Star {
    fn shoot(mut self) {
        self.p += self.v;
    }
}

impl fmt::Display for Star {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "Position: {}, \tVelocity: {}", self.p, self.v)
    }
}

impl FromStr for Star {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"position=< ?(-?\d+),  ?(-?\d+)> velocity=< ?(-?\d+),  ?(-?\d+)>").unwrap();
        let caps = re.captures(s).unwrap();

        Ok(Star { p: Vec2d { x: caps.get(1).unwrap().as_str().parse::<i32>().unwrap(), y: caps.get(2).unwrap().as_str().parse::<i32>().unwrap() }, v: Vec2d { x: caps.get(3).unwrap().as_str().parse::<i32>().unwrap(), y: caps.get(4).unwrap().as_str().parse::<i32>().unwrap() } })
    }
}

struct NightSky {
    stars: Vec<Star>,
    northwest: Vec2d,
    southeast: Vec2d,
    time: i32,
}

impl NightSky {
    fn new(stars: Vec<Star>, northwest: Vec2d, southeast: Vec2d) -> NightSky {
        NightSky { stars, northwest, southeast, time: 0 }
    }

    fn update(&mut self) {
        self.stars.iter_mut().for_each(|star| star.shoot());
        self.time += 1;
    }
}


pub fn day10(input_lines: &[Vec<String>]) -> (String, String) {
    let stars: Vec<Star> = input_lines[0].iter().map(|line| line.parse::<Star>().unwrap()).collect();
    let north: i32 = stars.iter().max_by(|star1, star2| star1.p.y.cmp(&star2.p.y)).map(|star| star.p.y).unwrap();
    let south: i32 = stars.iter().min_by(|star1, star2| star1.p.y.cmp(&star2.p.y)).map(|star| star.p.y).unwrap();
    let east: i32 = stars.iter().min_by(|star1, star2| star1.p.y.cmp(&star2.p.y)).map(|star| star.p.x).unwrap();
    let west: i32 = stars.iter().max_by(|star1, star2| star1.p.y.cmp(&star2.p.y)).map(|star| star.p.x).unwrap();

    let mut sky = NightSky::new(stars, Vec2d::new(west, north), Vec2d::new(east, south));
    
    while sky.time <= 100 {
        sky.update();
    }

    let answer1 = 0;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day10;
    use crate::utils::load_input;

    #[test]
    fn check_day10_case01() {
        full_test(
            "",  // INPUT STRING
            "0", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day10(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
