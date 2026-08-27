use macroquad::{
    color::{BLUE, GREEN, RED, WHITE},
    shapes::draw_circle,
};
use runner::{Example, draw_info_panel, draw_world_border, world_height, world_width};
use vec_math::Vec2;

use crate::mover::Mover;

pub struct MassExample {
    movers: Vec<Mover>,
}

impl MassExample {
    pub fn new() -> Self {
        let y = world_height() * 0.5;

        Self {
            movers: vec![
                Mover::new(Vec2::new(world_width() * 0.5, y - 80.0), 1.0),
                Mover::new(Vec2::new(world_width() * 0.25, y), 2.0),
                Mover::new(Vec2::new(world_width(), y + 80.0), 5.0),
            ],
        }
    }
}

impl Example for MassExample {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        // Every mover receives exactly the same force.
        let force = Vec2::new(0.0, 0.1);

        for mover in &mut self.movers {
            mover.apply_force(force);
            mover.update();
            mover.check_edges();
        }
    }

    fn draw(&self) {
        draw_world_border();

        let colors = [RED, GREEN, BLUE];

        for (mover, color) in self.movers.iter().zip(colors.iter()) {
            draw_circle(mover.position.x, mover.position.y, mover.radius, *color);
        }

        let lines = [
            ("MASS", WHITE),
            ("Same force applied to every mover", WHITE),
            ("Mass 1.0 -> fastest", RED),
            ("Mass 2.0 -> slower", GREEN),
            ("Mass 5.0 -> slowest", BLUE),
        ];

        draw_info_panel(10.0, world_height() - 100.0, 260.0, &lines);
    }
}
