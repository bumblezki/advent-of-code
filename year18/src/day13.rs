use itertools::Itertools;
use nalgebra::{Vector2, Matrix2, DMatrix, RowDVector};
use rotate_enum::RotateEnum;

const UP: Vector2<i32> = Vector2::new(-1, 0);
const DOWN: Vector2<i32> = Vector2::new(1, 0);
const LEFT: Vector2<i32> = Vector2::new(0, -1);
const RIGHT: Vector2<i32> = Vector2::new(0, 1);
const TURN_LEFT: Matrix2<i32> = Matrix2::new(0, 1, -1, 0);
const GO_STRAIGHT: Matrix2<i32> = Matrix2::new(1, 0, 0, 1);
const TURN_RIGHT: Matrix2<i32> = Matrix2::new(0, -1, 1, 0);

#[derive(Clone, Copy, RotateEnum)]
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

    fn go(&mut self, track_map: &DMatrix<char>) {
        self.position += self.velocity;
        let track_piece: char = track_map[(self.position.x as usize, self.position.y as usize)];
        self.handle_track_piece(track_piece);
    }

    fn handle_track_piece(&mut self, track_piece: char) {
        let turning_matrix = match track_piece {
            '+' => self.get_update_intersection_behavior(),
            '\\' => {
                // Going left/right (true) or up/down (false)?
                match self.velocity.y.abs() == 1 {
                    true => TURN_RIGHT,
                    false => TURN_LEFT,
                }
            },
            '/' => {
                // Going left/right (true) or up/down (false)?
                match self.velocity.y.abs() == 1 {
                    true => TURN_LEFT,
                    false => TURN_RIGHT,
                }
            },
            '-' | '|' => {
                GO_STRAIGHT
            }
            _ => {
                GO_STRAIGHT
                // panic!("{}", track_piece)
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

    // Find all the carts and replace their starting location on the map with either '|' or '-'.
    // This assumes that no carts start at intersections or corners.
    // Take account of the padding when instantiating the carts as we'll soon remove this.
    let mut carts: Vec<Cart> = Vec::new();
    for jj in 0..track_map.nrows() {
        for ii in 0..track_map.ncols() {
            match track_map[(ii, jj)] {
                '>' => {
                    carts.push(Cart::new(ii as i32 - 1, jj as i32 - 1, RIGHT));
                    track_map[(ii, jj)] = '-';
                },
                '<' =>  {
                    carts.push(Cart::new(ii as i32 - 1, jj as i32 - 1, LEFT));
                    track_map[(ii, jj)] = '-';
                },
                '^' =>  {
                    carts.push(Cart::new(ii as i32 - 1, jj as i32 - 1, UP));
                    track_map[(ii, jj)] = '|';
                },
                'v' =>  {
                    carts.push(Cart::new(ii as i32 - 1, jj as i32 - 1, DOWN));
                    track_map[(ii, jj)] = '|';
                },
                _ => continue,
            }
        }
    }
    let ncols = track_map.ncols();
    let nrows = track_map.nrows();
    let track_map = track_map.remove_columns_at(&[0, ncols]);
    let track_map = track_map.remove_rows_at(&[0, nrows]);

    loop {
        let mut drawn_track_map = track_map.clone();
        for cart in &carts {
            drawn_track_map[(cart.position.x as usize, cart.position.y as usize)] = 'O';
        }
        println!("{}", drawn_track_map);
        for cart in carts.iter_mut() {
            cart.go(&track_map)
        }
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
