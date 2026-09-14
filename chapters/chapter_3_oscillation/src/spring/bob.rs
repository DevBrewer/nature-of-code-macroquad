#![allow(dead_code)]
use macroquad::{
    color::Color,
    shapes::{draw_circle, draw_circle_lines},
};
use vec_math::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Bob {
    pub mass: f32,
    pub radius: f32,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub damping: f32,
    pub position: Vec2,
    pub is_dragging: bool,
}

impl Bob {
    pub fn new(position: Vec2, mass: f32) -> Self {
        Self {
            mass,
            radius: mass * 2.0,
            velocity: Vec2::new(0.0, 0.0),
            acceleration: Vec2::new(0.0, 0.0),
            damping: 0.98,
            position,
            is_dragging: false,
        }
    }

    // Newton's second law
    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force / self.mass
    }

    // Euler Intergration
    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        self.velocity *= self.damping;
        self.position += self.velocity;

        self.acceleration = Vec2::ZERO;
    }

    pub fn handle_mouse_press(&mut self, mouse: Vec2) {
        if (mouse - self.position).mag() < self.radius {
            self.is_dragging = true;
        }
    }

    pub fn handle_mouse_drag(&mut self, mouse: Vec2) {
        if self.is_dragging {
            self.position = mouse;
            self.velocity = Vec2::ZERO
        }
    }

    pub fn handle_mouse_release(&mut self) {
        self.is_dragging = false;
    }

    pub fn show(&self) {
        // Draw Bob

        draw_circle_lines(
            self.position.x,
            self.position.y,
            self.radius,
            2.0,
            Color::new(0.0, 0.0, 0.0, 0.4),
        );

        // Radial Gradient Sphere (concentric circles)
        // #f43f53 Bright red
        let inner_color = Color::from_hex(0xf43f5e); // Bright rose
        let outer_color = Color::from_hex(0x881337); // Deep crimson

        let steps = 16; // Higher number = smoother gradient
        for i in 0..steps {
            let t = i as f32 / (steps - 1) as f32;
            // u goes from 1.0(outer edge) down to 0.0 (inner core)
            let u = 1.0 - t;

            // Interpolate radius and offset the focal point towards top-left
            let current_radius = 4.0 + (self.radius - 4.0) * u;
            let offset_x = self.position.x - 8.0 * (1.0 - u);
            let offset_y = self.position.y - 8.0 * (1.0 - u);

            // Blend colors from other outer to inner
            let color = Self::lerp_color(outer_color, inner_color, u);
            draw_circle(offset_x, offset_y, current_radius, color);
        }
    }

    fn lerp_color(start: Color, end: Color, t: f32) -> Color {
        // Clamp t between 0.0 and 1.0 to prevent overshooting colors
        let t = t.clamp(0.0, 1.0);

        Color::new(
            start.r + (end.r - start.r) * t,
            start.g + (end.g - start.g) * t,
            start.b + (end.b - start.b) * t,
            start.a + (end.a - start.a) * t,
        )
    }
}
