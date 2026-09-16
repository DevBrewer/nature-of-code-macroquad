use std::f32::consts::FRAC_PI_4;

use macroquad::{
    color::{GREEN, RED, WHITE},
    input::{MouseButton, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released},
    shapes::{draw_circle, draw_line},
    time::get_frame_time,
};
use runner::{Example, draw_vector, draw_world_border, mouse_world_position, world_width};
use vec_math::Vec2;

const GRAVITY: f32 = 450.0; // scaled for screen space pixel length
const DRAG_COEFFICIENT: f32 = 0.2; // DragForce
const BOB_RADIUS: f32 = 24.0;
// Add a scale constant for drawing the force vector
const VECTOR_VISUAL_SCALE: f32 = 0.12;

pub struct Pendulum {
    pub length: f32,
    pub radius: f32,
    pub angular_velocity: f32,
    pub angular_acceleration: f32,
    pub angle: f32,
    pub bob: Vec2,
    pub pivot: Vec2,
    pub is_dragging: bool,
}

impl Pendulum {
    pub fn new(position: Vec2, length: f32) -> Self {
        let mut pendulum = Self {
            length,
            radius: BOB_RADIUS,
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            angle: FRAC_PI_4,
            pivot: position,
            bob: Vec2::ZERO,
            is_dragging: false,
        };

        // pre-calculated initial bob position so it doesn't flash at (0,0) on frame 1
        pendulum.update_bob_position();
        pendulum
    }

    pub fn mouse_press(&mut self, mouse: Vec2) {
        let mouse_pos = mouse - self.bob;
        let distance = mouse_pos.mag();
        if distance < self.radius {
            self.is_dragging = true;
            self.angular_velocity = 0.0;
            self.angular_acceleration = 0.0;
        }
    }

    pub fn mouse_drag(&mut self, mouse: Vec2) {
        if self.is_dragging {
            let diff = mouse - self.pivot;
            self.angle = diff.x.atan2(diff.y);
            self.update_bob_position();
            self.angular_velocity = 0.0;
            self.angular_acceleration = 0.0;
        }
    }

    pub fn mouse_release(&mut self) {
        self.is_dragging = false;
    }

    fn update_bob_position(&mut self) {
        let (sin_angle, cos_angle) = self.angle.sin_cos();

        self.bob.x = self.pivot.x + self.length * sin_angle;
        self.bob.y = self.pivot.y + self.length * cos_angle;
    }

    pub fn update(&mut self, dt: f32) {
        // prevent updates if dt is missing or frozen to avoid NaN bugs, or if being dragged
        if dt <= 0.0 || self.is_dragging {
            return;
        }

        // Position:     s = L * Θ      =>  Θ = s / L
        // Velocity:     v = L * ω      =>  ω = v / L
        // Acceleration: a = L * α      =>  α = a / L
        // As per Newton's second law F = m a
        // Force acting on the bob is gravity = mg,
        // Breaking the components x: -mg * sin(Θ) (minus sign because of restoring force)
        // substitute second laws -mg * Sin(Θ) = m * ɑ/L
        // (-g * Sin(Θ))/L = ɑ;

        self.angular_acceleration = -GRAVITY * self.angle.sin() / self.length;

        // Semi-implicit Euler Intergration
        self.angular_velocity += self.angular_acceleration * dt;
        self.angular_velocity *= f32::exp(-DRAG_COEFFICIENT * dt);

        self.angle += self.angular_velocity * dt;

        // Apply polar to cartesian conversion to update the bob's coordinates
        self.update_bob_position();
    }

    pub fn show(&self) {
        // Draw the pendulum arm
        draw_line(
            self.pivot.x,
            self.pivot.y,
            self.bob.x,
            self.bob.y,
            2.0,
            WHITE,
        );

        // Draw pivot
        draw_circle(self.pivot.x, self.pivot.y, 6.0, WHITE);

        // Draw Bob
        draw_circle(self.bob.x, self.bob.y, self.radius, WHITE);

        // Compute Trig values for vector headings
        let (sin_angle, cos_angle) = self.angle.sin_cos();

        // Draw Gravity Vector (Fg = mg) -> Points Straight Down
        // We use mass * GRAVITY to get the true force magnitude, then scale it for visibility
        let gravity_force_magnitude = GRAVITY * VECTOR_VISUAL_SCALE;

        let fg_vector = Vec2::new(0.0, gravity_force_magnitude);
        draw_vector(self.bob, fg_vector, RED);

        // Draw Tangential Displacement Vector (s) -> Points along the arc path
        // Perpendicular tangent heading to a circle in screen space: (cos, -sin)
        let displacement_direction = Vec2::new(cos_angle, -sin_angle);

        // Scale the arrow based on the current angular velocity to show movement magnitude
        let displacement_magnitude = self.angular_velocity * self.length * 0.2;

        let s_vector = displacement_direction * displacement_magnitude;

        draw_vector(self.bob, s_vector, GREEN);
    }
}

