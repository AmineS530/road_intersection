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
                let car = lane.spawn_vehicle() ;
                    lane.vehicles.push(car);
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

        // Directly create a traffic light
        let traffic_light = TrafficLight {
            state: Light::Red,
            timer: 0.0,
            green_time: 5.0,
            red_time: 5.0,
        };

        Self {
            direction,
            start_x,
            start_y,
            length: lane_length,
            vehicles: Vec::new(),
            capacity: 20,
            traffic_light,
        }
    }
    pub fn update_capacity_from_light(&mut self) {
        self.capacity = if self.traffic_light.is_green() {
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
    pub fn safe_speed(&self, idx: usize) -> f32 {
        let vehicle = &self.vehicles[idx];

        // Check the car in front
        if idx == 0 {
            // first car in lane → no one in front
            return vehicle.speed;
        }

        let front = &self.vehicles[idx - 1];
        let gap = match vehicle.direction {
            Direction::South => front.y - vehicle.y - VEHICLE_LENGTH_Y,
            Direction::North => vehicle.y - front.y - VEHICLE_LENGTH_Y,
            Direction::East => vehicle.x - front.x - VEHICLE_LENGTH_X,
            Direction::West => front.x - vehicle.x - VEHICLE_LENGTH_X,
        };

        // If gap < SAFETY_GAP → stop, otherwise move at normal speed
        if gap < SAFETY_GAP { 0.0 } else { vehicle.speed }
    }
}

pub fn get_random_route() -> Route {
    let options: [Route; 3] = [Route::Straight, Route::Left, Route::Right];
    options[random_range(0..3)]
}
pub fn update_draw_lanes(lanes: &mut HashMap<Direction, Lane>, dt: f32, sprites: &Sprites) {
    for lane in lanes.values_mut() {
        for i in 0..lane.vehicles.len() {
            let allowed_speed = lane.safe_speed(i);

            // Also stop for red light
            let vehicle = &mut lane.vehicles[i];
            let stop_distance = 50.0;
            let intersection_pos = G_WIDTH / 2.0;

            let light_stop = match lane.direction {
                Direction::North => {
                    !lane.traffic_light.is_green() && vehicle.y <= intersection_pos + stop_distance
                }
                Direction::South => {
                    !lane.traffic_light.is_green()
                        && vehicle.y >= G_HEIGHT - intersection_pos - stop_distance
                }
                Direction::East => {
                    !lane.traffic_light.is_green() && vehicle.x <= intersection_pos + stop_distance
                }
                Direction::West => {
                    !lane.traffic_light.is_green()
                        && vehicle.x >= G_WIDTH - intersection_pos - stop_distance
                }
            };

            let final_speed = if light_stop { 0.0 } else { allowed_speed };
            vehicle.update_with_speed(dt, final_speed);
            vehicle.draw(sprites);
        }
    }
}

pub fn remove_out_of_bounds_vehicles(lanes: &mut HashMap<Direction, Lane>) {
    for lane in lanes.values_mut() {
        lane.vehicles.retain(|v| v.is_in_bounds());
    }
}
