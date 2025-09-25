use macroquad::prelude::Texture2D;

pub const G_WIDTH: f32 = 1000.0;
pub const G_HEIGHT: f32 = 1000.0;
pub const ROAD_WIDTH: f32 = 75.0;
pub const SAFETY_GAP: f32 = 30.0;
pub const SPAWNING_OFFSET: f32 = 30.0;
pub const VEHICLE_LENGTH_X: f32 = 88.0;
pub const VEHICLE_LENGTH_Y: f32 = 75.0;

// North ↧ (x,y)
pub const G_SPAWN_P_UP: (f32, f32) = (G_WIDTH / 2.0 - ROAD_WIDTH - SPAWNING_OFFSET, 0.0);
// South ↑ (x,y)
pub const G_SPAWN_P_DOWN: (f32, f32) = (G_WIDTH / 2.0 - SPAWNING_OFFSET, G_HEIGHT);
// Left → (x,y)
pub const G_SPAWN_P_LEFT: (f32, f32) = (-50.0, G_HEIGHT / 2.0 - 10.0);
// Right ← (x,y)
pub const G_SPAWN_P_RIGHT: (f32, f32) = (G_WIDTH, G_HEIGHT / 2.0 - ROAD_WIDTH - 10.0);

// Vehicle direction
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

// Route choice
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum Route {
    Left,
    Right,
    Straight,
}

// Traffic light state
#[derive(PartialEq, Eq)]
pub enum Light {
    Red,
    Green,
}

// Vehicle
#[derive(Debug)]
pub struct Vehicle {
    pub x: f32,
    pub y: f32,
    pub direction: Direction,
    pub route: Route,
    pub speed: f32,
    pub frame_index: usize,
    pub frame_timer: f32,
}
pub struct Sprites {
    pub tex_left: Texture2D,
    pub tex_right: Texture2D,
    pub tex_straight: Texture2D,
}
// Traffic light
pub struct TrafficLight {
    pub state: Light,
    pub timer: f32,      // counts time since last switch
    pub green_time: f32, // how long green lasts
    pub red_time: f32,
}

// Road / lane capacity
pub struct Lane {
    pub vehicles: Vec<Vehicle>,
    pub direction: Direction,
    pub start_x: f32,
    pub start_y: f32,
    pub length: f32,     // ?
    pub capacity: usize, // capacity = floor(lane_length / (vehicle_length + safety_gap))
    pub traffic_light: TrafficLight,
}
