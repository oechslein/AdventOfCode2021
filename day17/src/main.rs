#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![feature(generators, generator_trait)]
#![feature(test)]
#![feature(drain_filter)]
#![feature(const_option)]
#![feature(type_alias_impl_trait)]

extern crate test;

use itertools::Itertools;

use std::error;

mod utils;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() -> Result<(), Box<dyn error::Error>> {
    utils::with_measure("Part 1", || solve_part1("test.txt"));
    utils::with_measure("Part 2", || solve_part2("test.txt"));
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////

//#[derive(Debug, PartialEq, Eq, FromPrimitive)]
type NumberType = i32;
type CoorType = (NumberType, NumberType);
type PathType = Vec<(CoorType, NumberType, NumberType)>;

fn solve_part1(_: &str) -> Result<NumberType, String> {
    //let (target_area_start, target_area_end) = ((20, -5), (30, -10));
    let (target_area_start, target_area_end) = ((102, -90), (157, -146));

    if let Some(all_possible_paths) = solve(target_area_start, target_area_end) {
        let max_highest_y = all_possible_paths
            .iter()
            .map(|possible_path| possible_path.iter().map(|((_, y), _, _)| y).max().unwrap())
            .max()
            .unwrap();
        Ok(*max_highest_y)
    } else {
        Err("Nothing found".to_string())
    }
}

fn solve_part2(_: &str) -> Result<usize, String> {
    let (target_area_start, target_area_end) = ((20, -5), (30, -10));
    //let (target_area_start, target_area_end) = ((102, -90), (157, -146));

    if let Some(mut all_possible_speeds) = solve(target_area_start, target_area_end) {
        let all_possible_speeds_len = all_possible_speeds.len();
        all_possible_speeds.sort_by_key(|path| path.iter().map(|((x,y),x_speed,y_speed)| (*y, *x_speed+*y_speed)).max().unwrap());
        visualize(all_possible_speeds, target_area_start, target_area_end);
        Ok(all_possible_speeds_len)
    } else {
        Err("Nothing found".to_string())
    }
}

fn solve(target_area_start: CoorType, target_area_end: CoorType) -> Option<Vec<PathType>> {
    let mut max_highest_y = NumberType::MIN;
    let mut all_possible_paths = Vec::new();
    // otherwise it would jump over the target area
    let y_speed_max_heuristic = (target_area_start.1 as NumberType)
        .abs()
        .max((target_area_end.1 as NumberType).abs());
    for (x_speed, y_speed) in (1..target_area_start.0 + target_area_end.0)
        .cartesian_product(-y_speed_max_heuristic..y_speed_max_heuristic)
    {
        let result = calculate_forward_path(x_speed, y_speed, target_area_start, target_area_end);
        //println!("({} {}) => {:?}", x_speed, y_speed, result);
        if let Some((highest_y, path)) = result {
            all_possible_paths.push(path);
            max_highest_y = max_highest_y.max(highest_y);
        }
    }

    if all_possible_paths.is_empty() {
        None
    } else {
        Some(all_possible_paths)
    }
}

fn calculate_forward_path(
    x_speed: NumberType,
    y_speed: NumberType,
    target_area_start: CoorType,
    target_area_end: CoorType,
) -> Option<(NumberType, PathType)> {
    let mut x = 0;
    let mut y = 0;
    let mut x_speed = x_speed;
    let mut y_speed = y_speed;
    let mut highest_y = y;
    //println!("x,y: {},{}; xspeed,yspeed: {},{}", x, y, x_speed, y_speed);
    let mut path = vec![((x, y), x_speed, y_speed)];

    loop {
        x += x_speed;
        y += y_speed;
        x_speed = match x_speed.cmp(&0) {
            std::cmp::Ordering::Less => x_speed + 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => x_speed - 1,
        };
        y_speed -= 1;
        highest_y = highest_y.max(y);
        //println!("x,y: {},{}; xspeed,yspeed: {},{}", x, y, x_speed, y_speed);
        path.push(((x, y), x_speed, y_speed));

        // y axis is inverted
        if x > target_area_end.0 || y < target_area_end.1 {
            return None;
        }
        if x >= target_area_start.0 && y <= target_area_start.1 {
            assert!(x <= target_area_end.0 && y >= target_area_end.1);
            return Some((highest_y, path));
        }
        // since y_speed get lower and lower, y get lower and lower and will become lower than target_area_end.1
        // so this loop will always end
    }
}

/////////////////////////////////////////////////////////////////////////////////////

extern crate find_folder;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston_window::*;

fn visualize(
    all_possible_pathes: Vec<PathType>,
    target_area_start: CoorType,
    target_area_end: CoorType,
) {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: PistonWindow = WindowSettings::new("probe shots", [800, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let glyph_cache =
        opengl_graphics::GlyphCache::new("assets/FiraSans-Regular.ttf", (), TextureSettings::new())
            .unwrap();

    // Create a new game and run it.
    let mut app = App::new(
        GlGraphics::new(opengl),
        glyph_cache,
        all_possible_pathes,
        target_area_start,
        target_area_end,
    );

    let mut settings = EventSettings::new();
    settings.max_fps *= 5;
    settings.ups *= 5;
    let mut events = Events::new(settings);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            //println!("Pressed keyboard key '{:?}'", key);
            if key == Key::Space {
                app._paused = !app._paused;
            }
        };

    }
}

pub struct App<'a> {
    gl: GlGraphics, // OpenGL drawing backend.
    glyph_cache: opengl_graphics::GlyphCache<'a>,
    all_possible_pathes: Vec<PathType>,
    target_area_start: CoorType,
    target_area_end: CoorType,
    _paused: bool,
    _path_index: usize,
    _index: usize,
    _fractal_index: f64,
    _min_x: f64,
    _min_y: f64,
    _max_x: f64,
    _max_y: f64,
}

