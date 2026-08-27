use macroquad::{
    color::{Color, WHITE},
    shapes::{draw_circle, draw_rectangle},
};
use runner::{Example, draw_info_panel, draw_world_border, world_height, world_width};
use vec_math::Vec2;

use crate::{liquid::Liquid, mover::Mover};

pub struct FluidResistance {
    movers: Vec<Mover>,
    liquid: Liquid,
}

impl FluidResistance {
    pub fn new() -> Self {
        let width = world_width();
        let height = world_height();

        let mut movers = Vec::with_capacity(5);

        for i in 0..5 {
            let mass = 0.5 + i as f32;
            movers.push(Mover::new(Vec2::new(40.0 + i as f32 * 75.0, 50.0), mass));
        }

        let liquid = Liquid::new(0.0, height * 0.5, width, height * 0.5, 0.01);

        Self { movers, liquid }
    }
}

impl Example for FluidResistance {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        for mover in &mut self.movers {
            // Drag only acts while the mover
            // is inside the liquid
            if self.liquid.contains(mover) {
                let drag = self.liquid.calculate_drag(mover);
                mover.apply_force(drag);
            }

            // Gravity is scaled by mass.
            // This makes gravitional acceleration
            let gravity = Vec2::new(0.0, 0.1 * mover.mass);

            mover.apply_force(gravity);

            mover.update();
            mover.check_edges();
        }
    }

    fn draw(&self) {
        draw_world_border();

        // Liquid
        draw_rectangle(
            self.liquid.x,
            self.liquid.y,
            self.liquid.width,
            self.liquid.height,
            Color::new(0.25, 0.45, 0.65, 0.7),
        );

        // Balls
        for mover in &self.movers {
            draw_circle(mover.position.x, mover.position.y, mover.radius, WHITE);
        }

        let lines = [
            ("FLUID RESISTANCE", WHITE),
            ("Drag: -c x v^2 * v(unit)", WHITE),
            ("Liquid: lower half", WHITE),
            ("c = 0.1", WHITE),
            ("Differenct masses", WHITE),
        ];

        draw_info_panel(10.0, 100.0, 220.0, &lines);
    }
}
