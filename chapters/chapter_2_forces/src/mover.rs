use macroquad::{color::WHITE, shapes::draw_circle};
use runner::{world_height, world_width};
use vec_math::Vec2;

pub struct Mover {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
    pub radius: f32,
}

impl Mover {
    pub fn new(position: Vec2, mass: f32) -> Self {
        let mass = mass.max(f32::EPSILON);

        Self {
            position,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            mass,
            radius: mass * 10.0,
        }
    }

    /// Apply a force using Newtorn's second law
    /// F = ma
    /// a= F / m
    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force / self.mass;
    }

    pub fn update(&mut self) {
        // Euler Integration.
        self.velocity += self.acceleration;
        self.position += self.velocity;

        // Acceleration is accumulated during the frame.
        // Clear it after integration so forces must be
        // applied again on the next frame.
        self.acceleration = Vec2::ZERO;
    }

    pub fn check_edges(&mut self) {
        let width = world_width();
        let height = world_height();

        // Left boundary
        if self.position.x < self.radius {
            self.position.x = self.radius;
            self.velocity.x *= -1.0;
        }

        // Right boundary
        if self.position.x > width - self.radius {
            self.position.x = width - self.radius;
            self.velocity.x *= -1.0;
        }

        // Top boundary
        if self.position.y < self.radius {
            self.position.y = self.radius;
            self.velocity.y *= -1.0;
        }

        // bottom boundary
        if self.position.y > height - self.radius {
            self.position.y = height - self.radius;
            self.velocity.y *= -1.0;
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, WHITE);
    }
}
