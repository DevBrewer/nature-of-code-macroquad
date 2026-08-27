use crate::mover::Mover;
use macroquad::{
    color::{BLUE, GRAY, WHITE},
    input::{MouseButton, is_mouse_button_down},
    shapes::draw_circle,
};
use runner::{
    Example, draw_info_panel, draw_vector, draw_world_border, world_center, world_height,
};
use vec_math::Vec2;

pub struct ForceExample {
    mover: Mover,
}

impl ForceExample {
    pub fn new() -> Self {
        Self {
            mover: Mover::new(world_center(), 1.0),
        }
    }
}

impl Example for ForceExample {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        // Apply a constant force every frame.
        // F = ma => a = F/m
        // Wind is applied only while the mouse button is held.
        if is_mouse_button_down(MouseButton::Left) {
            let wind = Vec2::new(0.1, 0.0);
            self.mover.apply_force(wind);
        }

        // Gravity is applied every frame
        let gravity = Vec2::new(0.0, 0.2);
        self.mover.apply_force(gravity);

        // Integrate acceleration -> velocity -> position.
        self.mover.update();

        // Keep the mover inside the world.
        self.mover.check_edges();
    }

    fn draw(&self) {
        draw_world_border();

        draw_circle(
            self.mover.position.x,
            self.mover.position.y,
            self.mover.radius,
            WHITE,
        );
        // Visual representation of gravity.
        draw_vector(self.mover.position, Vec2::new(0.0, 30.0), BLUE);

        // Visual representation of wind
        if is_mouse_button_down(MouseButton::Left) {
            draw_vector(self.mover.position, Vec2::new(30.0, 0.0), GRAY);
        }
        // -------------------------
        // Information
        // -------------------------
        let lines = [
            ("FORCES", WHITE),
            ("Gravity: (0.0, 0.2)", BLUE),
            ("Wind: (0.1, 0.0) [hold mouse]", GRAY),
            ("", WHITE),
            ("F = m x a", WHITE),
            ("a = F / m", WHITE),
        ];

        draw_info_panel(0.0, world_height() - 100.0, 210.0, &lines);
    }
}
