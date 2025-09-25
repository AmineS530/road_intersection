use crate::types::{TrafficLight,Light};
use macroquad::prelude::*;

impl TrafficLight {
    pub fn update(&mut self, dt: f32) {
        self.timer += dt;

        match self.state {
            Light::Green if self.timer >= self.green_time => {
                self.state = Light::Red;
                self.timer = 0.0;
            }
            Light::Red if self.timer >= self.red_time => {
                self.state = Light::Green;
                self.timer = 0.0;
            }
            _ => {}
        }
    }

    pub fn draw(&self, x: f32, y: f32) {
        // Black rectangle as background
        draw_rectangle(x, y, 20.0, 50.0, BLACK);

        // Red light
        draw_circle(
            x + 10.0,
            y + 10.0,
            8.0,
            if self.state == Light::Red { RED } else { GRAY },
        );

        // Green light
        draw_circle(
            x + 10.0,
            y + 40.0,
            8.0,
            if self.state == Light::Green { GREEN } else { GRAY },
        );
    }

    // Return true if cars can move
    pub fn is_green(&self) -> bool {
        self.state == Light::Green
    }
}