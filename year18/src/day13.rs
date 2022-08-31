use itertools::Itertools;
use nalgebra::{Vector2, Matrix2, DMatrix, RowDVector};
use rotate_enum::RotateEnum;

// const UP: Vector2<i32> = Vector2::new(0, 1);
// const DOWN: Vector2<i32> = Vector2::new(0, -1);
// const LEFT: Vector2<i32> = Vector2::new(-1, 0);
// const RIGHT: Vector2<i32> = Vector2::new(1, 0);
const TURN_LEFT: Matrix2<i32> = Matrix2::new(0, 1, -1, 0);
const GO_STRAIGHT: Matrix2<i32> = Matrix2::new(1, 0, 0, 1);
const TURN_RIGHT: Matrix2<i32> = Matrix2::new(0, -1, 1, 0);

#[derive(Debug, Clone, Copy, RotateEnum)]
enum IntersectionBehavior {
    TurnLeft,
    GoStraight,
    TurnRight,
}

impl Into<Matrix2<i32>> for IntersectionBehavior {
    fn into(self) -> Matrix2<i32> {
        match self {
            IntersectionBehavior::TurnLeft => TURN_LEFT,
            IntersectionBehavior::GoStraight => GO_STRAIGHT,
            IntersectionBehavior::TurnRight => TURN_RIGHT,
        }
    }
}

struct Cart {
    position: Vector2<i32>,
    velocity: Vector2<i32>,
    intersection_behavior: IntersectionBehavior,
}

impl Cart {
    fn go(&mut self, track_map: DMatrix<char>) {
        self.position += self.velocity;
        // let track_piece: char = track_map[Into::<(i32, i32)>::into(self.position)]; 
    }

    fn handle_track_piece(&mut self, track_piece: char) {
        let turning_matrix = match track_piece {
            '+' => self.get_update_intersection_behavior(),
            '\\' => {
                match (self.velocity.y.abs() == 1, self.velocity.x.abs() == 0) {
                    (true, false) => TURN_LEFT,
                    (false, true) => TURN_RIGHT,
                    _ => panic!()
                }
            },
            '/' => {
                match (self.velocity.x.abs() == 1, self.velocity.y.abs() == 0) {
                    (true, false) => TURN_LEFT,
                    (false, true) => TURN_RIGHT,
                    _ => panic!()
                }
            },
            '-' | '|' => {
                GO_STRAIGHT
            }
            _ => {
                panic!()
            }
        };
        self.velocity = turning_matrix * self.velocity;
    }

    fn get_update_intersection_behavior(&mut self) -> Matrix2<i32> {
        self.intersection_behavior = self.intersection_behavior.next();
        Into::<Matrix2<i32>>::into(self.intersection_behavior)
    }
}



pub fn day13(input_lines: &[Vec<String>]) -> (String, String) {
    
    let mut track_map: DMatrix<char> = DMatrix::from_rows(
        &input_lines[0]
            .iter()
            .map(|line| RowDVector::from_vec(line.chars().collect_vec()))
            .collect_vec()
    );
    

    let answer1 = 0;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[cfg(test)]
mod tests {
    use super::day13;
    use crate::utils::load_input;

    #[test]
    fn check_day13_case01() {
        full_test(
            "",  // INPUT STRING
            "0", // PART 1 RESULT
            "0", // PART 2 RESULT
        )
    }

    fn full_test(input_text: &str, part1_result: &str, part2_result: &str) {
        let input_lines = load_input(input_text);
        assert_eq!(
            day13(&input_lines),
            (part1_result.to_string(), part2_result.to_string())
        );
    }
}
