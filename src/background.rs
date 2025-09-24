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
        let center_x = self.window_width / 2.0;
        let center_y = self.window_height / 2.0;
        let road_width = 60.0;

        // Draw horizontal and vertical roads to form the intersection
        draw_rectangle(0.0, center_y - road_width, self.window_width, road_width * 2.0, GRAY);
        draw_rectangle(center_x - road_width, 0.0, road_width * 2.0, self.window_height, GRAY);

        // Dashed lines to separate lanes
        for i in (0..(self.window_width as i32)).step_by(40) {
            draw_line(i as f32, center_y, i as f32 + 20.0, center_y, 2.0, WHITE);
        }
        for i in (0..(self.window_height as i32)).step_by(40) {
            draw_line(center_x, i as f32, center_x, i as f32 + 20.0, 2.0, WHITE);
        }

        // Stop lines where cars must wait for a green light
        let stop_line_thickness = 4.0;
        draw_rectangle(center_x - road_width, center_y - road_width - stop_line_thickness, road_width * 2.0, stop_line_thickness, WHITE);
        draw_rectangle(center_x - road_width, center_y + road_width, road_width * 2.0, stop_line_thickness, WHITE);
        draw_rectangle(center_x - road_width - stop_line_thickness, center_y - road_width, stop_line_thickness, road_width * 2.0, WHITE);
        draw_rectangle(center_x + road_width, center_y - road_width, stop_line_thickness, road_width * 2.0, WHITE);
    }
}