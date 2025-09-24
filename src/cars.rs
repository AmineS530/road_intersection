use macroquad::prelude::*;

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
    pub position: (f32, f32),
    pub direction: Direction,
    pub distination: Distination,
}

impl Cars {
    pub fn new(pos: (f32, f32), dir: Direction, dis: Distination) -> Self {
        Self {
            position: pos,
            direction: dir,
            distination: dis,
        }
    }
    pub fn car_draw(&self, color: Cocar) {
        let color = match color {
            Cocar::MAGENTA => MAGENTA,
            Cocar::BLUE => BLUE,
            _ => YELLOW,
        };
        draw_rectangle(self.position.0, self.position.1, 60., 60., color);
    }
}

pub fn get_dis() -> Distination {
    use macroquad::rand::gen_range;

    match gen_range(0, 3) {
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