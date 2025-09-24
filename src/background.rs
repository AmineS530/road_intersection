use macroquad::prelude::*;

pub struct Scene {
    pub window_width: f32,
    pub window_height: f32,
}

impl Scene {
    pub fn new(window_width: f32, window_height: f32) -> Self {
        Scene {
            window_width,
            window_height,
        }
    }
    pub fn draw(&self) {
        draw_line(
            self.window_width / 2.,
            0.,
            self.window_width / 2.,
            self.window_height,
            1.,
            GRAY,
        );
        draw_line(
            0.,
            self.window_height / 2.,
            self.window_width,
            self.window_height / 2.,
            1.,
            GRAY,
        );

        draw_line(
            self.window_width / 2. + 60.,
            0.,
            self.window_width / 2. + 60.,
            self.window_height,
            1.,
            WHITE,
        );
        draw_line(
            self.window_width / 2. - 60.,
            0.,
            self.window_width / 2. - 60.,
            self.window_height,
            1.,
            WHITE,
        );

        draw_line(
            0.,
            self.window_height / 2. + 60.,
            self.window_width,
            self.window_height / 2. + 60.,
            1.,
            WHITE,
        );
        draw_line(
            0.,
            self.window_height / 2. - 60.,
            self.window_width,
            self.window_height / 2. - 60.,
            1.,
            WHITE,
        );
    }
}