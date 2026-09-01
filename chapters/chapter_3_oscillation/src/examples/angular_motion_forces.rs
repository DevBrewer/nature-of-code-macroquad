use macroquad::{
    color::{GRAY, WHITE},
    rand::gen_range,
};
use runner::{Example, draw_info_panel, draw_world_border, world_height, world_width};
use vec_math::Vec2;

use crate::{angular_mover::AngularMover, attractor::Attractor};
pub struct AngularMotionForces {
    movers: Vec<AngularMover>,
    attractor: Attractor,
}

impl AngularMotionForces {
    pub fn new() -> Self {
        let movers = (0..20)
            .map(|_| {
                let position = Vec2::new(
                    gen_range(0.0, world_width()),
                    gen_range(0.0, world_height()),
                );
                let mass = gen_range(0.5, 2.0);

                AngularMover::new(position, mass)
            })
            .collect();

        Self {
            movers,
            attractor: Attractor::new(),
        }
    }
}

impl Example for AngularMotionForces {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        self.movers.iter_mut().for_each(|mover| {
            // The attractor calculates the gravitional force
            // acting on this mover.
            let force = self.attractor.attract(&mover.body);

            // Apply the force to the mover
            mover.apply_force(force);

            // update both
            // 1. linear 2. angular motion
            mover.update();
        });
    }

    fn draw(&self) {
        draw_world_border();

        // Draw the central gravitional attractor.
        self.attractor.draw();

        // Draw all moving and rotating bodies.
        self.movers.iter().for_each(|mover| mover.draw());

        let lines = [
            ("ANGULAR MOTION + FORCES", WHITE),
            ("Acceleration -> rotation", GRAY),
            ("Angular acceleration", GRAY),
            ("Angular velocity", GRAY),
        ];

        draw_info_panel(10.0, world_height() - 75.0, 245.0, &lines);
    }
}
