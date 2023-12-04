// Potential improvements:
//

use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;
use std::fmt::Display;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum CubeColour {
    Red,
    Green,
    Blue,
}

impl Display for CubeColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            CubeColour::Red => "red",
            CubeColour::Green => "green",
            CubeColour::Blue => "blue",
        })
    }
}

struct CubeGame {
    id: u32,
    colour_maxes: HashMap<CubeColour, u32>,
}

impl FromStr for CubeGame {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse a string like "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        let mut id_and_game = s.split(":");
        let id = id_and_game
            .next().unwrap()
            .split(" ").skip(1).next().unwrap()
            .parse::<u32>().unwrap();

        let rounds = id_and_game
            .next().unwrap()
            .split(";").collect::<Vec<&str>>();

        // Find the max count of each colour across all rounds
        let mut colour_maxes: HashMap<CubeColour, u32> = HashMap::from([
            (CubeColour::Red, 0),
            (CubeColour::Green, 0),
            (CubeColour::Blue, 0),
        ]);
        for round in rounds {
            let mut cube_sample: HashMap<CubeColour, u32> = HashMap::new();
            for cube in round.trim().split(", ") {
                let mut count_and_colour = cube.split(" ");
                let count = count_and_colour.next().unwrap().trim().parse::<u32>().unwrap();
                let colour = match count_and_colour.next().unwrap().trim() {
                    "red" => CubeColour::Red,
                    "green" => CubeColour::Green,
                    "blue" => CubeColour::Blue,
                    _ => panic!("Unknown colour"),
                };
                cube_sample.insert(colour, count);
                if &count > colour_maxes.get(&colour).unwrap() {
                    colour_maxes.insert(colour, count);
                }
            }
        }
        Ok(CubeGame {
            id,
            colour_maxes,
        })
    }
}

impl CubeGame {
    fn is_possible_with(&self, red_total: u32, green_total: u32, blue_total: u32) -> bool {
        red_total >= *self.colour_maxes.get(&CubeColour::Red).unwrap()
        && green_total >= *self.colour_maxes.get(&CubeColour::Green).unwrap()
        && blue_total >= *self.colour_maxes.get(&CubeColour::Blue).unwrap()
    }
}

pub fn day02(input_lines: &[Vec<String>]) -> (String, String) {
    
    let cube_games = input_lines[0].iter().map(|line| {
        CubeGame::from_str(line).unwrap()
    }).collect::<Vec<CubeGame>>();

    let answer1: u32 = cube_games.iter()
        .filter(|game| 
            game.is_possible_with(12, 13, 14)
        )
        .map(|game| game.id)
        .sum();

    let answer2: u32 = cube_games.iter()
        .map(|game| game.colour_maxes.values().product::<u32>())
        .sum();

    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day02;
    use crate::utils::load_input;

    #[test]
    fn check_day02_case01() {
        full_test(
"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", // INPUT STRING
"8", // PART 1 RESULT
"2286" // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(day02(&input_lines), (part1_result.to_string(), part2_result.to_string()));
    }
}