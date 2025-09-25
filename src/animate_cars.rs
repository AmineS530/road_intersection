use crate::types::*;
use macroquad::prelude::*;

impl Sprites {
    pub async fn load() -> Self {
        let tex_straight = load_texture("assets/sprites/car_00.png").await.unwrap();
        let tex_left = load_texture("assets/sprites/car_01.png").await.unwrap();
        let tex_right = load_texture("assets/sprites/car_02.png").await.unwrap();

        // Optional: enable texture filtering for sharper pixel art
        tex_left.set_filter(FilterMode::Nearest);
        tex_right.set_filter(FilterMode::Nearest);
        tex_straight.set_filter(FilterMode::Nearest);

        Self {
            tex_left,
            tex_right,
            tex_straight,
        }
    }

    pub fn texture_for_route(&self, route: &Route) -> &Texture2D {
        match route {
            Route::Left => &self.tex_left,
            Route::Right => &self.tex_right,
            Route::Straight => &self.tex_straight,
        }
    }
}

impl Vehicle {
    pub fn is_in_bounds(&self) -> bool {
        self.x >= -100.0
            && self.x <= G_WIDTH + 100.0
            && self.y >= -100.0
            && self.y <= G_HEIGHT + 100.0
    }

    pub fn update(&mut self, dt: f32, speed: f32) {
        self.frame_timer += dt;

        // Change frame every 0.1s
        if self.frame_timer > 0.1 {
            self.frame_timer = 0.0;
            self.frame_index = (self.frame_index + 1) % self.frame_count();
        }

        // Move car
        match self.direction {
            Direction::South => self.y += speed * dt,
            Direction::North => self.y -= speed * dt,
            Direction::East => self.x -= speed * dt,
            Direction::West => self.x += speed * dt,
        }
    }

    fn frame_count(&self) -> usize {
        match self.direction {
            Direction::South => 8,
            Direction::East => 2,
            Direction::West => 2,
            Direction::North => 2,
        }
    }

    pub fn draw(&self, sprite: &Sprites) {
        let frame_w = 88.0;
        let frame_h = 88.0;
        let texture = sprite.texture_for_route(&self.route);

        let (row, count) = match self.direction {
            Direction::South => (0, 8),
            Direction::East => (1, 2),
            Direction::West => (2, 2),
            Direction::North => (3, 2),
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
            self.y,
            WHITE,
            DrawTextureParams {
                source: Some(source),
                dest_size: Some(vec2(frame_w, frame_h)), // keep size
                ..Default::default()
            },
        );
    }
}
