pub const G_WIDTH: f32 = 1000.0;
pub const G_HEIGHT: f32 = 1000.0;
pub const ROAD_WIDTH: f32 = 75.0;
pub const INTERSECTION_SIZE: f32 = 150.0;

// Vehicle direction
pub enum Direction {
    North,
    South,
    East,
    West,
}

// Route choice
pub enum Route {
    Left,
    Right,
    Straight,
}

// Traffic light state
pub enum Light {
    Red,
    Green,
}

// Vehicle
pub struct Vehicle {
    x: f32,
    y: f32,
    direction: Direction,
    route: Route,
    speed: f32,
    frame_index: usize,
    frame_timer: f32,
}

// Traffic light
pub struct TrafficLight {
    direction: Direction,
    state: Light,
    timer: f32,      // counts time since last switch
    green_time: f32, // how long green lasts
}

// Road / lane capacity
pub struct Lane {
    direction: Direction,
    start_x: f32,
    start_y: f32,
    length: f32,
    vehicles: Vec<Vehicle>,
    capacity: usize, // capacity = floor(lane_length / (vehicle_length + safety_gap))
}
