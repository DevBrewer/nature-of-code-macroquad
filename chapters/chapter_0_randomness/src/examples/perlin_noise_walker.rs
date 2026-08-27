use macroquad::{
    color::{Color, WHITE},
    shapes::draw_circle,
};
use runner::{Example, draw_info_panel, world_height, world_width};
use vec_math::{Vec2, noise::perlin_1d};

const NOISE_STEP: f32 = 0.01;
const ANGLE_RANGE: f32 = std::f32::consts::TAU;

pub struct PerlinNoiseWalker {
    position: Vec2,
    noise_x: f32,
    radius: f32,
}

impl PerlinNoiseWalker {
    pub fn new() -> Self {
        Self {
            position: Vec2::new(world_width() * 0.5, world_height() * 0.5),
            noise_x: 0.0,
            radius: 8.0,
        }
    }
}

impl Example for PerlinNoiseWalker {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        // Move through noise space slowly.
        self.noise_x += NOISE_STEP;

        // Convert the noise value into angle.
        //
        // Perli noise is approx. [-1, 1]
        // We map that range to [0, TAU]
        let noise_value = perlin_1d(self.noise_x);

        let angle = (noise_value + 1.0) * 0.5 * ANGLE_RANGE;

        // Convert the angle into a direction.
        let velocity = Vec2::new(angle.cos(), angle.sin());

        // Move the walker.
        self.position += velocity;

        // Keep the walker inside the world.
        self.position.x = self
            .position
            .x
            .clamp(self.radius, world_width() - self.radius);

        self.position.y = self
            .position
            .y
            .clamp(self.radius, world_height() - self.radius);
    }

    fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, WHITE);

        draw_info_panel(
            10.0,
            world_height() - 44.0,
            220.0,
            &[
                ("PERLIN NOISE WALKER", WHITE),
                ("Direction changes smoothly", Color::new(0.7, 0.7, 0.7, 1.0)),
            ],
        );
    }
}
