use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct TrafficLight {
    pub position: (f32, f32),
    pub state: bool,
    pub empty: bool,
}

impl TrafficLight {
    pub fn new(x: f32, y: f32, initial_state: bool) -> Self {
        Self {
            position: (x, y),
            state: initial_state,
            empty: false,
        }
    }

    pub fn change_light(&mut self) {
        self.state = !self.state
    }

    pub fn draw_light(self) {
        let color = match self.state {
            false => RED,
            true => GREEN,
        };

        draw_rectangle(self.position.0, self.position.1, 40., 40., color);
    }
}