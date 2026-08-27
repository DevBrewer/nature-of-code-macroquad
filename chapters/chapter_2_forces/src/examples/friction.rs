use crate::mover::Mover;

use macroquad::{
    color::{BLUE, GRAY, WHITE},
    input::{MouseButton, is_mouse_button_down},
    shapes::draw_circle,
};

use runner::{Example, draw_info_panel, draw_vector, draw_world_border, world_height, world_width};

use vec_math::Vec2;

pub struct FrictionExample {
    mover: Mover,
}

impl FrictionExample {
    pub fn new() -> Self {
        Self {
            mover: Mover::new(Vec2::new(world_width() * 0.5, world_height() * 0.5), 1.0),
        }
    }
}

impl Example for FrictionExample {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        // Apply a horizontal force so that
        // the mover has something to slide against.
        if is_mouse_button_down(MouseButton::Left) {
            let wind = Vec2::new(0.1, 0.0);
            self.mover.apply_force(wind);
        }

        // Gravity.
        //
        // In this simplified example, gravity is used
        // to represent contact with the horizontal surface.
        let gravity = Vec2::new(0.0, 0.2);
        self.mover.apply_force(gravity);

        // -------------------------------------------------
        // Friction
        // -------------------------------------------------
        //
        // Friction points opposite the direction of motion.
        //
        // The book uses:
        //
        // friction = -μ * N * velocity_unit_vector
        //
        // For this simplified example:
        //
        // N = 1
        // μ = 0.01

        let friction_coefficient = 0.01;
        let normal_force = 1.0;

        if self.mover.velocity.mag_sq() > f32::EPSILON {
            // Start with the direction of velocity.
            let mut friction = self.mover.velocity.normalized();

            // Friction opposes motion.
            friction *= -1.0;

            // Magnitude = μN.
            friction *= friction_coefficient * normal_force;

            self.mover.apply_force(friction);
        }

        self.mover.update();
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

        // Gravity representation.
        draw_vector(self.mover.position, Vec2::new(0.0, 30.0), BLUE);

        // Wind representation.
        if is_mouse_button_down(MouseButton::Left) {
            draw_vector(self.mover.position, Vec2::new(30.0, 0.0), GRAY);
        }

        // Friction representation.
        if self.mover.velocity.mag_sq() > f32::EPSILON {
            let friction_direction = self.mover.velocity.normalized() * -30.0;

            draw_vector(self.mover.position, friction_direction, WHITE);
        }

        let lines = [
            ("FRICTION", WHITE),
            ("Hold mouse: apply wind", GRAY),
            ("Friction opposes velocity", WHITE),
            ("μ = 0.01", WHITE),
            ("N = 1", WHITE),
            ("Friction = -μN v̂", WHITE),
        ];

        draw_info_panel(10.0, world_height() - 105.0, 235.0, &lines);
    }
}