impl<'a> App<'_> {
    fn new(
        gl: GlGraphics,
        glyphs: opengl_graphics::GlyphCache,
        all_possible_pathes: Vec<PathType>,
        target_area_start: CoorType,
        target_area_end: CoorType,
    ) -> App {
        let min_x = all_possible_pathes
            .iter()
            .map(|path| path.iter().map(|((x, _), _, _)| *x).min().unwrap())
            .min()
            .unwrap() as f64;
        let min_y = all_possible_pathes
            .iter()
            .map(|path| path.iter().map(|((_, y), _, _)| *y).min().unwrap())
            .min()
            .unwrap() as f64;
        let max_x = all_possible_pathes
            .iter()
            .map(|path| path.iter().map(|((x, _), _, _)| *x).max().unwrap())
            .max()
            .unwrap() as f64;
        let max_y = all_possible_pathes
            .iter()
            .map(|path| path.iter().map(|((_, y), _, _)| *y).max().unwrap())
            .max()
            .unwrap() as f64;
        App {
            gl, // OpenGL drawing backend.
            glyph_cache: glyphs,
            all_possible_pathes,
            target_area_start,
            target_area_end,
            _paused: true,
            _path_index: 0,
            _index: 0,
            _fractal_index: 0.0,
            _min_x: min_x,
            _min_y: min_y,
            _max_x: max_x,
            _max_y: max_y,
        }
    }

    fn map_coordinates(
        &self,
        coor: (f64, f64),
        window_start: f64,
        window_end: f64,
        window_width: f64,
        window_height: f64,
    ) -> (f64, f64) {
        let min_x = self._min_x;
        let min_y = self._min_y;
        let max_x = self._max_x;
        let max_y = self._max_y;

        let (x, y) = coor;

        // (max_x, max_y) => (0,0)
        // (min_x, min_y) => (window_width, window_height)
        // max_x => 0 , min_x => window_width
        // f(max_x) = 0 , f(min_x) = window_width
        // x1=max_x, x2=min_x, y1=0, y2=window_width

        // m_x = (y2-y1)/(x2-x1) = (window_width-0)/(min_x-max_x)
        // f(x) = m_x * (x-x1) + y1 = m_x * (x-max_x) + 0
        // f(x) = (window_width-0)/(min_x-max_x) * (x-max_x) + 0

        // (min_x, max_y) => (0,0)
        // (max_x, min_y) => (window_width, window_height)
        // min_x => 0 , max_x => window_width
        // f(min_x) = 0 , f(max_x) = window_width
        // x1=min_x, x2=max_x, y1=0, y2=window_width

        // m_x = (y2-y1)/(x2-x1) = (window_width-0)/(max_x-min_x)
        // f(x) = m_x * (x-x1) + y1 = m_x * (x-min_x) + 0
        // f(x) = (window_width-0)/(max_x-min_x) * (x-min_x) + 0

        (
            window_start + (window_width / (max_x - min_x) * (x - min_x)),
            window_end + (window_height / (min_y - max_y) * (y - max_y)),
        )
    }

