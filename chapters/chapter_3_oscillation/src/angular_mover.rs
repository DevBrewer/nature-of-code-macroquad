use vec_math::Vec2;

use crate::body::Body;

pub struct AngularMover {
    pub body: Body,
    pub angle: f32,
    pub angle_velocity: f32,
    pub angle_acceleration: f32,
}

impl AngularMover {
    pub fn new(position: Vec2, mass: f32) -> Self {
        Self {
            body: Body::new(position, mass),
            angle: 0.0,
            angle_velocity: 0.0,
            angle_acceleration: 0.0,
        }
    }
    pub fn apply_force(&mut self, force: Vec2) {
        self.body.apply_force(force);
    }

    pub fn update(&mut self) {
        // Relationship between linear accleration and angularation accleration
        self.angle_acceleration = self.body.acceleration.x / 10.0;
        self.body.update();

        self.angle_velocity += self.angle_acceleration;
        self.angle_velocity = self.angle_velocity.clamp(-0.1, 0.1);
        self.angle += self.angle_velocity;
    }

    pub fn draw(&self) {
        self.body.draw();
    }
}
