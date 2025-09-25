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
    pub intersection_point: Point,
    // pub length: f64, // depends on the direction vertical or horizontal
}

impl Lane {
    pub fn new(
        direction: DirectionLane,
        start_position: Point,
        end_position: Point,
        width: f32,
        height: f32
    ) -> Self {
        let intersection_pt = match direction {
            DirectionLane::North => Point(width / 2.0 - 50.0, height / 2.0 - 50.0),
            DirectionLane::South => Point(width / 2.0, height / 2.0),
            DirectionLane::West => Point(width / 2.0, height / 2.0 - 50.0),
            DirectionLane::East => Point(width / 2.0 - 50.0, height / 2.0),
        };

        Self {
            direction: direction,
            start_position: start_position,
            end_position: end_position,
            vehicles: Vec::new(),
            intersection_point: intersection_pt,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    lane: Lane,
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
            direction: direction,
            lane: lane.clone(),
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
        if self.should_change_direction() {
            self.change_pixels();
            self.change_direction();
        }

        self.x += (self.direction.0 as f32) * 1.0;
        self.y += (self.direction.1 as f32) * 1.0;
    }

    fn should_change_direction(&self) -> bool {
        self.x == self.lane.intersection_point.0 && self.y == self.lane.intersection_point.1
    }

    fn change_direction(&mut self) {
        self.direction = match (self.direction, &self.route) {
            ((0, 1), Route::Left) => (1, 0),
            ((0, 1), Route::Right) => (-1, 0),
            ((0, 1), Route::Straight) => (0, 1),

            ((0, -1), Route::Left) => (-1, 0),
            ((0, -1), Route::Right) => (1, 0),
            ((0, -1), Route::Straight) => (0, -1),

            ((1, 0), Route::Left) => (0, -1),
            ((1, 0), Route::Right) => (0, 1),
            ((1, 0), Route::Straight) => (1, 0),

            ((-1, 0), Route::Left) => (0, 1),
            ((-1, 0), Route::Right) => (0, -1),
            ((-1, 0), Route::Straight) => (-1, 0),
            (dir, _) => dir,
        };
    }

    fn change_pixels(&mut self) {
        if self.route == Route::Left {
            match self.lane.direction {
                DirectionLane::North => {
                    self.y += 50.0;
                }
                DirectionLane::South => {
                    self.y -= 50.0;
                }
                DirectionLane::East => {
                    self.x += 50.0;
                }
                DirectionLane::West => {
                    self.x -= 50.0;
                }
            };
        }
    }
}

pub fn random_direction_lane() -> DirectionLane {
    let random_direction_lane: DirectionLane = rng().sample(StandardUniform);
    random_direction_lane
}

#[derive(Debug, Clone, PartialEq)]
pub enum LightState {
    Red,
    Green,
}

#[derive(Debug, Clone)]
pub struct TrafficLight {
    pub position: Point,
    pub state: LightState,
    pub timer: f32,
    pub green_duration: f32,
    pub red_duration: f32,
}

impl TrafficLight {
    pub fn new(position: Point, green_duration: f32, red_duration: f32) -> Self {
        Self {
            position,
            state: LightState::Red,
            timer: 0.0,
            green_duration,
            red_duration,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.timer += dt;
        match self.state {
            LightState::Green if self.timer >= self.green_duration => {
                self.state = LightState::Red;
                self.timer = 0.0;
            }
            LightState::Red if self.timer >= self.red_duration => {
                self.state = LightState::Green;
                self.timer = 0.0;
            }
            _ => {}
        }
    }

    pub fn is_green(&self) -> bool {
        self.state == LightState::Green
    }

    pub fn draw(&self) {
        let color = if self.state == LightState::Green { GREEN } else { RED };
        draw_rectangle(self.position.0, self.position.1, 45.0, 45.0, color);
    }
}
