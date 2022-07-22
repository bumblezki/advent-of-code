// Potential improvements:
//
use log::error;
use pixels::{Pixels, SurfaceTexture};
use regex::Regex;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use rand::Rng;
use nalgebra::{Vector2, Matrix2};

const BACKGROUND: [u8; 4] = [0, 0, 100, 255];
const FLIP_Y: Matrix2<i32> = Matrix2::new(1,0,0,-1);

#[derive(Clone, Copy)]
struct Star {
    p: Vector2<i32>,
    v: Vector2<i32>,
    colour: [u8; 4],
}

impl Star {
    fn new(p: Vector2<i32>, v: Vector2<i32>) -> Star {
        let colour = [
            rand::thread_rng().gen_range(100..=255),
            rand::thread_rng().gen_range(100..=255),
            rand::thread_rng().gen_range(100..=255),
            255
        ];
        Star { p, v, colour }
    }

    fn update(&mut self, time: i32) {
        self.p += self.v * time;
    }

    fn rewind(&mut self, time: i32) {
        self.p -= self.v * time;
    }
}

impl fmt::Display for Star {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Position: {}, \tVelocity: {}", self.p, self.v)
    }
}

impl FromStr for Star {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re =
            Regex::new(r"position=< ?(-?\d+),  ?(-?\d+)> velocity=< ?(-?\d+),  ?(-?\d+)>").unwrap();
        let caps = re.captures(s).unwrap();

        Ok(Star::new(
            Vector2::new(
                caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            ),
            Vector2::new(
                caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            ),
        ))
    }
}

struct NightSky {
    stars: Vec<Star>,
    time: i32,
}

impl fmt::Display for NightSky {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for j in (0..self.height()).rev() {
            for i in 0..self.width() {
                let p_prime = Vector2::new(
                    i as i32,
                    j as i32
                );
                let p = FLIP_Y * p_prime + self.northeast();
                let mut square = '.';
                for star in &self.stars {
                    if p == star.p {
                        square = '#';
                        break
                    }
                }
                s.push(square);
                s.push(' ')
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl NightSky {
    fn new(stars: Vec<Star>) -> NightSky {
        NightSky { stars, time: 0 }
    }

    fn update(&mut self, time: i32) {
        self.stars.iter_mut().for_each(|star| {
            star.update(time);
        });
        self.time += time;
    }

    fn rewind(&mut self, time: i32) {
        self.stars.iter_mut().for_each(|star| {
            star.rewind(time);
        });
        self.time -= time;
    }

    fn north(&self) -> i32 {
        self.stars
            .iter()
            .map(|star| star.p.y)
            .max()
            .unwrap()
    }

    fn east(&self) -> i32 {
        self.stars
            .iter()
            .map(|star| star.p.x)
            .min()
            .unwrap()
    }

    fn south(&self) -> i32 {
        self.stars
            .iter()
            .map(|star| star.p.y)
            .min()
            .unwrap()
    }

    fn west(&self) -> i32 {
        self.stars
            .iter()
            .map(|star| star.p.x)
            .max()
            .unwrap()
    }

    fn height(&self) -> u32 {
        self.north().abs_diff(self.south()) + 1
    }

    fn width(&self) -> u32 {
        self.east().abs_diff(self.west()) + 1
    }

    fn northeast(&self) -> Vector2<i32> {
        Vector2::new(self.east(), self.north())
    }

    fn draw(&self, frame: &mut [u8], height: u32, width: u32, northeast: Vector2<i32>) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let p_prime = Vector2::new(
                i as i32 % width as i32,
                height as i32 - 1 - i as i32 / width as i32
            );

            let p: Vector2<i32> = FLIP_Y * p_prime + northeast;

            let mut rgba = BACKGROUND;
            for star in &self.stars {
                if star.p == p {
                    rgba = star.colour;
                    break
                }
            }

            pixel.copy_from_slice(&rgba);
        }
    }
}

pub fn day10(input_lines: &[Vec<String>]) -> (String, String) {
    let stars: Vec<Star> = input_lines[0]
        .iter()
        .map(|line| line.parse::<Star>().unwrap())
        .collect();

    let mut sky = NightSky::new(stars);

    // Minimize the height of the sky.
    let mut height = sky.height();
    while sky.height() <= height {
        height = sky.height();
        sky.update(1);
    }
    sky.rewind(1);

    let height = sky.height();
    let width = sky.width();
    let northeast = sky.northeast();

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(width as f64, height as f64);
        WindowBuilder::new()
            .with_title("Day 10")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(width, height, surface_texture).unwrap()
    };

    let mut go = false;
    let mut forward = true;
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            sky.draw(pixels.get_frame(), height, width, northeast);
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            if input.key_pressed(VirtualKeyCode::Up) {
                sky.update(1);
                println!("{}", sky.time);
            }

            if input.key_held(VirtualKeyCode::Right) {
                sky.update(1);
            }

            if input.key_pressed(VirtualKeyCode::Down) {
                sky.rewind(1);
                println!("{}", sky.time);
            }

            if input.key_held(VirtualKeyCode::Left) {
                sky.rewind(1);
            }

            if input.key_pressed(VirtualKeyCode::Space) {
                go = !go
            }

            if input.key_pressed(VirtualKeyCode::Tab) {
                forward = !forward
            }

            if go {
                if forward {
                    sky.update(1);
                } else {
                    sky.rewind(1);
                }
            }
            window.request_redraw();
        }
    });

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
            "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>", // INPUT STRING
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
