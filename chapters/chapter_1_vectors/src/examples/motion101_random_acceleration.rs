use std::collections::VecDeque;

use macroquad::{
    color::{Color, GRAY, PINK, RED},
    shapes::draw_circle,
};

use vec_math::Vec2;

use runner::{
    Example,
    camera::{world_center, world_height, world_width},
    render::{draw_info_panel, draw_vector, draw_world_border},
};

pub struct Motion101RandomAcceleration {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    radius: f32,
    trail: VecDeque<Vec2>,
    max_speed: f32,
}

impl Motion101RandomAcceleration {
    pub fn new() -> Self {
        Self {
            position: world_center(),
            velocity: Vec2::default(),
            acceleration: Vec2::default(),
            radius: 15.0,
            trail: VecDeque::new(),
            max_speed: 2.5,
        }
    }
}

impl Example for Motion101RandomAcceleration {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        let w = world_width();
        let h = world_height();

        // Generate Random acceleration
        self.acceleration = Vec2::random_2d();

        // Random acceleration and magnitude
        let magnitude = macroquad::rand::gen_range(0., 1.);
        self.acceleration *= magnitude;

        // Euler Integration
        self.velocity += self.acceleration;
        self.velocity.limit(self.max_speed);

        self.position += self.velocity;

        // Store position for trail
        self.trail.push_back(self.position);

        if self.trail.len() > 150 {
            self.trail.pop_front();
        }

        // Bounce from the window boundaries
        if self.position.x < self.radius {
            self.position.x = self.radius;
            self.velocity.x *= -1.0;
        }

        if self.position.x > w - self.radius {
            self.position.x = w - self.radius;
            self.velocity.x *= -1.0;
        }

        if self.position.y < self.radius {
            self.position.y = self.radius;
            self.velocity.y *= -1.0;
        }

        if self.position.y > h - self.radius {
            self.position.y = h - self.radius;
            self.velocity.y *= -1.0;
        }
    }

    fn draw(&self) {
        draw_world_border();

        // Draw trails
        for (i, point) in self.trail.iter().enumerate() {
            let alpha = (i + 1) as f32 / self.trail.len().max(1) as f32;

            draw_circle(point.x, point.y, 3.0, Color::new(1.0, 1.0, 1.0, alpha));
        }

        // Draw ball
        draw_circle(self.position.x, self.position.y, self.radius, PINK);

        // Draw Velocity vector
        draw_vector(self.position, self.velocity * 15.0, GRAY);

        // Draw Acceleration vector
        draw_vector(self.position, self.acceleration * 300.0, RED);

        let speed_str = format!("Speed : {:.2}", self.velocity.mag());
        let acc_str = format!("Accel : {:.3}", self.acceleration.mag());

        draw_info_panel(
            10.0,
            world_height() - 48.0,
            160.0,
            &[
                (&speed_str, GRAY),
                (&acc_str, RED),
            ],
        );
    }
}

