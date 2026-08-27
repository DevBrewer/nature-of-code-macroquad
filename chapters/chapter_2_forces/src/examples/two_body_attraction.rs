use macroquad::color::{GRAY, WHITE};
use runner::{Example, draw_info_panel, draw_world_border, world_center, world_height};
use vec_math::Vec2;

use crate::body::Body;

pub struct TwoBodyAttraction {
    body_a: Body,
    body_b: Body,
}

impl TwoBodyAttraction {
    pub fn new() -> Self {
        let center = world_center();

        // Gravity
        // Body::set_gravity(10.0);

        let mut body_a = Body::new(Vec2::new(center.x + 100.0, center.y), 3.0);
        body_a.set_velocity(Vec2::new(0.0, -3.0)); // set velocity

        let mut body_b = Body::new(Vec2::new(center.x - 100.0, center.y), 3.0);
        body_b.set_velocity(Vec2::new(0.0, 3.0)); // set velocity
        Self { body_a, body_b }
    }
}

impl Example for TwoBodyAttraction {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        // Force acting on A due to B.
        let force_on_a = Body::gravitational_force(&self.body_a, &self.body_b);

        // Force acting on B due to A.
        let force_on_b = Body::gravitational_force(&self.body_b, &self.body_a);

        self.body_a.apply_force(force_on_a);
        self.body_b.apply_force(force_on_b);
        self.body_a.update();
        self.body_b.update();
    }

    fn fade_background(&self) -> Option<f32> {
        Some(0.01)
    }

    fn draw(&self) {
        draw_world_border();

        self.body_a.draw();
        self.body_b.draw();

        let lines = [
            ("TWO-BODY ATTRACTION", WHITE),
            ("A <-> B", WHITE),
            ("F = G × m1 × m2 / r²", GRAY),
            ("Both bodies accelerate", GRAY),
        ];

        draw_info_panel(10.0, world_height() - 75.0, 230.0, &lines);
    }
}
