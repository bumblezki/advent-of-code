use nalgebra::{Vector2, Matrix2, DMatrix};
use rotate_enum::RotateEnum;

const UP: Vector2<i32> = Vector2::new(0, 1);
const DOWN: Vector2<i32> = Vector2::new(0, -1);
const LEFT: Vector2<i32> = Vector2::new(-1, 0);
const RIGHT: Vector2<i32> = Vector2::new(1, 0);
const TURN_LEFT: Matrix2<i32> = Matrix2::new(0, 1, -1, 0);
const GO_STRAIGHT: Matrix2<i32> = Matrix2::new(1, 0, 0, 1);
const TURN_RIGHT: Matrix2<i32> = Matrix2::new(0, -1, 1, 0);

#[derive(Debug, Clone, Copy, RotateEnum)]
enum IntersectionBehaviour {
    TurnLeft,
    GoStraight,
    TurnRight,
}

impl Into<Matrix2<i32>> for IntersectionBehaviour {
    fn into(self) -> Matrix2<i32> {
        match self {
            IntersectionBehaviour::TurnLeft => TURN_LEFT,
            IntersectionBehaviour::GoStraight => GO_STRAIGHT,
            IntersectionBehaviour::TurnRight => TURN_RIGHT,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Velocity {
    Up,
    Down,
    Left,
    Right,
}

impl Into<Vector2<i32>> for Velocity {
    fn into(self) -> Vector2<i32> {
        match self {
            Velocity::Up => UP,
            Velocity::Down => DOWN,
            Velocity::Left => LEFT,
            Velocity::Right => RIGHT,
        }
    }
}

impl From<Vector2<i32>> for Velocity {
    fn from(value: Vector2<i32>) -> Self {
        match value {
            UP => Velocity::Up,
            DOWN => Velocity::Down,
            LEFT => Velocity::Left,
            RIGHT => Velocity::Right,
            _ => panic!(),
        }
    }
}

struct Cart {
    position: Vector2<i32>,
    velocity: Velocity,
    intersection_behaviour: IntersectionBehaviour,
}

impl Cart {
    fn go(&mut self) {
        self.position += Into::<Vector2<i32>>::into(self.velocity);
    }

    fn handle_intersection(&mut self) {
        self.velocity = Velocity::from(Into::<Matrix2<i32>>::into(self.intersection_behaviour) * Into::<Vector2<i32>>::into(self.velocity));
        self.intersection_behaviour = self.intersection_behaviour.next();
    }

    // fn handle_turn()

        
    //         '\\' => {
    //             match self.velocity {
    //                 Velocity::Up | Velocity::Down => TURN_LEFT,
    //                 Velocity::Right | Velocity::Left => TURN_RIGHT,
    //             }
    //         },
    //         '/' => {
    //             match self.velocity {
    //                 Velocity::Up | Velocity::Down => TURN_RIGHT,
    //                 Velocity::Left | Velocity::Right => TURN_LEFT,
    //             }
    //         },
    //         '-' | '|' => {
    //             GO_STRAIGHT
    //         }
    //         _ => {
    //             panic!()
    //         }
    //     }
    // 
}



pub fn day13(_input_lines: &[Vec<String>]) -> (String, String) {
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
