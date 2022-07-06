// Potential improvements:
//
use log::error;
use pixels::{Pixels, SurfaceTexture};
use regex::Regex;
use std::fmt;
use std::num::ParseIntError;
use std::ops::AddAssign;
use std::str::FromStr;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;


const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

#[derive(Clone, Copy, PartialEq)]
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
    fn add_assign(&mut self, other: Vec2d) {
        *self = Vec2d::new(self.x + other.x, self.y + other.y);
    }
}

impl fmt::Display for Vec2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone, Copy)]
struct Star {
    p: Vec2d,
    v: Vec2d,
}

impl Star {
    fn new(p: Vec2d, v: Vec2d) -> Star {
        Star { p, v }
    }

    fn shoot(mut self) {
        self.p += self.v;
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
            Vec2d::new(
                caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            ),
            Vec2d::new(
                caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            ),
        ))
    }
}

struct NightSky {
    stars: Vec<Star>,
    time: u32,
}

impl NightSky {
    fn new(stars: Vec<Star>) -> NightSky {
        NightSky { stars, time: 0 }
    }

    fn update(&mut self) {
        self.stars.iter_mut().for_each(|star| star.shoot());
        self.time += 1;
    }

    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i as i32 % WIDTH as i32;
            let y = i as i32 / WIDTH as i32;

            let p = Vec2d::new(x, y);

            let rgba = if self.stars.iter().any(|star| star.p == p) {
                [0xff, 0xff, 0xff, 0xff]
            } else {
                [0x00, 0x00, 0x5a, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}

pub fn day10(input_lines: &[Vec<String>]) -> (String, String) {
    let stars: Vec<Star> = input_lines[0]
        .iter()
        .map(|line| line.parse::<Star>().unwrap())
        .collect();
    // let north: i32 = stars
    //     .iter()
    //     .max_by(|star1, star2| star1.p.y.cmp(&star2.p.y))
    //     .map(|star| star.p.y)
    //     .unwrap();
    // let south: i32 = stars
    //     .iter()
    //     .min_by(|star1, star2| star1.p.y.cmp(&star2.p.y))
    //     .map(|star| star.p.y)
    //     .unwrap();
    // let east: i32 = stars
    //     .iter()
    //     .min_by(|star1, star2| star1.p.y.cmp(&star2.p.y))
    //     .map(|star| star.p.x)
    //     .unwrap();
    // let west: i32 = stars
    //     .iter()
    //     .max_by(|star1, star2| star1.p.y.cmp(&star2.p.y))
    //     .map(|star| star.p.x)
    //     .unwrap();

    let mut sky = NightSky::new(stars);

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
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
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
    };

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            sky.draw(pixels.get_frame());
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

            // Update internal state and request a redraw
            sky.update();
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
