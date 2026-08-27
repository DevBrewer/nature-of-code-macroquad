use macroquad::{
    color::{GRAY, WHITE},
    input::is_mouse_button_down,
};
use runner::{
    Example, draw_info_panel, draw_world_border, mouse_world_position, world_height, world_width,
};
use vec_math::Vec2;

use crate::{attractor::Attractor, mover::Mover};

pub struct AttractionExample {
    mover: Mover,
    attractor: Attractor,
}

impl AttractionExample {
    pub fn new() -> Self {
        Self {
            mover: Mover::new(Vec2::new(world_width() * 0.5, 50.0), 2.0),
            attractor: Attractor::new(),
        }
    }
}

impl Example for AttractionExample {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        // Calculate gravitional attraction.
        let attraction = self.attractor.attract(&self.mover);

        // Apply the force to the mover.
        self.mover.apply_force(attraction);

        // Update motion.
        self.mover.update();

        // Keep the mover inside the world.
        self.mover.check_edges();

        // Optional mouse integration.
        // Holding the mouse moves the attractor
        // toward the mouse position.
        if is_mouse_button_down(macroquad::input::MouseButton::Left) {
            let mouse = mouse_world_position();

            self.attractor.position = mouse;
        }
    }

    fn draw(&self) {
        draw_world_border();

        self.attractor.draw();
        self.mover.draw();

        let lines = [
            ("GRAVITATIONAL ATTRACTION", WHITE),
            ("F = G × m1 × m2 / r*r", WHITE),
            ("G = 1.0", GRAY),
            ("Hold mouse: move attractor", GRAY),
        ];

        draw_info_panel(10.0, world_height() - 80.0, 245.0, &lines);
    }
}
