use std::f32::consts::TAU;

use macroquad::{
    color::{Color, WHITE},
    rand::gen_range,
    shapes::draw_circle,
};
use runner::world_size;
use vec_math::Vec2;
/// A bioluminiscent plankton food particle exhibiting gentle harmonic drift
pub struct Plankton {
    pub position: Vec2,
    pub base_position: Vec2,
    #[allow(dead_code)]
    pub mass: f32,
    pub amplitude: f32,
    pub frequency: f32,
    pub phase: f32,
    pub is_alive: bool,
    pub glow_color: Color,
}

impl Plankton {
    pub fn new(position: Vec2) -> Self {
        let phase = gen_range(0.0, TAU);
        let frequency = gen_range(1.2, 2.5);
        let amplitude = gen_range(4.0, 10.0);

        Self {
            position,
            base_position: position,
            mass: 1.0,
            amplitude,
            frequency,
            phase,
            is_alive: true,
            glow_color: Color::new(0.40, 1.0, 0.65, 0.85),
        }
    }

    /// Spawns a plankton randomly within the world boundaries
    pub fn random_in_world(margin: f32) -> Self {
        let Vec2 {
            x: width,
            y: height,
        } = world_size();

        let x = gen_range(margin, (width - margin).max(margin + 10.0));
        let y = gen_range(margin, (height - margin).max(margin + 10.0));
        Self::new(Vec2::new(x, y))
    }

    pub fn update(&mut self, dt: f32) {
        self.phase += self.frequency * dt;
        let offset_x = self.phase.sin() * self.amplitude;
        let offset_y = (self.phase * 0.7).cos() * (self.amplitude * 0.4);
        self.position = self.base_position + Vec2::new(offset_x, offset_y);
    }

    pub fn draw(&self) {
        if !self.is_alive {
            return;
        }
        let mut halo = self.glow_color;

        halo.a = 0.25;
        draw_circle(self.position.x, self.position.y, 6.0, halo);
        draw_circle(self.position.x, self.position.y, 3.0, self.glow_color);
        draw_circle(self.position.x, self.position.y, 1.5, WHITE);
    }
}
