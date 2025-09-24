use macroquad::prelude::*;
use crate::background::Scene;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Direction {
    UP,
    LEFT,
    RIGHT,
    DOWN,
}

#[derive(Debug, Clone, Copy)]
pub enum Destination {
    LEFT,
    RIGHT,
    STRAIGHT,
}

#[derive(Debug, Clone, Copy)]
pub struct Cars {
    pub position: (f32, f32),
    pub original_direction: Direction,
    pub direction: Direction,
    pub destination: Destination,
    pub speed: f32,
}

impl Cars {
    pub fn new(pos: (f32, f32), dir: Direction, dest: Destination) -> Self {
        Self {
            position: pos,
            original_direction: dir,
            direction: dir,
            destination: dest,
            speed: 2.5,
        }
    }

    // Car color is determined by its destination.
    pub fn draw(&self) {
        let color = match self.destination {
            Destination::LEFT => BLUE,
            Destination::RIGHT => YELLOW,
            Destination::STRAIGHT => MAGENTA,
        };
        draw_rectangle(self.position.0, self.position.1, 60.0, 60.0, color);
    }
    
    // Updates car position and handles turning logic when inside the intersection.
    pub fn update(&mut self, bg: &Scene) {
        let intersection_rect = Rect::new(
            bg.window_width / 2.0 - 60.0,
            bg.window_height / 2.0 - 60.0,
            120.0,
            120.0,
        );
        let car_center = vec2(self.position.0 + 30.0, self.position.1 + 30.0);

        if intersection_rect.contains(car_center) {
            // To ensure the car turns only once, we check if its current direction
            // is still its original one.
            if self.direction == self.original_direction {
                self.direction = self.get_turn_direction();
            }
        }
        
        match self.direction {
            Direction::LEFT => self.position.0 -= self.speed,
            Direction::RIGHT => self.position.0 += self.speed,
            Direction::UP => self.position.1 -= self.speed,
            Direction::DOWN => self.position.1 += self.speed,
        }
    }
    
    // Determines the new direction based on original direction and destination.
    fn get_turn_direction(&self) -> Direction {
        match self.original_direction {
            Direction::UP => match self.destination {
                Destination::LEFT => Direction::LEFT,
                Destination::RIGHT => Direction::RIGHT,
                Destination::STRAIGHT => Direction::UP,
            },
            Direction::DOWN => match self.destination {
                Destination::LEFT => Direction::RIGHT,
                Destination::RIGHT => Direction::LEFT,
                Destination::STRAIGHT => Direction::DOWN,
            },
            Direction::LEFT => match self.destination {
                Destination::LEFT => Direction::DOWN,
                Destination::RIGHT => Direction::UP,
                Destination::STRAIGHT => Direction::LEFT,
            },
            Direction::RIGHT => match self.destination {
                Destination::LEFT => Direction::UP,
                Destination::RIGHT => Direction::DOWN,
                Destination::STRAIGHT => Direction::RIGHT,
            },
        }
    }
}

// Assigns a random destination to a newly spawned car.
pub fn get_random_destination() -> Destination {
    match rand::gen_range(0, 3) {
        0 => Destination::LEFT,
        1 => Destination::RIGHT,
        _ => Destination::STRAIGHT,
    }
}