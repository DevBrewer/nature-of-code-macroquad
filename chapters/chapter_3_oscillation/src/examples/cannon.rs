use std::f32::consts::PI;

use macroquad::{
    color::{DARKGRAY, GRAY, GREEN, WHITE, YELLOW},
    input::{KeyCode, MouseButton, is_key_down, is_key_pressed, is_mouse_button_pressed},
    shapes::{draw_circle, draw_line},
    time::get_frame_time,
};
use runner::{Example, draw_info_panel, draw_world_border, world_height, world_width};
use vec_math::Vec2;

use crate::cannon_ball::CannonBall;

pub struct CannonSimulation {
    ball: CannonBall,
    cannon_origin: Vec2,
    gravity: Vec2,
    launch_speed: f32,
    launch_angle_deg: f32,
    restitution: f32,
    friction_coef: f32,
    has_fired: bool,
}

impl CannonSimulation {
    pub fn new() -> Self {
        let cannon_origin = Vec2::new(60.0, world_height() - 45.0);
        let gravity = Vec2::new(0.0, 450.0); // Constant downward gravity force

        Self {
            ball: CannonBall::new(cannon_origin),
            cannon_origin,
            gravity,
            launch_speed: 450.0,
            launch_angle_deg: 45.0,
            restitution: 0.70, // Coefficient of Restitution for wall/ground bounces
            friction_coef: 0.15, // Surface friction coefficient μ
            has_fired: false,
        }
    }

    /// Calculate launch impulse vector from speed and angle
    fn launch_impulse(&self) -> Vec2 {
        let radians = self.launch_angle_deg * (PI / 180.0);
        // Angle 0 degrees points straight right, 90 degrees points straight UP
        Vec2::new(
            self.launch_speed * radians.cos(),
            -self.launch_speed * radians.sin(),
        )
    }

    /// Fire or re-fire the cannon
    fn fire(&mut self) {
        let impulse = self.launch_impulse();
        self.ball = CannonBall::new(self.cannon_origin);
        self.ball.shoot(impulse);
        self.has_fired = true;
    }
}

impl Example for CannonSimulation {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        let dt = get_frame_time();

        // 1. Adjust Launch Controls (Power & Angle)
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            self.launch_speed = (self.launch_speed + 150.0 * dt).min(800.0);
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            self.launch_speed = (self.launch_speed - 150.0 * dt).max(100.0);
        }
        if is_key_down(KeyCode::A) {
            self.launch_angle_deg = (self.launch_angle_deg + 45.0 * dt).min(85.0);
        }
        if is_key_down(KeyCode::D) {
            self.launch_angle_deg = (self.launch_angle_deg - 45.0 * dt).max(5.0);
        }

        // 2. Fire Cannon on Space or Mouse Click
        if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
            self.fire();
        }

        // 3. Reset Cannon Ball position on 'R' key
        if is_key_pressed(KeyCode::R) {
            self.ball = CannonBall::new(self.cannon_origin);
            self.has_fired = false;
        }

        // 4. Update Physics if fired
        if self.has_fired {
            // Apply constant downward gravity force every frame
            self.ball.apply_force(self.gravity);

            // Apply surface friction when rolling/sliding on ground
            self.ball
                .apply_ground_friction(self.friction_coef, self.gravity.y);

            // Integrate linear & angular kinematics
            self.ball.update(dt);

            // Handle wall collisions with Coefficient of Restitution
            let ground_y = world_height() - 30.0;
            let ceiling_y = 0.0;
            self.ball
                .check_ground_and_ceiling(ground_y, ceiling_y, self.restitution);
            self.ball.check_walls(0.0, world_width(), self.restitution);
        }
    }

    fn draw(&self) {
        draw_world_border();

        let ground_y = world_height() - 30.0;

        // Draw Ground Surface
        draw_line(0.0, ground_y, world_width(), ground_y, 4.0, DARKGRAY);

        // Draw Trajectory Preview Line when not fired or adjusting
        let impulse = self.launch_impulse();
        let barrel_len = 35.0;
        let barrel_dir = impulse.normalized();
        let barrel_end = self.cannon_origin + barrel_dir * barrel_len;

        if !self.has_fired {
            // Draw dotted trajectory preview curve
            let mut preview_pos = self.cannon_origin;
            let mut preview_vel = impulse;
            let step_dt = 0.035;
            for _ in 0..40 {
                let next_pos = preview_pos + preview_vel * step_dt;
                preview_vel += self.gravity * step_dt;

                // Draw dots along the parabolic trajectory path
                draw_circle(preview_pos.x, preview_pos.y, 2.5, YELLOW);

                preview_pos = next_pos;
                if preview_pos.y >= ground_y
                    || preview_pos.x >= world_width()
                    || preview_pos.x <= 0.0
                {
                    break;
                }
            }
        }

        // Draw Cannon Barrel
        draw_line(
            self.cannon_origin.x,
            self.cannon_origin.y,
            barrel_end.x,
            barrel_end.y,
            12.0,
            GRAY,
        );
        draw_circle(self.cannon_origin.x, self.cannon_origin.y, 16.0, DARKGRAY);

        // Draw Cannonball
        self.ball.draw();

        // Telemetry & Info Panel
        let current_speed = if self.has_fired {
            self.ball.velocity.mag()
        } else {
            0.0
        };
        let current_spin = if self.has_fired {
            self.ball.angular_velocity
        } else {
            0.0
        };

        let power_str = format!("Power: {:.0} px/s [W/S / UP/DOWN]", self.launch_speed);
        let angle_str = format!("Angle: {:.0}° [A/D]", self.launch_angle_deg);
        let bounce_str = format!("Restitution (cr): {:.2}", self.restitution);
        let friction_str = format!("Friction (μ): {:.2}", self.friction_coef);
        let speed_str = format!("Speed: {:.1} px/s", current_speed);
        let spin_str = format!("Spin: {:.2} rad/s", current_spin);

        let lines = [
            ("EXERCISE 3.3: CANNONBALL WITH SPIN", WHITE),
            ("Press SPACE / Click to Fire | R to Reset", GREEN),
            (&power_str, YELLOW),
            (&angle_str, YELLOW),
            (&bounce_str, GRAY),
            (&friction_str, GRAY),
            (&speed_str, WHITE),
            (&spin_str, WHITE),
        ];

        draw_info_panel(10.0, world_height() - 150.0, 310.0, &lines);
    }
}
