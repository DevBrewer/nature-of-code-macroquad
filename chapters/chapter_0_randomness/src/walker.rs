use macroquad::{color::WHITE, shapes::draw_circle};
use rand::RngExt;
use runner::{world_height, world_width};

pub struct Walker {
    x: f32,
    y: f32,
}

impl Walker {
    pub fn new() -> Self {
        Self {
            x: world_width() * 0.5,
            y: world_height() * 0.5,
        }
    }

    pub fn step(&mut self) {
        let mut rng = rand::rng();

        match rng.random_range(0..4) {
            1 => self.x += 1.0, // right
            2 => self.x -= 1.0, // left
            3 => self.y += 1.0, // down
            _ => self.y -= 1.0, // up
        }

        self.x = self.x.clamp(0.0, world_width());
        self.y = self.y.clamp(0.0, world_height());
    }

    pub fn draw(&self) {
        draw_circle(self.x, self.y, 2.0, WHITE);
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}
