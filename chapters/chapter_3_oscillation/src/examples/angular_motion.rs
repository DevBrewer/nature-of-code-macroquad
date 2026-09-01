use std::f32::consts::PI;

use macroquad::{
    color::{GRAY, GREEN, WHITE, YELLOW},
    input::{MouseButton, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released},
    shapes::{draw_circle, draw_line},
    time::get_frame_time,
};
use runner::{
    Example, draw_info_panel, mouse_world_position, world_center, world_height, world_width,
};
use vec_math::Vec2;

/// Demonstrates angular velocity motion by allowing the user to grab
/// and spin a baton with the mouse
/// While dragging:
/// mouse movement -> angular velocity -> angle
/// After releasing:
///  angular velocity continues to rotate the baton,
/// while damping gradually slows it down.
pub struct AngularMotion {
    angle: f32,
    angular_velocity: f32,
    angular_acceleration: f32,
    dragging: bool,
    // mouse angle to determine how much the baton was rotated
    previous_mouse_angle: f32,
}

impl AngularMotion {
    pub fn new() -> Self {
        Self {
            angle: 0.0,
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            dragging: false,
            previous_mouse_angle: 0.0,
        }
    }

    /// Calculate the angle from the baton center to the mouse
    /// atan2() gives us the direction
    fn mouse_angle() -> f32 {
        let mouse = mouse_world_position();
        let center = world_center();
        let direction = mouse - center;

        //tanΘ = sinΘ/cosΘ
        direction.y.atan2(direction.x)
    }

    /// Keeps an angle inside the range [-PI, PI].
    fn normalize_angle(angle: f32) -> f32 {
        (angle + PI).rem_euclid(2.0 * PI) - PI
    }

    /// Begins a mouse drag
    /// We store the current mouse angle
    fn begin_drag(&mut self) {
        self.dragging = true;
        self.previous_mouse_angle = Self::mouse_angle();

        self.angular_acceleration = 0.0;
    }

    /// Update the baton while the mouse is being dragged.
    fn drag(&mut self, delta_time: f32) {
        let mouse_angle = Self::mouse_angle();

        // Angular displacement since previous frame
        let delta_angle = Self::normalize_angle(mouse_angle - self.previous_mouse_angle);

        // Angular velocity is angular displacement divided by time
        let angular_velocity = delta_angle / delta_time.max(f32::EPSILON);

        // Limit the velocity so that baton does not spin at an unresonable speed.
        self.angular_velocity = angular_velocity.clamp(-8.0, 8.0);
        self.angle += delta_angle;

        // Angle for the next frame
        self.previous_mouse_angle = mouse_angle;
    }

    fn release_drag(&mut self) {
        self.dragging = false;
    }
}

impl Example for AngularMotion {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        let delta_time = get_frame_time();

        // Start dragging
        if is_mouse_button_pressed(MouseButton::Left) {
            self.begin_drag();
        }

        // Update the baton angular movement
        if self.dragging && is_mouse_button_down(MouseButton::Left) {
            self.drag(delta_time);
        }

        // Released
        if self.dragging && is_mouse_button_released(MouseButton::Left) {
            self.release_drag();
        }

        // Once release, the baton continues rotating according
        // to its angular velocity.
        if !self.dragging {
            self.angle += self.angular_velocity * delta_time;

            // Damping (rotation friction)
            self.angular_velocity *= 0.98_f32.powf(delta_time * 60.0);
        }
    }

    fn draw(&self) {
        let center = world_center();

        // Half-length of the baton
        let length = world_width() * 0.25;

        // Start with horizontal vector and rotate it by the current angle.
        let baton = Vec2::new(length, 0.0).rotate(self.angle);

        let start = center - baton;
        let end = center + baton;

        // Draw baton
        draw_line(start.x, start.y, end.x, end.y, 4.0, WHITE);

        // Draw Center circle
        draw_circle(center.x, center.y, 5.0, GRAY);

        // Batons endpoint
        draw_circle(start.x, start.y, 8.0, WHITE);
        draw_circle(end.x, end.y, 8.0, WHITE);

        let angle_deg = (self.angle.to_degrees() % 360.0 + 360.0) % 360.0;
        let angle_str = format!("Angle (θ): {:.2} rad ({:.1}°)", self.angle, angle_deg);
        let vel_str = format!("Angular Velocity (ω): {:.2} rad/s", self.angular_velocity);
        let accel_str = format!("Angular Accel (α): {:.2} rad/s²", self.angular_acceleration);
        let status_str = if self.dragging {
            "Status: [MOUSE DRAGGING]"
        } else {
            "Status: [FREE SPIN DAMPING]"
        };

        let lines = [
            ("EXAMPLE 3.2.1: ANGULAR MOTION", WHITE),
            ("Click & Drag mouse around center to spin", GREEN),
            (&angle_str, YELLOW),
            (&vel_str, YELLOW),
            (&accel_str, GRAY),
            (status_str, if self.dragging { GREEN } else { GRAY }),
        ];

        draw_info_panel(10.0, world_height() - 135.0, 310.0, &lines);
    }
}
