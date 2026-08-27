use crate::{attractor::Attractor, mover::Mover};
use macroquad::{
    color::{GRAY, WHITE},
    rand::gen_range,
};
use runner::{Example, draw_info_panel, draw_world_border, world_height, world_width};
use vec_math::Vec2;

pub struct AttractionMany {
    movers: Vec<Mover>,
    attractor: Attractor,
}

impl AttractionMany {
    pub fn new() -> Self {
        let mut movers = Vec::with_capacity(10);

        for _ in 0..10 {
            let position = Vec2::new(
                gen_range(20.0, world_width() - 20.0),
                gen_range(20.0, world_height() - 20.0),
            );

            let mass = gen_range(0.5, 3.0);

            movers.push(Mover::new(position, mass));
        }

        Self {
            movers,
            attractor: Attractor::new(),
        }
    }
}

impl Example for AttractionMany {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        for mover in &mut self.movers {
            // Calculate the attraction produced by
            // the same attractor for this particular mover
            let force = self.attractor.attract(mover);

            // Apply the force through Newton's second law
            mover.apply_force(force);

            mover.update();
            mover.check_edges();
        }
    }

    fn fade_background(&self) -> Option<f32> {
        Some(0.08)
    }

    fn draw(&self) {
        draw_world_border();

        self.attractor.draw();

        for mover in &self.movers {
            mover.draw();
        }

        let lines = [
            ("ATTRACTION WITH MANY MOVERS", WHITE),
            ("10 movers -> 1 attractor", WHITE),
            ("Mass: 0.5 - 3.0", GRAY),
            ("F = G × m1 × m2 / r²", GRAY),
        ];

        draw_info_panel(10.0, world_height() - 75.0, 245.0, &lines);
    }
}
