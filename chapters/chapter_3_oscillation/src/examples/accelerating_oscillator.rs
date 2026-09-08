use std::f32::consts::PI;

use macroquad::{
    color::{DARKGRAY, GRAY, GREEN, WHITE, YELLOW},
    input::{is_mouse_button_down, MouseButton},
    shapes::{draw_circle, draw_circle_lines, draw_line},
    time::get_frame_time,
};
use runner::{Example, draw_info_panel, draw_world_border, world_center, world_height};
use vec_math::Vec2;

/// Exercise 3.9: Incorporate angular acceleration into the Oscillator object.
///
/// In Example 3.7, the Oscillator moved with constant angular velocity:
///     `angle += angle_velocity`
///
/// In this exercise, we add angular acceleration to simulate forces/muscles:
///     `angle_velocity += angle_acceleration`
///     `angle += angle_velocity`
pub struct LegOscillator {
    pub angle: Vec2,
    pub angle_velocity: Vec2,
    pub angle_acceleration: Vec2,
    pub amplitude: Vec2,
    pub origin: Vec2,
    pub is_left: bool,
}

impl LegOscillator {
    pub fn new(origin: Vec2, phase: f32, is_left: bool) -> Self {
        // x is forward/backward swing, y is up/down lift (offset by PI/2 for an elliptical step)
        let angle = Vec2::new(phase, phase + PI * 0.5);
        let angle_velocity = Vec2::new(0.02, 0.02);
        let angle_acceleration = Vec2::ZERO;

        // Legs reach outward to the left (-x) or right (+x)
        let amplitude = if is_left {
            Vec2::new(-50.0, 20.0)
        } else {
            Vec2::new(50.0, 20.0)
        };

        Self {
            angle,
            angle_velocity,
            angle_acceleration,
            amplitude,
            origin,
            is_left,
        }
    }

    pub fn update(&mut self, dt: f32, acceleration_boost: f32) {
        // Exercise 3.9: Set angular acceleration
        // Base natural muscle oscillation + extra acceleration when user clicks
        let natural_accel = -0.001 * self.angle.x.sin();
        let applied_accel = natural_accel + acceleration_boost;

        self.angle_acceleration = Vec2::new(applied_accel, applied_accel);

        // 1. Angular velocity increases by angular acceleration
        self.angle_velocity += self.angle_acceleration * dt;

        // Slight damping so velocity doesn't grow infinitely
        self.angle_velocity *= 0.99;

        // Clamp speed to reasonable visual bounds
        self.angle_velocity.x = self.angle_velocity.x.clamp(-0.08, 0.08);
        self.angle_velocity.y = self.angle_velocity.y.clamp(-0.08, 0.08);

        // 2. Phase angle increases by angular velocity
        self.angle += self.angle_velocity * dt;
    }

    pub fn show(&self) {
        // Calculate foot position from origin + harmonic oscillation
        let foot_x = self.origin.x + self.angle.x.sin() * self.amplitude.x;
        let foot_y = self.origin.y + self.angle.y.sin() * self.amplitude.y;

        // Midpoint knee joint bent outward
        let knee_outward = if self.is_left { -25.0 } else { 25.0 };
        let knee_x = (self.origin.x + foot_x) * 0.5 + knee_outward;
        let knee_y = (self.origin.y + foot_y) * 0.5 - 10.0;

        // Draw leg segments: hip -> knee -> foot
        draw_line(self.origin.x, self.origin.y, knee_x, knee_y, 2.0, GRAY);
        draw_line(knee_x, knee_y, foot_x, foot_y, 2.0, WHITE);

        // Draw joints and foot
        draw_circle(knee_x, knee_y, 3.0, GRAY);
        draw_circle(foot_x, foot_y, 5.0, WHITE);
    }
}

pub struct AcceleratingOscillatorExample {
    legs: Vec<LegOscillator>,
    body_center: Vec2,
}

impl AcceleratingOscillatorExample {
    pub fn new() -> Self {
        let center = world_center();
        let num_leg_pairs = 4;
        let leg_spacing = 35.0;
        let start_y = center.y - (num_leg_pairs as f32 * leg_spacing) * 0.5;

        let mut legs = Vec::new();

        for i in 0..num_leg_pairs {
            let y = start_y + (i as f32) * leg_spacing;

            // Phase offset along body (metachronal wave)
            let phase = i as f32 * 0.6;

            // Left leg (attaches to left side of body)
            let left_origin = Vec2::new(center.x - 15.0, y);
            legs.push(LegOscillator::new(left_origin, phase, true));

            // Right leg (attaches to right side of body, out-of-phase by PI for alternating steps)
            let right_origin = Vec2::new(center.x + 15.0, y);
            legs.push(LegOscillator::new(right_origin, phase + PI, false));
        }

        Self {
            legs,
            body_center: center,
        }
    }
}

impl Example for AcceleratingOscillatorExample {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        let dt = get_frame_time() * 60.0;

        // When mouse is clicked, inject angular acceleration to make the insect scurry
        let mouse_down = is_mouse_button_down(MouseButton::Left);
        let accel_boost = if mouse_down { 0.002 } else { 0.0 };

        for leg in self.legs.iter_mut() {
            leg.update(dt, accel_boost);
        }
    }

    fn draw(&self) {
        draw_world_border();

        // 1. Draw insect body down the center
        let num_pairs = self.legs.len() / 2;
        let spacing = 35.0;
        let start_y = self.body_center.y - (num_pairs as f32 * spacing) * 0.5;

        // Head
        draw_circle(self.body_center.x, start_y - 20.0, 16.0, DARKGRAY);
        draw_circle_lines(self.body_center.x, start_y - 20.0, 16.0, 2.0, WHITE);

        // Body segments
        for i in 0..num_pairs {
            let y = start_y + (i as f32) * spacing;
            draw_circle(self.body_center.x, y, 14.0, DARKGRAY);
            draw_circle_lines(self.body_center.x, y, 14.0, 1.5, GRAY);
        }

        // 2. Draw each leg oscillator
        for leg in &self.legs {
            leg.show();
        }

        // 3. Info panel explaining the tutorial concept
        let first_leg = &self.legs[0];
        let acc_text = format!("angle_acceleration: {:.4}", first_leg.angle_acceleration.x);
        let vel_text = format!("angle_velocity:     {:.3}", first_leg.angle_velocity.x);

        let lines = [
            ("EXERCISE 3.9: ACCELERATING OSCILLATOR", WHITE),
            ("angle_velocity += angle_acceleration * dt", GREEN),
            ("angle += angle_velocity * dt", GREEN),
            (&acc_text, GRAY),
            (&vel_text, GRAY),
            ("Click & hold to accelerate legs!", YELLOW),
        ];

        draw_info_panel(10.0, world_height() - 120.0, 320.0, &lines);
    }
}
