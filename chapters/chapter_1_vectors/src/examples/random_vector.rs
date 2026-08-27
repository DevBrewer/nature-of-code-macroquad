use macroquad::{
    color::Color,
    input::{KeyCode, is_key_pressed},
    rand::gen_range,
    shapes::draw_line,
};

use vec_math::Vec2;

use runner::{
    Example,
    camera::world_center,
};

pub struct RandomVector {
    vector: Vec2,
    position: Vec2,
}

impl RandomVector {
    pub fn new() -> Self {
        Self {
            position: world_center(),
            vector: Vec2::random_2d(),
        }
    }
}

impl Example for RandomVector {
    fn reset(&mut self) {
        self.position = world_center();
        self.vector = Vec2::random_2d();
    }

    fn update(&mut self) {
        self.position = world_center();

        if is_key_pressed(KeyCode::Space) {
            self.reset();
        }

        // New random vector EVERY frame.
        self.vector = Vec2::random_2d();

        // Random length between 50 and 100.
        self.vector.set_mag(gen_range(50.0, 100.0));
    }

    fn draw(&self) {
        draw_line(
            self.position.x,
            self.position.y,
            self.position.x + self.vector.x,
            self.position.y + self.vector.y,
            4.0,
            Color::new(1.0, 1.0, 1.0, 50.0 / 255.0),
        );
    }

    fn clear_background(&self) -> bool {
        false
    }

    fn background_color(&self) -> Color {
        Color::new(0.0, 0.0, 0.0, 1.0)
    }
}

