use std::f32::consts::TAU;

use macroquad::{
    color::Color,
    shapes::{draw_circle, draw_line},
    time::get_frame_time,
};
use runner::{Example, draw_world_border, world_center, world_height, world_width};
use vec_math::Vec2;

// Exercise 3.8
//Try initializing each Oscillator object with velocities and amplitudes that aren’t random to create some sort of regular pattern. Can you make the oscillators appear to be the legs of an insect-like creature?

pub struct RadialPetal {
    pub angle: Vec2,
    pub angle_velocity: Vec2,
    pub amplitude: Vec2,
}

impl RadialPetal {
    pub fn new(width: f32, height: f32, index: usize, total: usize) -> Self {
        let angle_step = TAU / total as f32;
        let base_angle = index as f32 * angle_step;

        // Staggering phase offsets per index to create a blooming, cascading wave
        let angle = Vec2::new(index as f32 * 0.2, index as f32 * 0.2);
        let angle_velocity = Vec2::new(0.01, 0.01);

        // Use trigonometric distribution to map amplitudes symmetrically into sunburst
        let radius = width.min(height) * 0.35;
        let amplitude = Vec2::new(
            base_angle.cos() * radius + radius * 0.2,
            base_angle.sin() * radius + radius * 0.2,
        );

        Self {
            angle,
            angle_velocity,
            amplitude,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.angle += self.angle_velocity * dt;
    }

    pub fn show(&self) {
        let center = world_center();

        let x = center.x + self.angle.x.sin() * self.amplitude.x;
        let y = center.y + self.angle.y.sin() * self.amplitude.y;

        // Draw line
        draw_line(
            center.x,
            center.y,
            x,
            y,
            1.5,
            Color::from_rgba(138, 43, 226, 64),
        );

        // Draw Circle
        draw_circle(x, y, 10.0, Color::from_rgba(138, 43, 226, 255));
    }
}

pub struct RadialPetalExample {
    petals: Vec<RadialPetal>,
}

impl RadialPetalExample {
    pub fn new() -> Self {
        let total_petals = 24;
        let petals = (0..total_petals)
            .map(|idx| RadialPetal::new(world_width(), world_height(), idx, total_petals))
            .collect();

        Self { petals }
    }
}

impl Example for RadialPetalExample {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        let dt = get_frame_time() * 60.0;

        self.petals.iter_mut().for_each(|petal| petal.update(dt));
    }

    fn draw(&self) {
        draw_world_border();

        self.petals.iter().for_each(|petal| petal.show());
    }
}
