use macroquad::{color::BLUE, shapes::draw_circle};
use runner::{
    Example,
    camera::{world_height, world_width},
    render::draw_world_border,
};

pub struct BouncingBall {
    x: f32,
    y: f32,
    x_speed: f32,
    y_speed: f32,
    radius: f32,
}

impl BouncingBall {
    pub fn new() -> Self {
        Self {
            x: 100.0,
            y: 100.0,
            x_speed: 2.5,
            y_speed: 2.5,
            radius: 20.0,
        }
    }
}

impl Example for BouncingBall {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        let w = world_width();
        let h = world_height();

        // Move
        self.x += self.x_speed;
        self.y += self.y_speed;

        // Left
        if self.x < self.radius {
            self.x = self.radius;
            self.x_speed *= -1.0;
        }

        // Right
        if self.x > w - self.radius {
            self.x = w - self.radius;
            self.x_speed *= -1.0;
        }

        // Top
        if self.y < self.radius {
            self.y = self.radius;
            self.y_speed *= -1.0;
        }

        // Bottom
        if self.y > h - self.radius {
            self.y = h - self.radius;
            self.y_speed *= -1.0;
        }
    }

    fn draw(&self) {
        draw_world_border();
        draw_circle(self.x, self.y, self.radius, BLUE);
    }
}

