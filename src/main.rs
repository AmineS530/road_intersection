// use macroquad::prelude::*;

// mod types;
// use types::*;

// fn window_conf() -> Conf {
//     Conf {
//         window_title: "Road Intersection".to_string(),
//         window_width: G_WIDTH as i32,
//         window_height: G_HEIGHT as i32,
//         window_resizable: false,
//         fullscreen: false,
//         ..Default::default()
//     }
// }

// #[macroquad::main(window_conf)]
// async fn main() {
//     let grass: Texture2D = load_texture("assets/background.png").await.unwrap();
//     grass.set_filter(FilterMode::Nearest);
//     loop {
//         clear_background(BLACK);
//         draw_texture_ex(
//             &grass,
//             0.0,
//             0.0,
//             WHITE,
//             DrawTextureParams {
//                 dest_size: Some(vec2(screen_width(), screen_height())),
//                 ..Default::default()
//             },
//         );
//         // North : Up
//         draw_rectangle(
//             G_HEIGHT / 2.0, // x
//             0.0,            // y
//             ROAD_WIDTH,     // width
//             G_HEIGHT,       // width
//             BLACK,
//         );
//         // South : Down
//         draw_rectangle(
//             G_HEIGHT / 2.0 - ROAD_WIDTH, // x
//             0.0,                         // y
//             ROAD_WIDTH,                  // width
//             G_HEIGHT,                    // width
//             DARKGRAY,
//         );
//         // East : Left
//         draw_rectangle(
//             0.0,                        // y
//             G_WIDTH / 2.0 - ROAD_WIDTH, // x
//             G_WIDTH,                    // width
//             ROAD_WIDTH,                 // width
//             DARKGRAY,
//         );
//         // West : Right
//         draw_rectangle(
//             0.0,           // y
//             G_WIDTH / 2.0, // x
//             G_WIDTH,       // width
//             ROAD_WIDTH,    // width
//             BLACK,
//         );

//         draw_horizontal_dashed_line(
//             G_HEIGHT / 2.0 - 5.0, // y position
//             0.0,                  // start x
//             screen_width(),       // end x
//             30.0,                 // dash length
//             25.0,                 // gap
//             6.0,                  // thickness
//             WHITE,
//         );

//         // Vertical dashed line
//         draw_vertical_dashed_line(
//             G_WIDTH / 2.0 - 5.0, // x position
//             0.0,                 // start y
//             screen_height(),     // end y
//             30.0,                // dash length
//             25.0,                // gap
//             6.0,                 // thickness
//             WHITE,
//         );
//         // finish frame
//         next_frame().await
//     }
// }

// fn draw_horizontal_dashed_line(
//     y: f32,
//     x_start: f32,
//     x_end: f32,
//     dash_len: f32,
//     gap: f32,
//     thickness: f32,
//     color: Color,
// ) {
//     let mut x = x_start;
//     let inter_left = G_WIDTH / 2.0 - INTERSECTION_SIZE / 2.0;
//     let inter_right = G_WIDTH / 2.0 + INTERSECTION_SIZE / 2.0;

//     while x < x_end {
//         let dash_right = x + dash_len;

//         // Only draw if the dash is fully outside the intersection zone
//         if dash_right < inter_left || x > inter_right {
//             draw_rectangle(x, y, dash_len, thickness, color);
//         }

//         x += dash_len + gap;
//     }
// }

// fn draw_vertical_dashed_line(
//     x: f32,
//     y_start: f32,
//     y_end: f32,
//     dash_len: f32,
//     gap: f32,
//     thickness: f32,
//     color: Color,
// ) {
//     let mut y = y_start;
//     let inter_top = G_HEIGHT / 2.0 - INTERSECTION_SIZE;
//     let inter_bottom = G_HEIGHT / 2.0 + INTERSECTION_SIZE;

//     while y < y_end {
//         let dash_bottom = y + dash_len;

//         // Only draw if the dash is fully outside the intersection zone
//         if dash_bottom < inter_top || y > inter_bottom {
//             draw_rectangle(x, y, thickness, dash_len, color);
//         }

//         y += dash_len + gap;
//     }
// }
extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{Duration, Instant}; 

mod background;
mod cars;
mod traffic_light;
use background::*;
use cars::*;

