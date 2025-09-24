use macroquad::prelude::*;

mod background;
mod cars;
mod traffic_light;

use background::*;
use cars::*;
use traffic_light::*;

const VEHICLE_LENGTH: f32 = 60.0;
const SAFETY_GAP: f32 = 15.0; // Safety gap between vehicles

fn window_conf() -> Conf {
    Conf {
        window_title: "Road Intersection".to_owned(),
        window_width: 900,
        window_height: 700,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let bg = Scene::new(screen_width(), screen_height());
    
    // Four traffic lights are created, one for each incoming lane.
    // N/S lights start RED, E/W lights start GREEN.
    let mut lights = vec![
        TrafficLight::new(bg.window_width / 2.0 + 61.0, bg.window_height / 2.0 + 60.0, false), // For UP traffic
        TrafficLight::new(bg.window_width / 2.0 - 100.0, bg.window_height / 2.0 - 100.0, false), // For DOWN traffic
        TrafficLight::new(bg.window_width / 2.0 - 100.0, bg.window_height / 2.0 + 61.0, true), // For RIGHT traffic
        TrafficLight::new(bg.window_width / 2.0 + 61.0, bg.window_height / 2.0 - 100.0, true), // For LEFT traffic
    ];

    let mut cars: Vec<Cars> = vec![];
    let mut last_light_switch = get_time();
    let light_duration = 5.0; // Each light phase (N-S or E-W) lasts 5 seconds.

    loop {
        // --- Vehicle Spawning based on keyboard input ---
        if is_key_pressed(KeyCode::Up) {
            try_spawn_car(&mut cars, Direction::UP, &bg);
        }
        if is_key_pressed(KeyCode::Down) {
            try_spawn_car(&mut cars, Direction::DOWN, &bg);
        }
        if is_key_pressed(KeyCode::Right) {
            try_spawn_car(&mut cars, Direction::RIGHT, &bg);
        }
        if is_key_pressed(KeyCode::Left) {
            try_spawn_car(&mut cars, Direction::LEFT, &bg);
        }
        if is_key_pressed(KeyCode::R) {
            let random_dir = match rand::gen_range(0, 4) {
                0 => Direction::UP,
                1 => Direction::DOWN,
                2 => Direction::LEFT,
                _ => Direction::RIGHT,
            };
            try_spawn_car(&mut cars, random_dir, &bg);
        }
        if is_key_pressed(KeyCode::Escape) {
            break;
        }


        // --- Traffic Light Algorithm ---
        // A simple fixed timer switches the lights between N-S green and E-W green.
        if get_time() - last_light_switch > light_duration {
            for light in &mut lights {
                light.change_light();
            }
            last_light_switch = get_time();
        }


        // --- Car Movement & Collision Algorithm ---
        for i in 0..cars.len() {
            let mut can_move = true;
            let car_rect = Rect::new(cars[i].position.0, cars[i].position.1, VEHICLE_LENGTH, VEHICLE_LENGTH);
            
            // A "sensor" rectangle is placed slightly in front of the car to detect obstacles.
            let mut sensor_rect = car_rect;

            // 1. Check for Red Light at stop lines
            match cars[i].original_direction {
                Direction::UP => {
                    let stop_line = bg.window_height / 2.0 + 60.0;
                    if !lights[0].state && car_rect.top() <= stop_line && car_rect.top() > stop_line - 10.0 {
                        can_move = false;
                    }
                    sensor_rect.y -= 5.0; // Sensor is in front
                }
                Direction::DOWN => {
                    let stop_line = bg.window_height / 2.0 - 60.0;
                    if !lights[1].state && car_rect.bottom() >= stop_line && car_rect.bottom() < stop_line + 10.0 {
                        can_move = false;
                    }
                    sensor_rect.y += 5.0; // Sensor is in front
                }
                Direction::RIGHT => {
                    let stop_line = bg.window_width / 2.0 - 60.0;
                    if !lights[2].state && car_rect.right() >= stop_line && car_rect.right() < stop_line + 10.0 {
                        can_move = false;
                    }
                    sensor_rect.x += 5.0; // Sensor is in front
                }
                Direction::LEFT => {
                    let stop_line = bg.window_width / 2.0 + 60.0;
                    if !lights[3].state && car_rect.left() <= stop_line && car_rect.left() > stop_line - 10.0 {
                        can_move = false;
                    }
                    sensor_rect.x -= 5.0; // Sensor is in front
                }
            }

            // 2. Check for other cars using the sensor
            for j in 0..cars.len() {
                if i == j { continue; }
                let other_car_rect = Rect::new(cars[j].position.0, cars[j].position.1, VEHICLE_LENGTH, VEHICLE_LENGTH);
                if sensor_rect.overlaps(&other_car_rect) {
                    can_move = false;
                    break;
                }
            }

            // 3. Move the car if the path is clear
            if can_move {
                cars[i].update(&bg);
            }
        }
        
        // Remove cars that have driven off-screen to keep the simulation efficient
        cars.retain(|car| {
            car.position.0 > -VEHICLE_LENGTH - 10.0 && car.position.0 < bg.window_width + VEHICLE_LENGTH + 10.0 &&
            car.position.1 > -VEHICLE_LENGTH - 10.0 && car.position.1 < bg.window_height + VEHICLE_LENGTH + 10.0
        });

        // --- Drawing ---
        clear_background(DARKGRAY);
        bg.draw();

        for car in &cars {
            car.draw();
        }

        for light in &lights {
            light.draw();
        }

        next_frame().await
    }
}

// Spawns a car only if there is a safe distance from the last car in that lane.
fn try_spawn_car(cars: &mut Vec<Cars>, direction: Direction, bg: &Scene) {
    let (spawn_pos, spawn_distance_check) = match direction {
        Direction::UP => ((bg.window_width / 2.0 + 5.0, bg.window_height), bg.window_height - (VEHICLE_LENGTH + SAFETY_GAP)),
        Direction::DOWN => ((bg.window_width / 2.0 - 55.0, -VEHICLE_LENGTH), VEHICLE_LENGTH + SAFETY_GAP),
        Direction::RIGHT => ((-VEHICLE_LENGTH, bg.window_height / 2.0 + 5.0), VEHICLE_LENGTH + SAFETY_GAP),
        Direction::LEFT => ((bg.window_width, bg.window_height / 2.0 - 55.0), bg.window_width - (VEHICLE_LENGTH + SAFETY_GAP)),
    };

    let can_spawn = cars
        .iter()
        .filter(|c| c.original_direction == direction)
        .last()
        .map_or(true, |last_car| match direction {
            Direction::UP => last_car.position.1 < spawn_distance_check,
            Direction::DOWN => last_car.position.1 > spawn_distance_check,
            Direction::RIGHT => last_car.position.0 > spawn_distance_check,
            Direction::LEFT => last_car.position.0 < spawn_distance_check,
        });

    if can_spawn {
        cars.push(Cars::new(spawn_pos, direction, get_random_destination()));
    }
}