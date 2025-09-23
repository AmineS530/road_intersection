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
    velocity: f32,
    color: (u8, u8, u8),
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
    capacity: usize,
}
