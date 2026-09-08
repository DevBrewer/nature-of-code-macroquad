use macroquad::{
    color::{GRAY, WHITE},
    rand::gen_range,
    shapes::{draw_circle, draw_circle_lines, draw_line},
};
use runner::{world_height, world_width};
use vec_math::Vec2;

pub struct Oscillator {
    pub angle: Vec2,
    pub angle_velocity: Vec2,
    pub amplitude: Vec2,
    pub radius: f32,
    pub origin: Vec2,
}

#[rustfmt::skip]
impl Oscillator {
    pub fn new() -> Self {
        Self {
            angle: Vec2::new(0.0, 0.0),
            angle_velocity: Vec2::new(
                gen_range(-0.05, 0.05), 
                gen_range(-0.05, 0.05)),
            amplitude: Vec2::new(
                gen_range(20.0, world_width() / 2.0),
                gen_range(20.0, world_height() / 2.0),
            ),
            origin: Vec2::new(
                world_width() / 2.0, 
                world_height() / 2.0),
            radius: 20.0,
        }
    }

    pub fn update(&mut self) {
        // Incrementing angle via velocity.
        self.angle += self.angle_velocity
    }

    pub fn show(&self) {
        // Oscillating on the x-axis from the center
        let x = self.origin.x + self.angle.x.sin() * self.amplitude.x;

        // Oscillating on the y-axis from the center
        let y = self.origin.y + self.angle.y.sin() * self.amplitude.y;

        // Draw Line
        draw_line(self.origin.x, self.origin.y, x, y, 2.0, GRAY);

        // Draw circle
        draw_circle(x, y, self.radius, WHITE);

        // Draw outline of the circles
        draw_circle_lines(x, y, self.radius, 2.0, GRAY);
    }
}
