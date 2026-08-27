use macroquad::{
    color::WHITE,
    shapes::draw_circle,
    window::{screen_height, screen_width},
};
use rand::RngExt;

pub struct RightWalker {
    x: f32,
    y: f32,
}

impl RightWalker {
    pub fn new() -> Self {
        Self {
            x: screen_width() * 0.5,
            y: screen_height() * 0.5,
        }
    }

    pub fn step(&mut self) {
        let mut rng = rand::rng();

        // Generate a random value in [0, 1]
        let choice: f32 = rng.random();

        if choice < 0.5 {
            self.x += 1.0;
        } else if choice < 0.7 {
            self.x -= 1.0;
        } else if choice < 0.85 {
            self.y += 1.0;
        } else {
            self.y -= 1.0;
        }

        // Keep the walker inside the canvas.
        self.x = self.x.clamp(0.0, screen_width());
        self.y = self.y.clamp(0.0, screen_height());
    }

    pub fn draw(&self) {
        draw_circle(self.x, self.y, 2.0, WHITE);
    }
}
