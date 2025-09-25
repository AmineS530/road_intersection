use macroquad::prelude::*;

mod road;

use road::*;

#[macroquad::main("Road Intersection")]
async fn main() {
    let width = screen_width();
    let height = screen_height();
    let mut north_lane = Lane::new(
        DirectionLane::North,
        Point(width / 2.0 - 50.0, 0.0),
        Point(width / 2.0 - 50.0, height / 2.0 - 50.0),
        width,
        height
    );

    let mut south_lane = Lane::new(
        DirectionLane::South,
        Point(width / 2.0, height - 50.0),
        Point(width / 2.0, height / 2.0 + 50.0),
        width,
        height
    );

    let mut west_lane = Lane::new(
        DirectionLane::West,
        Point(width - 50.0, height / 2.0 - 50.0),
        Point(width / 2.0 + 50.0, height / 2.0 - 50.0),
        width,
        height
    );

    let mut east_lane = Lane::new(
        DirectionLane::East,
        Point(0.0, height / 2.0),
        Point(width / 2.0 - 50.0, height / 2.0),
        width,
        height
    );
    let center_x = width / 2.0;
    let center_y = height / 2.0;

    let mut north_light = TrafficLight::new(Point(center_x - 95.0, center_y - 95.0), 5.0, 5.0);
    let mut south_light = TrafficLight::new(Point(center_x + 50.0, center_y + 50.0), 5.0, 5.0);
    let mut west_light = TrafficLight::new(Point(center_x + 50.0, center_y - 95.0), 5.0, 5.0);
    let mut east_light = TrafficLight::new(Point(center_x - 95.0, center_y + 50.0), 5.0, 5.0);

    loop {
        clear_background(BLACK);
        let dt = get_frame_time();

        north_light.update(dt);
        south_light.update(dt);
        west_light.update(dt);
        east_light.update(dt);

        north_light.draw();
        south_light.draw();
        west_light.draw();
        east_light.draw();

        // vertical lines so y =0
        draw_line(width / 2.0, 0.0, width / 2.0, height, 1.0, WHITE);

        // draw 2 more lines with the same distance from the center
        draw_line(width / 2.0 - 50.0, 0.0, width / 2.0 - 50.0, height, 1.0, WHITE);
        draw_line(width / 2.0 + 50.0, 0.0, width / 2.0 + 50.0, height, 1.0, WHITE);

        // horizontal lines  so  x is width/2 and y is height/2
        draw_line(0.0, height / 2.0, width, height / 2.0, 1.0, WHITE);
        // the same with these lines too the 50px for the cars where they need to move
        draw_line(0.0, height / 2.0 - 50.0, width, height / 2.0 - 50.0, 1.0, WHITE);
        draw_line(0.0, height / 2.0 + 50.0, width, height / 2.0 + 50.0, 1.0, WHITE);
        // let's handle the keys pressed so everytime we press a key a

        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }

        // we need to create and know the lanes and based on this we will draw the vehicles

        let last_pressed_key = get_last_key_pressed();
        match last_pressed_key {
            Some(key_val_pressed) => {
                if is_allowed_key(key_val_pressed) {
                    match key_val_pressed {
                        KeyCode::Up => {
                            north_lane.vehicles.push(Vehicle::new(&north_lane));
                        }
                        KeyCode::Down => {
                            south_lane.vehicles.push(Vehicle::new(&south_lane));
                        }
                        KeyCode::Left => {
                            east_lane.vehicles.push(Vehicle::new(&east_lane));
                        }
                        KeyCode::Right => {
                            west_lane.vehicles.push(Vehicle::new(&west_lane));
                        }
                        KeyCode::R => {
                            let random_direction_lane = random_direction_lane();
                            match random_direction_lane {
                                DirectionLane::North => {
                                    north_lane.vehicles.push(Vehicle::new(&north_lane));
                                }
                                DirectionLane::South => {
                                    south_lane.vehicles.push(Vehicle::new(&south_lane));
                                }
                                DirectionLane::East => {
                                    east_lane.vehicles.push(Vehicle::new(&east_lane));
                                }
                                DirectionLane::West => {
                                    west_lane.vehicles.push(Vehicle::new(&west_lane));
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            None => {}
        }

        for car in &mut east_lane.vehicles {
            car.draw_vehicle();
            car.update();
        }
        for car in &mut west_lane.vehicles {
            car.draw_vehicle();
            car.update();
        }
        for car in &mut north_lane.vehicles {
            car.draw_vehicle();
            car.update();
        }

        for car in &mut south_lane.vehicles {
            car.draw_vehicle();
            car.update();
        }

        next_frame().await;
    }
}

pub fn is_allowed_key(keycode: KeyCode) -> bool {
    keycode == KeyCode::Up ||
        keycode == KeyCode::Down ||
        keycode == KeyCode::Left ||
        keycode == KeyCode::Right ||
        keycode == KeyCode::R
}
