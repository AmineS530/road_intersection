// use rand::distr::StandardUniform;
// use rand::prelude::{ Distribution, Rng };
use ::rand::prelude::Distribution;
use ::rand::distr::StandardUniform;
use ::rand::rng;
use ::rand::Rng;
use ::rand::distr;
// use rand::*;

use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub enum ColorDir {
    Yellow,
    Blue,
    Pink,
}

impl Distribution<ColorDir> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ColorDir {
        match rng.random_range(0..3) {
            1 => ColorDir::Yellow,
            2 => ColorDir::Pink,
            _ => ColorDir::Blue,
        }
    }
}

#[derive(Debug, Clone)]
pub enum DirectionLane {
    South,
    North,
    West,
    East,
}

impl Distribution<DirectionLane> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DirectionLane {
        match rng.random_range(0..4) {
            1 => DirectionLane::South,
            2 => DirectionLane::North,
            3 => DirectionLane::West,
            _ => DirectionLane::East,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Point(pub f32, pub f32);

#[derive(Debug, Clone)]
pub struct Lane {
    pub direction: DirectionLane,
    pub start_position: Point,
    pub end_position: Point,
    pub vehicles: Vec<Vehicle>,
    // pub length: f64, // depends on the direction vertical or horizontal
}

impl Lane {
    pub fn new(direction: DirectionLane, start_position: Point, end_position: Point) -> Self {
        Self {
            direction: direction,
            start_position: start_position,
            end_position: end_position,
            vehicles: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Route {
    Straight,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Vehicle {
    pub x: f32,
    pub y: f32,
    pub color: ColorDir,
    pub color_rgb: Color,
    pub direction_lane: DirectionLane,
    pub route: Route,
    direction: (i32, i32),
}

impl Vehicle {
    pub fn new(lane: &Lane) -> Self {
        let mut rng = rng();
        let random_color: ColorDir = rng.sample(StandardUniform);
        let direction_lane = &lane.direction;
        let direction = match direction_lane {
            DirectionLane::South => (0, -1),
            DirectionLane::West => (-1, 0),
            DirectionLane::East => (1, 0),
            DirectionLane::North => (0, 1),
        };
        let color_rgb = Self::get_color_rgb(random_color.clone());
        Self {
            x: lane.start_position.0,
            y: lane.start_position.1,
            color: random_color.clone(),
            route: Self::get_route(random_color.clone()),
            direction_lane: direction_lane.clone(),
            color_rgb: color_rgb,
            direction : direction
        }
    }

    pub fn get_color_rgb(color: ColorDir) -> Color {
        match color {
            ColorDir::Yellow => YELLOW,
            ColorDir::Pink => PINK,
            ColorDir::Blue => BLUE,
        }
    }

    pub fn get_route(color: ColorDir) -> Route {
        match color {
            ColorDir::Yellow => Route::Right,
            ColorDir::Pink => Route::Left,
            ColorDir::Blue => Route::Straight,
        }
    }

    pub fn draw_vehicle(&self) {
        draw_rectangle(self.x, self.y, 45.0, 45.0, self.color_rgb);
    }

    pub fn update(&mut self) {
        self.x = self.x + (self.direction.0 as f32) * 1.0;
        self.y = self.y + (self.direction.1 as f32) * 1.0;
    }

    fn change_direction(&mut self) {}
}

pub fn random_direction_lane() -> DirectionLane {
    let random_direction_lane: DirectionLane = rng().sample(StandardUniform);
    random_direction_lane
}
