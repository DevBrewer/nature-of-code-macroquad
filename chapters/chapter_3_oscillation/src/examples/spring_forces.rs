use macroquad::input::{
    MouseButton, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released,
};
use runner::{Example, draw_world_border, mouse_world_position, world_width};
use vec_math::Vec2;

use crate::spring::{Bob, Spring};

pub struct SpringForceExample {
    bob: Bob,
    spring: Spring,
    gravity: Vec2,
}

impl SpringForceExample {
    pub fn new() -> Self {
        let anchor = Vec2::new(world_width() * 0.5, 50.0);
        let rest_length = 200.0;
        let bob_pos = Vec2::new(anchor.x, anchor.y + rest_length + 50.0);
        Self {
            bob: Bob::new(bob_pos, 24.0),
            spring: Spring::new(anchor, rest_length, 0.1),
            gravity: Vec2::new(0.0, 2.0),
        }
    }
}

impl Example for SpringForceExample {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        let mouse = mouse_world_position();

        // Get the mouse coordinates
        if is_mouse_button_pressed(MouseButton::Left) {
            self.bob.handle_mouse_press(mouse);
        }

        // Drag ball
        if is_mouse_button_down(MouseButton::Left) {
            self.bob.handle_mouse_drag(mouse);
        }

        // Stop dragging
        if is_mouse_button_released(MouseButton::Left) {
            self.bob.handle_mouse_release();
        }

        if !self.bob.is_dragging {
            self.bob.apply_force(self.gravity);
            self.spring.connect(&mut self.bob);
            self.spring.constrain_length(&mut self.bob, 30.0, 400.0);
            self.bob.update();
        }
    }

    fn draw(&self) {
        draw_world_border();
        self.spring.show(&self.bob);
        self.bob.show();
    }
}
