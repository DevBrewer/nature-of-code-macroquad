use macroquad::{
    color::{GRAY, WHITE},
    rand::gen_range,
};

use runner::{Example, draw_info_panel, draw_world_border, world_center, world_height};

use vec_math::Vec2;

use crate::body::Body;

pub struct NBodyAttraction {
    bodies: Vec<Body>,
}

impl NBodyAttraction {
    pub fn new() -> Self {
        const BODY_COUNT: usize = 150;
        const BODY_MASS: f32 = 2.0;
        const INITIAL_SPEED: f32 = 0.2;

        // Gravitational constant used by every pair interaction.
        Body::set_gravity(1.0);

        let center = world_center();

        let mut bodies = Vec::with_capacity(BODY_COUNT);

        for _ in 0..BODY_COUNT {
            // Random position around the center.
            let radius = gen_range(40.0, 180.0);

            let angle = gen_range(0.0, std::f32::consts::TAU);

            // Unit vector pointing from the center toward
            // the body's initial position.
            let radial = Vec2::new(angle.cos(), angle.sin());

            let position = center + radial * radius;

            let mut body = Body::new(position, BODY_MASS);

            // --------------------------------------------------
            // Initial tangential velocity.
            //
            // radial:
            //
            //             body
            //               ●
            //              /
            //             /
            //            ● center
            //
            // Rotate the radial direction by 90° to obtain
            // a tangential direction.
            // --------------------------------------------------

            let mut velocity = radial.rotate(std::f32::consts::FRAC_PI_2);
            velocity.set_mag(INITIAL_SPEED);

            body.set_velocity(velocity);
            bodies.push(body);
        }

        Self { bodies }
    }
}

impl Example for NBodyAttraction {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        let count = self.bodies.len();

        // ------------------------------------------------------
        // Phase 1: calculate all forces.
        //
        // Every force uses the positions from the current frame.
        // No body is updated during this phase.
        // ------------------------------------------------------
        let mut forces = vec![Vec2::ZERO; count];

        for (i, body_a) in self.bodies.iter().enumerate() {
            for (j, body_b) in self.bodies.iter().enumerate() {
                if i == j {
                    continue;
                }

                let force = Body::gravitational_force(body_a, body_b);
                forces[i] += force;
            }
        }

        // ------------------------------------------------------
        // Phase 2: apply forces and update bodies.
        // ------------------------------------------------------

        for (body, force) in self.bodies.iter_mut().zip(forces) {
            body.apply_force(force);
            body.update();
        }
    }

    fn draw(&self) {
        draw_world_border();

        for body in &self.bodies {
            body.draw();
        }

        let lines = [
            ("N-BODY ATTRACTION", WHITE),
            ("N  equal-mass bodies", WHITE),
            ("Every body attracts every other", GRAY),
            ("Tangential initial velocity", GRAY),
            ("O(N²) pair interactions", GRAY),
        ];

        draw_info_panel(10.0, world_height() - 90.0, 260.0, &lines);
    }
}
