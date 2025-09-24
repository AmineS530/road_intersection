use macroquad::prelude::*;

#[derive(Clone, Copy)]
enum Direction {
    Down,
    Up,
    Left,
    Right,
}

struct Car {
    x: f32,
    y: f32,
    direction: Direction,
    frame_index: usize,
    frame_timer: f32,
}

impl Car {
    fn update(&mut self, dt: f32) {
        // Step the timer
        self.frame_timer += dt;

        // Change frame every 0.1s
        if self.frame_timer > 0.1 {
            self.frame_timer = 0.0;
            self.frame_index = (self.frame_index + 1) % self.frame_count();
        }

        // Move car
        match self.direction {
            Direction::Down => self.y += 100.0 * dt,
            Direction::Up => self.y -= 100.0 * dt,
            Direction::Left => self.x -= 100.0 * dt,
            Direction::Right => self.x += 100.0 * dt,
        }
    }

    fn frame_count(&self) -> usize {
        match self.direction {
            Direction::Down => 4,  // 8 frames in "down" row
            Direction::Left => 2,  // 2 frames in "left" row
            Direction::Right => 2, // 2 frames in "right" row
            Direction::Up => 2,    // 1 frame in "up" row
        }
    }

    fn draw(&self, texture: &Texture2D) {
        let frame_w = 88.0;
        let frame_h = 88.0;

        let (row, count) = match self.direction {
            Direction::Down => (0, 8),
            Direction::Left => (1, 2),
            Direction::Right => (2, 2),
            Direction::Up => (3, 2),
        };

        // Clamp frame index to available count
        let frame = self.frame_index % count;

        let source = Rect::new(
            frame as f32 * frame_w,
            row as f32 * frame_h,
            frame_w,
            frame_h,
        );

        
        draw_texture_ex(
            texture,
            self.x + 20.,
            self.y ,
            WHITE,
            DrawTextureParams {
                source: Some(source),
                dest_size: Some(vec2(frame_w, frame_h)), // keep size
                ..Default::default()
            },
        );
    }
}

#[macroquad::main("Car Animation Test")]
async fn main() {
    let texture: Texture2D = load_texture("../assets/sprites/car_00.png").await.unwrap();

    let mut car = Car {
        x: 200.0,
        y: 100.0,
        direction: Direction::Down,
        frame_index: 0,
        frame_timer: 0.0,
    };

    loop {
        clear_background(GREEN);

        let dt = get_frame_time();
        car.update(dt);
        car.draw(&texture);

        // Example: press LEFT/RIGHT arrow to turn car
        if is_key_pressed(KeyCode::Left) {
            car.direction = Direction::Left;
            car.frame_index = 0;
        }
        if is_key_pressed(KeyCode::Right) {
            car.direction = Direction::Right;
            car.frame_index = 0;
        }
        if is_key_pressed(KeyCode::Up) {
            car.direction = Direction::Up;
            car.frame_index = 0;
        }
        if is_key_pressed(KeyCode::Down) {
            car.direction = Direction::Down;
            car.frame_index = 0;
        }

        next_frame().await;
    }
}
