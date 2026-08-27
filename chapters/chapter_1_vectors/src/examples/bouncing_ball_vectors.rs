use macroquad::{color::ORANGE, shapes::draw_circle};
use runner::{
    Example,
    camera::{world_height, world_width},
    render::draw_world_border,
};
use vec_math::Vec2;

pub struct BouncingBallVectors {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
}

impl BouncingBallVectors {
    pub fn new() -> Self {
        Self {
            position: Vec2::new(100.0, 100.0),
            velocity: Vec2::new(2.5, 2.0),
            radius: 24.0,
        }
    }
}

impl Example for BouncingBallVectors {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        let w = world_width();
        let h = world_height();

        // Move
        self.position += self.velocity;

        // Left
        if self.position.x < self.radius {
            self.position.x = self.radius;
            self.velocity.x *= -1.0;
        }

        // Right
        if self.position.x > w - self.radius {
            self.position.x = w - self.radius;
            self.velocity.x *= -1.0;
        }

        // Top
        if self.position.y < self.radius {
            self.position.y = self.radius;
            self.velocity.y *= -1.0;
        }

        // Bottom
        if self.position.y > h - self.radius {
            self.position.y = h - self.radius;
            self.velocity.y *= -1.0;
        }
    }

    fn draw(&self) {
        draw_world_border();
        draw_circle(self.position.x, self.position.y, self.radius, ORANGE);
    }
}

