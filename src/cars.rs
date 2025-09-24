use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::rect::Rect;
#[derive(Debug,Clone, PartialEq)]
pub enum Direction {
    UP,
    LEFT,
    RIGHT,
    DOWN,
}
#[derive(Debug,Clone)]
pub enum Distination {
    LEFT,
    RIGHT,
    STRAIGHT,
}
pub enum Cocar {
    MAGENTA,
    BLUE,
    YELLOW,
}
#[derive(Debug,Clone)]
pub struct Cars {
    pub position: (i32, i32),
    pub direction: Direction,
    pub distination: Distination,
}

impl Cars {
    pub fn new(pos: (i32, i32), dir: Direction, dis: Distination) -> Self {
        Self {
            position: pos,
            direction: dir,
            distination: dis,
        }
    }
    pub fn car_draw(&self, canvas: &mut Canvas<Window>, color: Cocar) {
        match color {
            Cocar::MAGENTA => canvas.set_draw_color(Color::MAGENTA),
            Cocar::BLUE => canvas.set_draw_color(Color::BLUE),
            _ => canvas.set_draw_color(Color::YELLOW),
        }
        let _ = canvas.draw_rect(Rect::new(self.position.0, self.position.1, 60, 60));
        let _ = canvas.fill_rect(Some(Rect::new(self.position.0, self.position.1, 60, 60)));
    }
}

pub fn get_dis() -> Distination {
    use rand::Rng;
    let mut value = rand::rng();

    match value.random_range(0..=2) {
        0 => Distination::LEFT,
        1 => Distination::RIGHT,
        _ => Distination::STRAIGHT,
    }
}

impl Distination {
    pub fn colorize(&self) -> Cocar {
        match self {
            Distination::LEFT => Cocar::MAGENTA,
            Distination::RIGHT => Cocar::BLUE,
            Distination::STRAIGHT => Cocar::YELLOW,
        }
    }
}
