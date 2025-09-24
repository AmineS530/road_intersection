// Example pseudocode:

// struct Car {
//     x: f32,
//     y: f32,
//     direction: Direction,
//     frame_index: usize,
//     frame_timer: f32,
// }

// impl Car {
//     fn update(&mut self, dt: f32) {
//         // Step the timer
//         self.frame_timer += dt;

//         // Change frame every 0.1s
//         if self.frame_timer > 0.1 {
//             self.frame_timer = 0.0;
//             self.frame_index = (self.frame_index + 1) % self.frame_count();
//         }

//         // Move based on direction
//         match self.direction {
//             Direction::Down => self.y += 2.0,
//             Direction::Up => self.y -= 2.0,
//             Direction::Left => self.x -= 2.0,
//             Direction::Right => self.x += 2.0,
//         }
//     }
// }

// 4. Handling Turning

// When the car turns:

// Change its direction (e.g., from Down to Left).

// Reset the frame index to 0 for that direction.

// Switch which row of the sprite sheet you’re using.

// Example:

// if turning_left {
//     car.direction = Direction::Left;
//     car.frame_index = 0;
// }


// Visually:

// When car is going down, cycle through the 8 “Down” frames to look like driving.

// When you turn left, swap to the 2 “Left” frames, and so on.