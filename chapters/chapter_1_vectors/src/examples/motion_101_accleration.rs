use std::collections::VecDeque;

use macroquad::{
    color::{Color, GRAY, PINK, RED},
    shapes::draw_circle,
};
use vec_math::Vec2;

use runner::{
    Example,
    camera::{world_height, world_width},
    render::{draw_info_panel, draw_vector, draw_world_border},
};

pub struct Motion101Acceleraton {
    position: Vec2,
    velocity: Vec2,
    accleration: Vec2,
    radius: f32,
    trail: VecDeque<Vec2>,
}

impl Motion101Acceleraton {
    pub fn new() -> Self {
        Self {
            position: Vec2::new(80.0, world_height() / 2.0),
            velocity: Vec2::new(0.0, 0.0),
            accleration: Vec2::new(0.05, 0.0),
            radius: 20.0,
            trail: VecDeque::new(),
        }
    }
}

impl Example for Motion101Acceleraton {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        let w = world_width();
        let h = world_height();

        // Euler integration explicit
        self.velocity += self.accleration;
        self.position += self.velocity;

        // Record current position
        self.trail.push_back(self.position);

        if self.trail.len() > 150 {
            self.trail.pop_front();
        }

        // Left boundary
        if self.position.x < self.radius {
            self.position.x = self.radius;
            self.velocity.x *= -1.0;
        }

        // Right boundary
        if self.position.x > w - self.radius {
            self.position.x = w - self.radius;
            self.velocity.x *= -1.0;
        }

        // Top boundary
        if self.position.y < self.radius {
            self.position.y = self.radius;
            self.velocity.y *= -1.0;
        }

        // Bottom boundary
        if self.position.y > h - self.radius {
            self.position.y = h - self.radius;
            self.velocity.y *= -1.0;
        }

        // accleration unchanged
    }

    fn draw(&self) {
        draw_world_border();

        for (i, point) in self.trail.iter().enumerate() {
            let t = (i + 1) as f32 / self.trail.len() as f32;

            draw_circle(point.x, point.y, 4.0, Color::new(1.0, 1.0, 1.0, t));
        }

        // Draw current ball
        draw_circle(self.position.x, self.position.y, self.radius, PINK);

        // Velocity vector (scaled so it's easier to see)
        draw_vector(self.position, self.velocity * 15.0, GRAY);

        // Acceleration vector
        draw_vector(self.position, self.accleration * 500.0, RED);

        let vel_str = format!("Velocity : {:.2}", self.velocity.mag());
        let acc_str = format!("Accel    : ({:.2}, {:.2})", self.accleration.x, self.accleration.y);
        let trail_str = format!("Trail    : {} pts", self.trail.len());

        draw_info_panel(
            10.0,
            world_height() - 62.0,
            200.0,
            &[
                (&vel_str, GRAY),
                (&acc_str, RED),
                (&trail_str, macroquad::color::WHITE),
            ],
        );
    }
}