use crate::traffic_light::TrafficLight;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let bg = Scene::new(900, 700);
    let window = video_subsystem
        .window(
            "HIT THE ROAD JACK",
            bg.window_width as u32,
            bg.window_height as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut lights: Vec<TrafficLight> = Vec::new();
    lights.push(TrafficLight::new(bg.window_width / 2 + 61, bg.window_height / 2 + 61, false)); 
    lights.push(TrafficLight::new(bg.window_width / 2 - 100, bg.window_height / 2 - 100, false));
    lights.push(TrafficLight::new(bg.window_width / 2 - 100, bg.window_height / 2 + 61, true)); 
    lights.push(TrafficLight::new(bg.window_width / 2 + 61, bg.window_height / 2 - 100, true)); 

    let mut cars: Vec<Cars> = vec![];
    
    // --- NEW: Timer for traffic lights ---
    let mut last_light_change = Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::UP),
                    repeat: false,
                    ..
                } => {
                    try_spawn_car(
                        &mut cars,
                        Direction::UP,
                        (bg.window_width / 2 + 5, bg.window_height),
                        80,
                        &bg,
                    );
                }
                Event::KeyDown {
                    keycode: Some(Keycode::DOWN),
                    repeat: false,
                    ..
                } => {
                    try_spawn_car(
                        &mut cars,
                        Direction::DOWN,
                        (bg.window_width / 2 - 55, -60),
                        20,
                        &bg,
                    );
                }
                Event::KeyDown {
                    keycode: Some(Keycode::RIGHT),
                    repeat: false,
                    ..
                } => {
                    try_spawn_car(
                        &mut cars,
                        Direction::RIGHT,
                        (-60, bg.window_height / 2 + 5),
                        20,
                        &bg,
                    );
                }
                Event::KeyDown {
                    keycode: Some(Keycode::LEFT),
                    repeat: false,
                    ..
                } => {
                    try_spawn_car(
                        &mut cars,
                        Direction::LEFT,
                        (bg.window_width, bg.window_height / 2 - 55),
                        80,
                        &bg,
                    );
                }

                _ => {}
            }
        }

        if last_light_change.elapsed().as_secs() >= 5 {
            for light in lights.iter_mut() {
                light.change_light();
            }
            last_light_change = Instant::now();
        }

        canvas.clear();
        bg.draw(&mut canvas);

        for car in &mut cars {
            car.car_draw(&mut canvas, car.distination.colorize());
        }

        // --- EDITED: Car Movement and Stopping Logic ---
        for i in 0..cars.len() {
            let mut can_move = true;
            let car_rect = Rect::new(cars[i].position.0, cars[i].position.1, 60, 60);
            let mut sensor_rect = car_rect;
            
            match cars[i].direction {
                Direction::UP => {
                    // Stop line is at the bottom of the intersection
                    let stop_line = bg.window_height / 2 + 80;
                    if !lights[0].state && car_rect.top() <= stop_line && car_rect.top() > stop_line - 10 {
                        can_move = false;
                    }
                    sensor_rect.set_y(car_rect.y() - 5);
                }
                Direction::DOWN => {
                    // Stop line is at the top of the intersection
                    let stop_line = bg.window_height / 2 - 80;
                    if !lights[1].state && car_rect.bottom() >= stop_line && car_rect.bottom() < stop_line + 10 {
                        can_move = false;
                    }
                    sensor_rect.set_y(car_rect.y() + 5);
                }
                Direction::RIGHT => {
                    // Stop line is at the left of the intersection
                    let stop_line = bg.window_width / 2 - 80;
                    if !lights[2].state && car_rect.right() >= stop_line && car_rect.right() < stop_line + 10 {
                        can_move = false;
                    }
                    sensor_rect.set_x(car_rect.x() + 5);
                }
                Direction::LEFT => {
                    // Stop line is at the right of the intersection
                    let stop_line = bg.window_width / 2 + 80;
                    if !lights[3].state && car_rect.left() <= stop_line && car_rect.left() > stop_line - 10 {
                        can_move = false;
                    }
                    sensor_rect.set_x(car_rect.x() - 5);
                }
            }

            // 2. Check for other cars (Safe Distance)
            for j in 0..cars.len() {
                if i == j { continue; }
                let other_car_rect = Rect::new(cars[j].position.0, cars[j].position.1, 60, 60);
                if sensor_rect.has_intersection(other_car_rect) {
                    can_move = false;
                    break;
                }
            }

            // Move the car if it's allowed to
            if can_move {
                match cars[i].direction {
                    Direction::LEFT => cars[i].position.0 -= 3,
                    Direction::RIGHT => cars[i].position.0 += 3,
                    Direction::UP => cars[i].position.1 -= 3,
                    Direction::DOWN => cars[i].position.1 += 3,
                }
            }
        }
        
        for light in lights.iter() {
            light.draw_light(&mut canvas);
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    }
}

fn try_spawn_car(
    cars: &mut Vec<Cars>,
    direction: Direction,
    spawn_position: (i32, i32),
    spawn_distance: i32,
    bg: &Scene,
) {
    let can_spawn = cars
        .iter()
        .rev()
        .find(|c| c.direction == direction)
        .map_or(true, |last_car| match direction {
            Direction::UP => last_car.position.1 < bg.window_height - spawn_distance,
            Direction::DOWN => last_car.position.1 > spawn_distance,
            Direction::RIGHT => last_car.position.0 > spawn_distance,
            Direction::LEFT => last_car.position.0 < bg.window_width - spawn_distance,
        });

    if can_spawn {
        let dis = get_dis();
        let new_car = Cars::new(spawn_position, direction, dis);
        cars.push(new_car);
    }
}