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
    pub is_on_ground: bool,
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
            radius: 14.0,
            is_on_ground: false,
        }
    }

    /// Applies a linear force (F = ma => a = F/m)
    pub fn apply_force(&mut self, mut force: Vec2) {
        // Normal force N balances downward gravity while on the ground
        if self.is_on_ground && force.y > 0.0 {
            force.y = 0.0;
        }
        self.acceleration += force / self.mass;
    }

    /// Fires the ball with an initial launch impulse (velocity) and spin
    pub fn shoot(&mut self, impulse: Vec2) {
        self.velocity = impulse / self.mass;
        // Initial spin proportional to tangential launch speed
        self.angular_velocity = impulse.x * 0.02;
        self.is_on_ground = false;
    }

    /// Apply ground surface friction when in contact with the ground
    /// friction = -μ * N * v_hat
    pub fn apply_ground_friction(&mut self, mu: f32, gravity_mag: f32) {
        if self.is_on_ground {
            if self.velocity.x.abs() < 1.0 {
                // Completely stop linear motion and spin when friction brings ball to rest
                self.velocity = Vec2::ZERO;
                self.angular_velocity = 0.0;
            } else {
                let normal_force = self.mass * gravity_mag;
                let friction_mag = mu * normal_force;
                let friction_direction = -self.velocity.x.signum();
                let friction_force = Vec2::new(friction_direction * friction_mag, 0.0);

                self.apply_force(friction_force);

                // When rolling along the ground, angular velocity couples with linear velocity (v = ω * r)
                self.angular_velocity = self.velocity.x / self.radius;
            }
        }
    }

    /// Bounce off horizontal ground or ceiling using Coefficient of Restitution
    pub fn check_ground_and_ceiling(&mut self, ground_y: f32, ceiling_y: f32, restitution: f32) {
        // Ground Collision
        if self.position.y >= ground_y - self.radius {
            self.position.y = ground_y - self.radius;

            if self.velocity.y.abs() > 20.0 {
                // Bounce back up with energy loss (Coefficient of Restitution)
                self.velocity.y *= -restitution;
                // Ground impact torque kick
                self.angular_velocity += self.velocity.x * 0.03;
                self.is_on_ground = false;
            } else {
                // Stop vertical bouncing and settle on ground for rolling/sliding
                self.velocity.y = 0.0;
                self.is_on_ground = true;
            }
        } else {
            self.is_on_ground = false;
        }

        // Ceiling Collision
        if self.position.y <= ceiling_y + self.radius {
            self.position.y = ceiling_y + self.radius;
            self.velocity.y *= -restitution;
        }
    }

    /// Bounce off vertical left and right walls using Coefficient of Restitution
    pub fn check_walls(&mut self, min_x: f32, max_x: f32, restitution: f32) {
        // Left Wall
        if self.position.x <= min_x + self.radius {
            self.position.x = min_x + self.radius;
            self.velocity.x *= -restitution;
            self.angular_velocity *= -restitution;
        }

        // Right Wall
        if self.position.x >= max_x - self.radius {
            self.position.x = max_x - self.radius;
            self.velocity.x *= -restitution;
            self.angular_velocity *= -restitution;
        }
    }

    // =======================
    // Physics Integration
    // =======================
    pub fn update(&mut self, dt: f32) {
        self.update_motion(dt);
        self.update_rotation(dt);
    }

    fn update_motion(&mut self, dt: f32) {
        self.velocity += self.acceleration * dt;
        if self.is_on_ground {
            self.velocity.y = 0.0;
        }
        self.position += self.velocity * dt;
        self.acceleration = Vec2::ZERO;
    }

    fn update_rotation(&mut self, dt: f32) {
        self.angular_velocity += self.angular_acceleration * dt;
        self.angle += self.angular_velocity * dt;

        // Apply air rotational damping when flying
        if !self.is_on_ground {
            self.angular_velocity *= 0.995;
        }

        self.angular_acceleration = 0.0;
    }

    pub fn draw(&self) {
        // Outer body of the cannonball
        draw_circle_lines(self.position.x, self.position.y, self.radius, 2.0, WHITE);

        // Draw rotating line crosshair to visualize spin
        let heading = Vec2::new(self.radius, 0.0).rotate(self.angle);
        let start = self.position - heading;
        let end = self.position + heading;
        draw_line(start.x, start.y, end.x, end.y, 2.0, GREEN);
    }
}
