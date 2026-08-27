use macroquad::{color::RED, shapes::draw_circle};
use runner::world_center;
use vec_math::Vec2;

use crate::mover::Mover;

pub struct Attractor {
    pub position: Vec2,
    pub mass: f32,
}

impl Attractor {
    pub fn new() -> Self {
        Self {
            position: world_center(),
            mass: 4.0,
        }
    }

    /// Calculate the gravitional attraction exerted
    /// by this attractor on a mover
    ///
    /// F = G * (m1 * m2)/r*r
    pub fn attract(&self, mover: &Mover) -> Vec2 {
        // Direction from mover toward attractor.
        let mut force = self.position - mover.position;

        // Distance between the two objects.
        let mut distance = force.mag();

        // Prevent division by zero and prevent
        // extremly weak/strong forces.
        distance = distance.clamp(0.5, 25.0);

        // Universal gravitional constant
        // This is a simulation scaling value rather
        // than the real-world SI value of G.
        let g = 1.0;

        // Newton's law of universal gravition.
        let strength = (g * self.mass * mover.mass) / (distance * distance);

        // Convert the direction vector into
        // the required force magnitude.
        force = force.normalized() * strength;
        force
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.mass, RED);
    }
}
