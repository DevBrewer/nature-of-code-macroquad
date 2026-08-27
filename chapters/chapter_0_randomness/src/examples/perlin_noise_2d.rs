use macroquad::{
    color::{Color, WHITE},
    shapes::draw_rectangle,
};
use runner::{Example, draw_info_panel, world_height, world_width};
use vec_math::noise::perlin_2d;

pub struct PerlinNoise2D {
    offset_x: f32,
    offset_y: f32,
    scale: f32,
}

impl PerlinNoise2D {
    pub fn new() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
            scale: 0.05, // quickly move through noise space (smaller scale: smooth features)
        }
    }

    fn noise_color(value: f32) -> Color {
        // Our noise is centered approx around zero.
        // convert [-1, 1] to [0, 1]
        // for grayscale rendering.
        let brightness = ((value + 1.0) * 0.5).clamp(0.0, 1.0);

        Color::new(brightness, brightness, brightness, 1.0)
    }
}

impl Example for PerlinNoise2D {
    fn reset(&mut self) {
        *self = Self::new();
    }
    fn update(&mut self) {}

    fn draw(&self) {
        let width = world_width() as i32;
        let height = world_height() as i32;

        for y in 0..height {
            let noise_y = (y as f32) * self.scale + self.offset_y;
            for x in 0..width {
                let noise_x = (x as f32) * self.scale + self.offset_x;
                let value = perlin_2d(noise_x, noise_y);
                let color = Self::noise_color(value);

                draw_rectangle(x as f32, y as f32, 1.0, 1.0, color);
            }
        }

        draw_info_panel(
            1.0,
            world_height() - 50.0,
            210.0,
            &[
                ("2D PERLIN NOISE", WHITE),
                ("Smooth spatial variation", Color::new(0.7, 0.7, 0.7, 1.0)),
            ],
        );
    }
}
