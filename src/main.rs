use macroquad::prelude::*;

mod background;
mod cars;
mod traffic_light;
use background::*;
use cars::*;

use crate::traffic_light::TrafficLight;

#[macroquad::main("HIT THE ROAD JACK")]
async fn main() {
    let bg = Scene::new(900., 700.);

    let mut lights: Vec<TrafficLight> = Vec::new();
    lights.push(TrafficLight::new(bg.window_width / 2. + 61., bg.window_height / 2. + 61., false)); // For UP traffic
    lights.push(TrafficLight::new(bg.window_width / 2. - 100., bg.window_height / 2. - 100., false)); // For DOWN traffic
    lights.push(TrafficLight::new(bg.window_width / 2. - 100., bg.window_height / 2. + 61., true)); // For RIGHT traffic
    lights.push(TrafficLight::new(bg.window_width / 2. + 61., bg.window_height / 2. - 100., true)); // For LEFT traffic

    let mut cars: Vec<Cars> = vec![];
    
    let mut last_light_change = get_time();

    loop {
        if is_key_pressed(KeyCode::Up) {
            try_spawn_car(
                &mut cars,
                Direction::UP,
                (bg.window_width / 2. + 5., bg.window_height),
                80.,
                &bg,
            );
        }
        if is_key_pressed(KeyCode::Down) {
            try_spawn_car(
                &mut cars,
                Direction::DOWN,
                (bg.window_width / 2. - 55., -60.),
                20.,
                &bg,
            );
        }
        if is_key_pressed(KeyCode::Right) {
            try_spawn_car(
                &mut cars,
                Direction::RIGHT,
                (-60., bg.window_height / 2. + 5.),
                20.,
                &bg,
            );
        }
        if is_key_pressed(KeyCode::Left) {
            try_spawn_car(
                &mut cars,
                Direction::LEFT,
                (bg.window_width, bg.window_height / 2. - 55.),
                80.,
                &bg,
            );
        }

        if get_time() - last_light_change >= 5. {
            for light in lights.iter_mut() {
                light.change_light();
            }
            last_light_change = get_time();
        }

        clear_background(BLACK);
        bg.draw();

        for car in &mut cars {
            car.car_draw(car.distination.colorize());
        }

        for i in 0..cars.len() {
            let mut can_move = true;
            let car_rect = Rect::new(cars[i].position.0, cars[i].position.1, 60., 60.);
            let mut sensor_rect = car_rect;
            
            match cars[i].direction {
                Direction::UP => {
                    let stop_line = bg.window_height / 2. + 80.;
                    if !lights[0].state && car_rect.top() <= stop_line && car_rect.top() > stop_line - 10. {
                        can_move = false;
                    }
                    sensor_rect.y = car_rect.y - 5.;
                }
                Direction::DOWN => {
                    let stop_line = bg.window_height / 2. - 80.;
                    if !lights[1].state && car_rect.bottom() >= stop_line && car_rect.bottom() < stop_line + 10. {
                        can_move = false;
                    }
                    sensor_rect.y = car_rect.y + 5.;
                }
                Direction::RIGHT => {
                    let stop_line = bg.window_width / 2. - 80.;
                    if !lights[2].state && car_rect.right() >= stop_line && car_rect.right() < stop_line + 10. {
                        can_move = false;
                    }
                    sensor_rect.x = car_rect.x + 5.;
                }
                Direction::LEFT => {
                    let stop_line = bg.window_width / 2. + 80.;
                    if !lights[3].state && car_rect.left() <= stop_line && car_rect.left() > stop_line - 10. {
                        can_move = false;
                    }
                    sensor_rect.x = car_rect.x - 5.;
                }
            }

            for j in 0..cars.len() {
                if i == j { continue; }
                let other_car_rect = Rect::new(cars[j].position.0, cars[j].position.1, 60., 60.);
                if sensor_rect.overlaps(&other_car_rect) {
                    can_move = false;
                    break;
                }
            }

            if can_move {
                match cars[i].direction {
                    Direction::LEFT => cars[i].position.0 -= 3.,
                    Direction::RIGHT => cars[i].position.0 += 3.,
                    Direction::UP => cars[i].position.1 -= 3.,
                    Direction::DOWN => cars[i].position.1 += 3.,
                }
            }
        }
        
        for light in lights.iter() {
            light.draw_light();
        }

        next_frame().await
    }
}

fn try_spawn_car(
    cars: &mut Vec<Cars>,
    direction: Direction,
    spawn_position: (f32, f32),
    spawn_distance: f32,
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