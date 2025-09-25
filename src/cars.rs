// use crate::types::{Direction, G_HEIGHT, G_WIDTH, Route, Vehicle};




// impl Vehicle {
//     pub fn new(dir: Direction, route: Route) -> Self {
//         Vehicle {
//             x: (),
//             y: (),
//             direction: dir,
//             route: route,
//             speed: (),
//             frame_index: (),
//             frame_timer: (),
//         }
//     }
// }

// pub fn spawn_car(car_vec: &mut Vec<Vehicle>, dir: Direction) {
//     let safety_distance: f32 = 60.0;

//     // Check the last car in the same direction
//     if let Some(last_car) = car_vec.iter().rev().find(|v| v.direction == dir) {
//         let too_close = match dir {
//             Direction::North => last_car.y > safety_distance,
//             Direction::South => last_car.y < G_HEIGHT - safety_distance,
//             Direction::East => last_car.x > safety_distance,
//             Direction::West => last_car.x < G_WIDTH - safety_distance,
//         };
//         if !too_close {
//             return;
//         }
//     }

//     // Create vehicle with proper spawn position
//     // let new_vehicle = Vehicle::new(dir);
//     // car_vec.push(new_vehicle);
// }
