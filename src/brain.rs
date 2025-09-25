use crate::types::*;
use macroquad::prelude::{KeyCode, is_key_pressed};
use rand::random_range;
use std::collections::HashMap;

fn next_direction(entry: Direction, route: Route) -> Direction {
    match (entry, route) {
        (Direction::North, Route::Straight) => Direction::South,
        (Direction::North, Route::Left) => Direction::East,
        (Direction::North, Route::Right) => Direction::West,

        (Direction::South, Route::Straight) => Direction::North,
        (Direction::South, Route::Left) => Direction::West,
        (Direction::South, Route::Right) => Direction::East,

        (Direction::East, Route::Straight) => Direction::West,
        (Direction::East, Route::Left) => Direction::North,
        (Direction::East, Route::Right) => Direction::South,

        (Direction::West, Route::Straight) => Direction::East,
        (Direction::West, Route::Left) => Direction::South,
        (Direction::West, Route::Right) => Direction::North,
    }
}

pub fn handle_input(lanes: &mut HashMap<Direction, Lane>) {
    if is_key_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }

    spawn_from_key(lanes, KeyCode::Up, Direction::North); // Up = from South
    spawn_from_key(lanes, KeyCode::Down, Direction::South); // Down = from North
    spawn_from_key(lanes, KeyCode::Right, Direction::West); // Right = from West
    spawn_from_key(lanes, KeyCode::Left, Direction::East); // Left = from East
}
fn spawn_from_key(lanes: &mut HashMap<Direction, Lane>, key: KeyCode, spawn_direction: Direction) {
    if is_key_pressed(key) {
        if let Some(lane) = lanes.get_mut(&spawn_direction) {
            if lane.vehicles.len() < lane.capacity {
                if let Some(car) = lane.spawn_vehicle_safe() {
                    lane.vehicles.push(car);
                }
            }
        }
    }
}

impl Lane {
    pub fn new(direction: Direction) -> Self {
        // Determine spawn coordinates and lane length based on direction
        let (start_x, start_y, lane_length, vehicle_length) = match direction {
            Direction::South => (
                G_SPAWN_P_UP.0,
                G_SPAWN_P_UP.1,
                G_HEIGHT / 2.0 - ROAD_WIDTH,
                VEHICLE_LENGTH_Y,
            ),
            Direction::North => (
                G_SPAWN_P_DOWN.0,
                G_SPAWN_P_DOWN.1,
                G_HEIGHT / 2.0 - ROAD_WIDTH,
                VEHICLE_LENGTH_Y,
            ),
            Direction::West => (
                G_SPAWN_P_LEFT.0,
                G_SPAWN_P_LEFT.1,
                G_WIDTH / 2.0 - ROAD_WIDTH,
                VEHICLE_LENGTH_X,
            ),
            Direction::East => (
                G_SPAWN_P_RIGHT.0,
                G_SPAWN_P_RIGHT.1,
                G_WIDTH / 2.0 - ROAD_WIDTH,
                VEHICLE_LENGTH_X,
            ),
        };
        // Calculate max number of vehicles that can safely fit

        Self {
            direction,
            start_x,
            start_y,
            length: lane_length,
            vehicles: Vec::new(),
            capacity: 20,
            light_status: None,
        }
    }
    pub fn update_light(&mut self, light: TrafficLight) {
        self.capacity = if light.state == Light::Green {
            20
        } else {
            let lane_length = match self.direction {
                Direction::East | Direction::West => G_WIDTH / 2.0 - ROAD_WIDTH,
                Direction::North | Direction::South => G_HEIGHT / 2.0 - ROAD_WIDTH,
            };

            // Compute how many vehicles can fit with SAFETY_GAP and VEHICLE_LENGTH
            let vehicle_length = match self.direction {
                Direction::East | Direction::West => VEHICLE_LENGTH_X,
                Direction::North | Direction::South => VEHICLE_LENGTH_Y,
            };

            ((lane_length) / (vehicle_length + SAFETY_GAP)).floor() as usize
        };
    }
    /// Spawn a vehicle only if there is space at the start of the lane
    pub fn spawn_vehicle_safe(&mut self) -> Option<Vehicle> {
        // If lane is empty, always safe
        if self.vehicles.is_empty() {
            return Some(self.spawn_vehicle());
        }

        // Get the first vehicle in the lane (closest to spawn)
        let first_car = &self.vehicles[0];

        // Check distance based on lane direction
        let safe_to_spawn = match self.direction {
            Direction::North => first_car.y > VEHICLE_LENGTH_Y + SAFETY_GAP,
            Direction::South => first_car.y < G_HEIGHT - VEHICLE_LENGTH_Y - SAFETY_GAP,
            Direction::East => first_car.x > VEHICLE_LENGTH_X + SAFETY_GAP,
            Direction::West => first_car.x < G_WIDTH - VEHICLE_LENGTH_X - SAFETY_GAP,
        };

        if safe_to_spawn {
            Some(self.spawn_vehicle())
        } else {
            None
        }
    }

    pub fn spawn_vehicle(&mut self) -> Vehicle {
        Vehicle {
            x: self.start_x,
            y: self.start_y,
            direction: self.direction.clone(),
            route: get_random_route(),
            speed: 100.0,
            frame_index: 0,
            frame_timer: 0.0,
        }
    }

    pub fn create_all_lanes() -> HashMap<Direction, Lane> {
        let mut lanes = HashMap::new();

        let directions = [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ];

        for &dir in &directions {
            lanes.insert(dir, Lane::new(dir));
        }

        lanes
    }
}

pub fn get_random_route() -> Route {
    let options: [Route; 3] = [Route::Straight, Route::Left, Route::Right];
    options[random_range(0..3)]
}

pub fn update_draw_lanes(lanes: &mut HashMap<Direction, Lane>, dt: f32, sprites: &Sprites) {
    for lane in lanes.values_mut() {
        for vehicle in lane.vehicles.iter_mut() {
            vehicle.update(dt);
            vehicle.draw(sprites);
        }
    }
}
pub fn remove_out_of_bounds_vehicles(lanes: &mut HashMap<Direction, Lane>) {
    for lane in lanes.values_mut() {
        lane.vehicles.retain(|v| v.is_in_bounds());
    }
}
