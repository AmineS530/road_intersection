use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct TrafficLight {
    pub position: (f32, f32),
    pub state: bool, // false = Red, true = Green
}

impl TrafficLight {
    pub fn new(x: f32, y: f32, initial_state: bool) -> Self {
        Self {
            position: (x, y),
            state: initial_state,
        }
    }

    pub fn change_light(&mut self) {
        self.state = !self.state;
    }

    pub fn draw(&self) {
        let color = if self.state { GREEN } else { RED };
        draw_rectangle(self.position.0, self.position.1, 40.0, 40.0, color);
    }
}