use macroquad::{
    color::{Color, WHITE},
    shapes::draw_line,
};

use vec_math::noise::perlin_1d;

use runner::{
    Example,
    camera::{world_height, world_width},
    render::draw_info_panel,
};

pub struct PerlinNoiseExample {
    x: f32,
    points: Vec<(f32, f32)>,
}

impl PerlinNoiseExample {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            points: Vec::new(),
        }
    }
}

impl Example for PerlinNoiseExample {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        // Move grad_1dually through noise space.
        self.x += 0.01;

        let noise_value = perlin_1d(self.x);

        let width = world_width();
        let height = world_height();

        let screen_x = self.points.len() as f32;

        // Convert [-1, 1] into screen coordinates.
        let screen_y = height * 0.5 - noise_value * height * 0.4;

        self.points.push((screen_x, screen_y));

        // Once we reach the edge, start drawing again.
        if screen_x >= width {
            self.points.clear();
        }
    }

    fn draw(&self) {
        for pair in self.points.windows(2) {
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];

            draw_line(x1, y1, x2, y2, 2.0, WHITE);
        }

        draw_info_panel(
            10.0,
            world_height() - 50.0,
            200.0,
            &[
                ("1D PERLIN NOISE", WHITE),
                ("Input changes smoothly", Color::new(0.7, 0.7, 0.7, 1.0)),
            ],
        );
    }
}
