use macroquad::{
    color::{GRAY, GREEN},
    rand::gen_range,
    shapes::{draw_circle_lines, draw_line},
};
use vec_math::Vec2;

pub struct Body {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
    pub radius: f32,

    // Angular motion
    pub angle: f32,
    pub angle_velocity: f32,
    pub angle_acceleration: f32,
}

impl Body {
    pub fn new(position: Vec2, mass: f32) -> Self {
        Self {
            position,
            velocity: Vec2::new(gen_range(-1.0, 1.0), gen_range(-1.0, 1.0)),
            acceleration: Vec2::ZERO,
            mass,
            radius: mass * 8.0,
            angle: 0.0,
            angle_velocity: 0.0,
            angle_acceleration: 0.0,
        }
    }

    /// Newton's second law
    /// F=ma => a = F/m
    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force / self.mass
    }

    /// Integrate both linear an angular motion.
    pub fn update(&mut self) {
        // Euler Integration
        // linear motion
        self.velocity += self.acceleration;
        self.position += self.velocity;

        // Angular motion
        // Relationship between linear accleration and angular acceleration
        self.angle_acceleration = self.acceleration.x / 10.0;
        self.angle_velocity += self.angle_acceleration;

        // Prevent excessive rotation speed.
        self.angle_velocity = self.angle_velocity.clamp(-0.1, 0.1);
        self.angle += self.angle_velocity;

        // Forces are accumulated during the frame.
        // Clear acceleration before the next frame.
        self.acceleration = Vec2::ZERO;
    }

    pub fn draw(&self) {
        draw_circle_lines(self.position.x, self.position.y, self.radius, 2.0, GRAY);

        // A circle itself doesn't reveal its rotation.
        let direction = Vec2::new(self.radius, 0.0).rotate(self.angle);

        let end = self.position + direction;
        draw_line(self.position.x, self.position.y, end.x, end.y, 2.0, GREEN);
    }
}
