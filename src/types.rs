pub const G_WIDTH: f32 = 1000.0;
pub const G_HEIGHT: f32 = 1000.0;
pub const ROAD_WIDTH: f32 = 75.0;
pub const INTERSECTION_SIZE: f32 = 75.0;

// Vehicle direction
enum Direction {
    North,
    South,
    East,
    West,
}

// Route choice
enum Route {
    Left,
    Right,
    Straight,
}

// Traffic light state
enum Light {
    Red,
    Green,
}

// Vehicle
struct Vehicle {
    x: f32,
    y: f32,
    direction: Direction,
    route: Route,
    speed: f32,
    frame_index: usize,
    frame_timer: f32,
}

// Traffic light
struct TrafficLight {
    direction: Direction,
    state: Light,
    timer: f32,      // counts time since last switch
    green_time: f32, // how long green lasts
}

// Road / lane capacity
struct Lane {
    direction: Direction,
    vehicles: Vec<Vehicle>,
    capacity: usize, // capacity = floor(lane_length / (vehicle_length + safety_gap))
}
