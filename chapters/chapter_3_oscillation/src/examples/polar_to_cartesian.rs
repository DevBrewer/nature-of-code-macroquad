#![allow(dead_code)]

use std::collections::VecDeque;

use macroquad::{
    color::{Color, GRAY, GREEN, RED, WHITE, YELLOW},
    input::{KeyCode, is_key_down},
    shapes::{draw_circle, draw_circle_lines, draw_line},
    time::get_frame_time,
};
use runner::{Example, draw_axes, draw_info_panel, draw_world_border, world_center, world_height};
use vec_math::Vec2;

/// Demonstrates Example 3.4: Converting Polar Coordinates (r, θ) to Cartesian Coordinates (x, y).
///
/// Polar coordinates define position using:
/// - `r`: Radial distance from the origin (radius)
/// - `theta` (θ): Angle of rotation in radians
///
/// Cartesian coordinates define position using:
/// - `x = r * cos(theta)`
/// - `y = r * sin(theta)`
/// Or equivalently using vector math: `position = origin + Vec2::from_angle(theta) * r`
pub struct PolarToCartesian {
    radius: f32,
    theta: f32,
    angular_velocity: f32,
    trail: VecDeque<Vec2>,
}

impl PolarToCartesian {
    pub fn new() -> Self {
        Self {
            radius: 140.0,
            theta: 0.0,
            angular_velocity: 1.5, // Radians per second
            trail: VecDeque::with_capacity(120),
        }
    }
}

impl Example for PolarToCartesian {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        let dt = get_frame_time();

        // Interactive controls for radius (r) and angular velocity (ω)
        if is_key_down(KeyCode::W) {
            self.radius = (self.radius + 60.0 * dt).min(250.0);
        }
        if is_key_down(KeyCode::S) {
            self.radius = (self.radius - 60.0 * dt).max(20.0);
        }
        if is_key_down(KeyCode::D) {
            self.angular_velocity += 1.0 * dt;
        }
        if is_key_down(KeyCode::A) {
            self.angular_velocity -= 1.0 * dt;
        }

        // Increment theta by angular velocity over frame time: θ = θ + ω * dt
        self.theta += self.angular_velocity * dt;

        // Keep theta normalized within [0, 2π) for clean telemetry display
        if self.theta > std::f32::consts::TAU {
            self.theta -= std::f32::consts::TAU;
        } else if self.theta < 0.0 {
            self.theta += std::f32::consts::TAU;
        }

        // Convert Polar (r, θ) to Cartesian (x, y) relative to world origin center
        let origin = world_center();
        let x = self.radius * self.theta.cos();
        let y = self.radius * self.theta.sin();
        let cartesian_pos = origin + Vec2::new(x, y);

        // Store trail points for visual orbit path
        self.trail.push_back(cartesian_pos);
        if self.trail.len() > 100 {
            self.trail.pop_front();
        }
    }

    fn draw(&self) {
        draw_world_border();
        draw_axes();

        let origin = world_center();

        // Fundamental Polar to Cartesian Conversion formulas:
        // x = r * cos(θ)
        // y = r * sin(θ)
        let x_offset = self.radius * self.theta.cos();
        let y_offset = self.radius * self.theta.sin();
        let point = origin + Vec2::new(x_offset, y_offset);

        // Draw polar radius circular path
        draw_circle_lines(
            origin.x,
            origin.y,
            self.radius,
            1.5,
            Color::new(0.3, 0.4, 0.5, 0.5),
        );

        // Draw circular orbital trail
        for (i, p) in self.trail.iter().enumerate() {
            let alpha = (i + 1) as f32 / self.trail.len().max(1) as f32;
            draw_circle(p.x, p.y, 2.5, Color::new(0.4, 0.8, 1.0, alpha * 0.6));
        }

        // Draw Cartesian component projection lines (Triangle legs: X & Y offsets)
        // Horizontal X component line: origin.x -> point.x
        draw_line(origin.x, origin.y, point.x, origin.y, 2.0, RED);
        // Vertical Y component line: origin.y -> point.y
        draw_line(point.x, origin.y, point.x, point.y, 2.0, GREEN);

        // Draw Polar Radial Vector (Hypotenuse r)
        draw_line(origin.x, origin.y, point.x, point.y, 3.0, WHITE);

        // Draw Origin and Cartesian Point
        draw_circle(origin.x, origin.y, 5.0, YELLOW);
        draw_circle(point.x, point.y, 8.0, Color::new(0.2, 0.8, 1.0, 1.0));
        draw_circle(point.x, point.y, 4.0, WHITE);

        // Format Telemetry Info Panel
        let angle_deg = self.theta.to_degrees();
        let polar_str = format!(
            "Polar (r, theta): r = {:.1} px, theta = {:.2} rad ({:.1} deg)",
            self.radius, self.theta, angle_deg
        );
        let cart_str = format!("Cartesian (x, y): x = {:.1}, y = {:.1}", x_offset, y_offset);
        let formula_x = format!(
            "Formula X: x = r * cos(theta) = {:.1} * {:.2} = {:.1}",
            self.radius,
            self.theta.cos(),
            x_offset
        );
        let formula_y = format!(
            "Formula Y: y = r * sin(theta) = {:.1} * {:.2} = {:.1}",
            self.radius,
            self.theta.sin(),
            y_offset
        );
        let speed_str = format!(
            "Angular Velocity (w): {:.2} rad/s [A / D]",
            self.angular_velocity
        );
        let radius_str = format!("Radius (r): {:.1} px [W / S]", self.radius);

        let lines = [
            ("EXAMPLE 3.4: POLAR TO CARTESIAN", WHITE),
            ("x = r * cos(theta) | y = r * sin(theta)", GREEN),
            (&polar_str, YELLOW),
            (&cart_str, Color::new(0.4, 0.8, 1.0, 1.0)),
            (&formula_x, RED),
            (&formula_y, GREEN),
            (&speed_str, GRAY),
            (&radius_str, GRAY),
        ];

        draw_info_panel(10.0, world_height() - 170.0, 360.0, &lines);
    }
}
