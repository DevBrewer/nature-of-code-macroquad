#![allow(dead_code)]

use macroquad::{
    color::{GRAY, GREEN, LIGHTGRAY, RED, WHITE, YELLOW},
    input::{KeyCode, is_key_down},
    math::vec2,
    shapes::{draw_circle, draw_line, draw_triangle},
    time::get_frame_time,
};
use runner::{
    Example, draw_info_panel, draw_vector, draw_world_border, world_center, world_height,
    world_width,
};
use vec_math::Vec2;

/// Vehicle object that can be driven around the screen using Arrow Keys or WASD.
/// Accelerates in the input direction, applies friction when coasting,
/// and rotates to point in the direction of current motion.
pub struct Vehicle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub top_speed: f32,
    pub acceleration_strength: f32,
    pub angle: f32,
}

impl Vehicle {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            top_speed: 350.0,             // Maximum speed (px/s)
            acceleration_strength: 600.0, // Steering acceleration force (px/s^2)
            angle: 0.0,
        }
    }

    /// Reads keyboard inputs (Arrow Keys or WASD) and returns a normalized
    /// vector representing the steering acceleration direction.
    fn handle_input(&self) -> Vec2 {
        let mut steer = Vec2::ZERO;

        if is_key_down(KeyCode::A) {
            steer.x -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            steer.x += 1.0;
        }
        if is_key_down(KeyCode::W) {
            steer.y -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            steer.y += 1.0;
        }

        if steer.mag_sq() > 0.0 {
            steer.normalized()
        } else {
            Vec2::ZERO
        }
    }

    /// Updates vehicle kinematics: input acceleration, friction/damping,
    /// velocity limiting, position integration, orientation, and wall bounding.
    pub fn update(&mut self, dt: f32) {
        let steer_dir = self.handle_input();

        if steer_dir.mag_sq() > 0.0 {
            // Apply steering acceleration in input direction
            self.acceleration = steer_dir * self.acceleration_strength;
        } else {
            // Apply frame-rate independent friction (coasting damping) when no key is pressed
            self.velocity *= 0.98_f32.powf(dt * 60.0);
            self.acceleration = Vec2::ZERO;
        }

        // Euler Integration
        self.velocity += self.acceleration * dt;
        self.velocity.limit(self.top_speed);
        self.position += self.velocity * dt;

        // Calculate heading angle pointing in direction of current velocity vector
        if self.velocity.mag_sq() > 0.001 {
            self.angle = self.velocity.heading();
        }

        // Clamp vehicle within world canvas (accounting for nose/tail radius)
        self.check_edges();

        // Reset acceleration for next frame
        self.acceleration = Vec2::ZERO;
    }

    /// Clamps position to keep entire vehicle body (including front tip) within bounds.
    fn check_edges(&mut self) {
        let margin = 20.0; // Margin accounts for front nose tip extending 18px from center
        let min_x = margin;
        let max_x = world_width() - margin;
        let min_y = margin;
        let max_y = world_height() - margin;

        if self.position.x < min_x {
            self.position.x = min_x;
            self.velocity.x = 0.0;
        } else if self.position.x > max_x {
            self.position.x = max_x;
            self.velocity.x = 0.0;
        }

        if self.position.y < min_y {
            self.position.y = min_y;
            self.velocity.y = 0.0;
        } else if self.position.y > max_y {
            self.position.y = max_y;
            self.velocity.y = 0.0;
        }
    }

    /// Renders the vehicle body rotated by heading angle.
    pub fn draw(&self) {
        // Vehicle local geometry (facing right along X-axis when angle = 0)
        let local_tip = Vec2::new(18.0, 0.0);
        let local_back_left = Vec2::new(-12.0, -8.0);
        let local_back_right = Vec2::new(-12.0, 8.0);
        let local_indent = Vec2::new(-6.0, 0.0);

        // Transform local vertices to world space using rotation & position offset
        let tip = self.position + local_tip.rotate(self.angle);
        let back_left = self.position + local_back_left.rotate(self.angle);
        let back_right = self.position + local_back_right.rotate(self.angle);
        let indent = self.position + local_indent.rotate(self.angle);

        // Draw filled vehicle chassis
        draw_triangle(
            vec2(tip.x, tip.y),
            vec2(back_left.x, back_left.y),
            vec2(indent.x, indent.y),
            GRAY,
        );
        draw_triangle(
            vec2(tip.x, tip.y),
            vec2(indent.x, indent.y),
            vec2(back_right.x, back_right.y),
            LIGHTGRAY,
        );

        // Draw vehicle outline
        draw_line(tip.x, tip.y, back_left.x, back_left.y, 2.0, WHITE);
        draw_line(back_left.x, back_left.y, indent.x, indent.y, 2.0, WHITE);
        draw_line(indent.x, indent.y, back_right.x, back_right.y, 2.0, WHITE);
        draw_line(back_right.x, back_right.y, tip.x, tip.y, 2.0, WHITE);

        // Draw pivot center dot
        draw_circle(self.position.x, self.position.y, 3.0, RED);

        // Render velocity vector arrow
        draw_vector(self.position, self.velocity * 0.25, GREEN);
    }
}

pub struct VehicleSimulation {
    vehicle: Vehicle,
}

impl VehicleSimulation {
    pub fn new() -> Self {
        Self {
            vehicle: Vehicle::new(world_center()),
        }
    }
}

impl Example for VehicleSimulation {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        let dt = get_frame_time();
        self.vehicle.update(dt);
    }

    fn draw(&self) {
        draw_world_border();

        self.vehicle.draw();

        let speed = self.vehicle.velocity.mag();
        let angle_deg = (self.vehicle.angle.to_degrees() % 360.0 + 360.0) % 360.0;

        let speed_str = format!(
            "Speed: {:.1} px/s (Max: {:.0})",
            speed, self.vehicle.top_speed
        );
        let heading_str = format!(
            "Heading (angle): {:.2} rad ({:.1}°)",
            self.vehicle.angle, angle_deg
        );

        let lines = [
            ("EXERCISE 3.4: VEHICLE STEERING SIMULATION", WHITE),
            ("Drive with Arrow Keys or WASD", GREEN),
            (&speed_str, YELLOW),
            (&heading_str, YELLOW),
        ];

        draw_info_panel(10.0, world_height() - 110.0, 340.0, &lines);
    }
}