pub struct SimplePendulumExample {
    pendulum: Pendulum,
}

impl SimplePendulumExample {
    pub fn new() -> Self {
        let length = 200.0;
        let position = Vec2::new(world_width() * 0.5, 100.0);

        Self {
            pendulum: Pendulum::new(position, length),
        }
    }
}

impl Example for SimplePendulumExample {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        let dt = get_frame_time();
        let mouse = mouse_world_position();

        // Get the mouse coordinates
        if is_mouse_button_pressed(MouseButton::Left) {
            self.pendulum.mouse_press(mouse);
        }

        // Drag bob
        if is_mouse_button_down(MouseButton::Left) {
            self.pendulum.mouse_drag(mouse);
        }

        // stop dragging
        if is_mouse_button_released(MouseButton::Left) {
            self.pendulum.mouse_drag(mouse);
            self.pendulum.mouse_release();
        }
        if !self.pendulum.is_dragging {
            self.pendulum.update(dt);
        }
    }

    fn draw(&self) {
        draw_world_border();
        self.pendulum.show();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_drag_and_release_synchronization() {
        let pivot = Vec2::new(200.0, 100.0);
        let length = 200.0;
        let mut pendulum = Pendulum::new(pivot, length);

        // Initial angle is FRAC_PI_4 (~0.7853)
        assert!((pendulum.angle - FRAC_PI_4).abs() < 1e-4);

        // Simulate mouse press on the bob
        pendulum.mouse_press(pendulum.bob);
        assert!(pendulum.is_dragging);

        // Drag the bob to the left side: pivot.x - 150.0, pivot.y + 150.0 (-PI/4)
        let drag_target = Vec2::new(pivot.x - 150.0, pivot.y + 150.0);
        pendulum.mouse_drag(drag_target);

        // Expected angle should be -PI/4 (~ -0.7853)
        let expected_angle = -PI / 4.0;
        assert!(
            (pendulum.angle - expected_angle).abs() < 1e-4,
            "Angle should update to drag direction: expected {}, got {}",
            expected_angle,
            pendulum.angle
        );

        // Bob position should be synchronized with the new angle on the pendulum arc
        let (sin_a, cos_a) = pendulum.angle.sin_cos();
        assert!((pendulum.bob.x - (pivot.x + length * sin_a)).abs() < 1e-4);
        assert!((pendulum.bob.y - (pivot.y + length * cos_a)).abs() < 1e-4);

        // Release the mouse
        pendulum.mouse_release();
        assert!(!pendulum.is_dragging);

        // Run an update step: angle must start from the released angle (-PI/4), not snap back to FRAC_PI_4
        let angle_before_step = pendulum.angle;
        pendulum.update(0.016);

        // At negative angle, restoring acceleration is positive (accelerating towards 0)
        assert!(
            pendulum.angular_acceleration > 0.0,
            "Restoring acceleration should swing back towards zero"
        );
        // Angle should have moved towards 0 from -PI/4, NOT jumped to positive FRAC_PI_4
        assert!(
            pendulum.angle > angle_before_step && pendulum.angle < 0.0,
            "Angle should smoothly swing from released position: was {}, now {}",
            angle_before_step,
            pendulum.angle
        );
    }

    #[test]
    fn test_gravity_vector_scale() {
        let gravity_force_magnitude = GRAVITY * VECTOR_VISUAL_SCALE;
        // Verify gravity force magnitude is properly scaled (e.g. ~54px, neat and not overwhelming)
        assert!(
            gravity_force_magnitude > 40.0 && gravity_force_magnitude < 80.0,
            "Gravity vector magnitude {} should be reasonable and cleanly scaled",
            gravity_force_magnitude
        );
    }
}