    fn get_current_path(&self) -> &PathType {
        self.all_possible_pathes.get(self._path_index).unwrap()
    }

    fn get_current_path_coors(&self) -> (f64, f64) {
        let ((path_x, path_y), _, _) = self.get_current_path().get(self._index).unwrap();
        if let Some(((new_path_x, new_path_y), _, _)) = self.get_current_path().get(self._index + 1)
        {
            (
                *path_x as f64 + ((*new_path_x - *path_x) as f64 * self._fractal_index),
                *path_y as f64 + ((*new_path_y - *path_y) as f64 * self._fractal_index),
            )
        } else {
            (*path_x as f64, *path_y as f64)
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let rect_size = 10.0;
        let border = 30.0;

        let square = rectangle::square(-rect_size, -rect_size, rect_size);
        let (win_x, win_y) = self.map_coordinates(
            self.get_current_path_coors(),
            rect_size + border,
            rect_size + border,
            args.window_size[0] - 2.0 * (rect_size + border),
            args.window_size[1] - 2.0 * (rect_size + border),
        );
        let square_zero = rectangle::square(-rect_size, -rect_size, rect_size);
        let (win_zero_x, win_zero_y) = self.map_coordinates(
            (0.0, 0.0),
            rect_size + border,
            rect_size + border,
            args.window_size[0] - 2.0 * (rect_size + border),
            args.window_size[1] - 2.0 * (rect_size + border),
        );

        let (win_target_area_start_x, win_target_area_start_y) = self.map_coordinates(
            (
                self.target_area_start.0 as f64,
                self.target_area_start.1 as f64,
            ),
            rect_size + border,
            rect_size + border,
            args.window_size[0] - 2.0 * (rect_size + border),
            args.window_size[1] - 2.0 * (rect_size + border),
        );

        let (win_target_area_end_x, win_target_area_end_y) = self.map_coordinates(
            (self.target_area_end.0 as f64, self.target_area_end.1 as f64),
            rect_size + border,
            rect_size + border,
            args.window_size[0] - 2.0 * (rect_size + border),
            args.window_size[1] - 2.0 * (rect_size + border),
        );
        let target_area_square = rectangle::rectangle_by_corners(
            win_target_area_start_x,
            win_target_area_start_y,
            win_target_area_end_x,
            win_target_area_end_y,
        );

        let ((path_x, path_y), _, _) = self
            .all_possible_pathes
            .get(self._path_index)
            .unwrap()
            .get(self._index)
            .unwrap();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            rectangle(RED, target_area_square, c.transform, gl);

            rectangle(RED, square_zero, c.transform.trans(win_zero_x, win_zero_y), gl);
            rectangle(GREEN, square, c.transform.trans(win_x, win_y), gl);

            text::Text::new_color([0.0, 0.5, 0.0, 1.0], 16)
                .draw(
                    format!(
                        "{} of {} different trajectories, pos ({}, {})",
                        self._path_index + 1,
                        self.all_possible_pathes.len(),
                        path_x,
                        path_y,
                    )
                    .as_str(),
                    &mut self.glyph_cache,
                    &DrawState::default(),
                    c.transform.trans(10.0, border),
                    gl,
                )
                .unwrap();
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        if self._paused {
            return;
        }
        self._fractal_index += 1.0 * args.dt * 1.0;
        if self._fractal_index >= 1.0 {
            self._fractal_index -= 1.0;
            self._index += 1;
            if self._index >= self.get_current_path().len() {
                self._index = 0;
                self._path_index += 1;
                if self._path_index >= self.all_possible_pathes.len() {
                    self._path_index = 0;
                }
            }
        }
        // Rotate 2 radians per second.
        //self.rotation += 2.0 * args.dt;
    }
}

////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test0() -> Result<(), Box<dyn error::Error>> {
        Ok(())
    }

    #[test]
    fn test1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part1("test.txt").unwrap(), 10585);
        Ok(())
    }

    #[test]
    fn verify1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part1("input.txt").unwrap(), 10585);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part2("test.txt").unwrap(), 5247);
        Ok(())
    }

    #[test]
    fn verify2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part2("input.txt").unwrap(), 5247);
        Ok(())
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| solve_part1("input.txt"));
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| solve_part2("input.txt"));
    }
}
