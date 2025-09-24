use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Debug, Clone, Copy)]
pub struct TrafficLight {
    pub position: (i32, i32),
    pub state: bool, // false = Red, true = Green
    pub empty: bool,
}

impl TrafficLight {
    pub fn new(x: i32, y: i32, initial_state: bool) -> Self {
        Self {
            position: (x, y),
            state: initial_state,
            empty: false,
        }
    }

    pub fn change_light(&mut self) {
        self.state = !self.state
    }

    pub fn draw_light(self, canvas: &mut Canvas<Window>) {
        match self.state {
            false => canvas.set_draw_color(Color::RED),
            true => canvas.set_draw_color(Color::GREEN),
        };

        let _ = canvas.draw_rect(Rect::new(self.position.0, self.position.1, 40, 40));
        let _ = canvas.fill_rect(Some(Rect::new(self.position.0, self.position.1, 40, 40)));
    }
}