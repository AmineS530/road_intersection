use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
pub struct Scene {
    pub window_width: i32,
    pub window_height: i32,
}

impl Scene {
    pub fn new(window_width: i32, window_height: i32) -> Self {
        Scene {
            window_width,
            window_height,
        }
    }
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::GRAY);
        canvas
            .draw_line(
                Point::new(self.window_width / 2, 0),
                Point::new(self.window_width / 2, self.window_height)
            )
            .unwrap();
        canvas
            .draw_line(
                Point::new(0, self.window_height / 2),
                Point::new(self.window_width, self.window_height / 2)
            )
            .unwrap();

        canvas.set_draw_color(Color::WHITE);
        canvas
            .draw_line(
                Point::new(self.window_width / 2 + 60, 0),
                Point::new(self.window_width / 2 + 60, self.window_height)
            )
            .unwrap();
        canvas
            .draw_line(
                Point::new(self.window_width / 2 - 60, 0),
                Point::new(self.window_width / 2 - 60, self.window_height)
            )
            .unwrap();

        canvas
            .draw_line(
                Point::new(0, self.window_height / 2 + 60),
                Point::new(self.window_width, self.window_height / 2 + 60)
            )
            .unwrap();
        canvas
            .draw_line(
                Point::new(0, self.window_height / 2 - 60),
                Point::new(self.window_width, self.window_height / 2 - 60)
            )
            .unwrap();
    }
}
