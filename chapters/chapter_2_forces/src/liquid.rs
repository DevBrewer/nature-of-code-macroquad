use vec_math::Vec2;

use crate::mover::Mover;

pub struct Liquid {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,

    /// Simplified drag coefficient
    pub drag_coefficient: f32,
}

impl Liquid {
    pub fn new(x: f32, y: f32, width: f32, height: f32, drag_coefficient: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            drag_coefficient,
        }
    }

    /// Returns true when the mover is inside the liquid region.
    pub fn contains(&self, mover: &Mover) -> bool {
        let position = mover.position;

        // checking Point in rectangle overlap check.
        position.x > self.x
            && position.x < self.x + self.width
            && position.y > self.y
            && position.y < self.y + self.height
    }

    /// Calculate the simplified drag force:
    ///
    /// Fd = -c * v² * v̂
    pub fn calculate_drag(&self, mover: &Mover) -> Vec2 {
        let speed = mover.velocity.mag();

        if speed <= f32::EPSILON {
            return Vec2::ZERO;
        }

        // Drag magnitude grows with speed squared.
        let magnitude = self.drag_coefficient * speed * speed;

        // Drag acts opposite velocity.
        let direction = mover.velocity.normalized() * -1.0;

        direction * magnitude
    }
}
