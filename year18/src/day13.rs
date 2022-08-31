use itertools::Itertools;
use nalgebra::{Vector2, Matrix2, DMatrix, RowDVector};
use rotate_enum::RotateEnum;

const UP: Vector2<i32> = Vector2::new(0, 1);
const DOWN: Vector2<i32> = Vector2::new(0, -1);
const LEFT: Vector2<i32> = Vector2::new(-1, 0);
const RIGHT: Vector2<i32> = Vector2::new(1, 0);
const TURN_LEFT: Matrix2<i32> = Matrix2::new(0, 1, -1, 0);
const GO_STRAIGHT: Matrix2<i32> = Matrix2::new(1, 0, 0, 1);
const TURN_RIGHT: Matrix2<i32> = Matrix2::new(0, -1, 1, 0);

#[derive(Debug, Clone, Copy, RotateEnum)]
enum IntersectionBehavior {
    TurnLeft,
    GoStraight,
    TurnRight,
}

impl std::fmt::Display for IntersectionBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            IntersectionBehavior::TurnLeft => 'L',
            IntersectionBehavior::GoStraight => '|',
            IntersectionBehavior::TurnRight => 'R',
        };
        write!(f, "{}", c)
    }
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

impl std::fmt::Display for Cart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arrow = match (self.velocity.x, self.velocity.y) {
            (1, 0) => '>',
            (0, 1) => '^',
            (-1, 0) => '<',
            (0, -1) => 'v',
            _ => panic!(),
        };
        write!(f, "{} {} : ({}, {})", arrow, self.intersection_behavior, self.position.x, self.position.y)
    }
}

impl Cart {
    fn new(x: i32, y: i32, velocity: Vector2<i32>) -> Self {
        Self {
            position: Vector2::new(x, y),
            velocity, 
            intersection_behavior: IntersectionBehavior::TurnLeft,
        }
    }

    fn go(&mut self, track_map: DMatrix<char>) {
        self.position += self.velocity;
        let track_piece: char = track_map[(self.position.y as usize, self.position.x as usize)];
        self.handle_track_piece(track_piece);
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
    
    let track_map: DMatrix<char> = DMatrix::from_rows(
        &input_lines[0]
            .iter()
            .map(|line| RowDVector::from_vec(line.chars().collect_vec()))
            .collect_vec()
    );
    // Add padding of one row around the edge so that we can take 3x3 slices and see the full surroundings of each cart.
    let nrows = track_map.nrows();
    let ncols = track_map.ncols();
    let track_map = track_map.insert_row(nrows, ' ');
    let track_map = track_map.insert_column( ncols, ' ');
    let track_map = track_map.insert_row(0, ' ');
    let mut track_map = track_map.insert_column(0, ' ');

    // Find all the carts
    let cart_surrounding_size = 3;
    let mut carts: Vec<Cart> = Vec::new();
    for jj in 0..track_map.nrows()-cart_surrounding_size {
        for ii in 0..track_map.ncols()-cart_surrounding_size {
            let slice = track_map.slice_mut((jj, ii), (cart_surrounding_size, cart_surrounding_size));
            match slice[(1, 1)] {
                '>' => carts.push(Cart::new(ii as i32, jj as i32, RIGHT)),
                '<' => carts.push(Cart::new(ii as i32, jj as i32, LEFT)),
                '^' => carts.push(Cart::new(ii as i32, jj as i32, UP)),
                'v' => carts.push(Cart::new(ii as i32, jj as i32, DOWN)),
                _ => continue,
            }
        }
    }

    for cart in carts {
        println!("{}", cart);
    }

    
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
