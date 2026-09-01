use macroquad::{
    color::{GRAY, WHITE},
    shapes::{draw_circle, draw_line},
};
use runner::{Example, draw_info_panel, world_center, world_height, world_width};
use vec_math::Vec2;

pub struct AngleRotation {
    angle: f32,
}

impl AngleRotation {
    pub fn new() -> Self {
        Self { angle: 0.0 }
    }
}

impl Example for AngleRotation {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        self.angle += 0.02;
    }

    fn draw(&self) {
        let center = world_center();
        let length = world_width() * 0.25;

        let baton = Vec2::new(length, 0.0).rotate(self.angle);

        let start = center - baton;
        let end = center + baton;

        // Draw Baton
        draw_line(start.x, start.y, end.x, end.y, 4.0, WHITE);
        // Draw pivot center
        draw_circle(center.x, center.y, 5.0, GRAY);

        // Draw Left and right circles
        draw_circle(start.x, start.y, 8.0, WHITE);
        draw_circle(end.x, end.y, 8.0, WHITE);

        let lines = [
            ("ANGULAR MOTION", WHITE),
            ("Angle: radians", GRAY),
            ("Rotation around Center", GRAY),
        ];

        draw_info_panel(10.0, world_height() - 75.0, 250.0, &lines);
    }
}
