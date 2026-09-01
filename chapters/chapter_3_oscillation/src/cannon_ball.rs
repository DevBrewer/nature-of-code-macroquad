use macroquad::{
    color::{GREEN, WHITE},
    shapes::{draw_circle_lines, draw_line},
};
use vec_math::Vec2;

pub struct CannonBall {
    // Linear Motion
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,

    // Angular Motion
    pub angle: f32,
    pub angular_velocity: f32,
    pub angular_acceleration: f32,

    pub radius: f32,
}

impl CannonBall {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            mass: 1.0,

            angle: 0.0,
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            radius: 12.0,
        }
    }

    /// Applies a linear force (F = ma => a = F/m)
    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force / self.mass;
    }

    /// Fires the ball with an initial launch force (impulse) and spin
    pub fn shoot(&mut self, force: Vec2) {
        // One-time sudden launch force
        self.apply_force(force);

        // Give it an initial spin (angular velocity) proportional to launch power
        self.angular_velocity = force.x * 0.005;
    }

    // =======================
    // Physics Integration
    // =======================
    pub fn update(&mut self, dt: f32) {
        self.update_motion(dt);
        self.update_rotation(dt);
    }

    fn update_motion(&mut self, dt: f32) {
        // Integrate velocity
        self.velocity += self.acceleration * dt;

        // Integrate position
        self.position += self.velocity * dt;

        // Reset acceleration
        self.acceleration = Vec2::ZERO;
    }

    fn update_rotation(&mut self, dt: f32) {
        // Integrate angular velocity
        self.angular_velocity += self.angular_acceleration * dt;

        // Integrate angle
        self.angle += self.angular_velocity * dt;

        // Damping / rotational friction with air
        self.angular_velocity *= 0.99;

        // Reset angular acceleration
        self.angular_acceleration = 0.0;
    }

    pub fn draw(&self) {
        // Outer body of the cannonball
        draw_circle_lines(self.position.x, self.position.y, self.radius, 2.0, WHITE);

        // Draw rotating crosshair line to visualize rotation (spin)
        let heading = Vec2::new(self.radius, 0.0).rotate(self.angle);
        let start = self.position - heading;
        let end = self.position + heading;
        draw_line(start.x, start.y, end.x, end.y, 2.0, GREEN);
    }
}
