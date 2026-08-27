use macroquad::{
    color::{Color, DARKPURPLE, GRAY, GREEN, WHITE},
    shapes::{draw_circle_lines, draw_rectangle, draw_rectangle_lines},
    text::draw_text,
};

use runner::{
    Example,
    camera::{mouse_world_position, world_center, world_height, world_width},
    render::{draw_axes, draw_vector},
};

pub struct VectorMagnitude;

impl VectorMagnitude {
    pub fn new() -> Self {
        Self
    }
}

impl Example for VectorMagnitude {
    fn reset(&mut self) {}

    fn update(&mut self) {}

    fn draw(&self) {
        draw_axes();

        // Window center
        let center = world_center();

        // Mouse position in world coordinates
        let mouse = mouse_world_position();

        // direction from center to mouse
        let direction = mouse - center;

        // Magnitude
        let magnitude = direction.mag();

        // Visual aid circle
        draw_circle_lines(center.x, center.y, magnitude, 1.5, DARKPURPLE);

        // Draw vector
        draw_vector(center, direction, GRAY);

        // Bottom HUD badge for magnitude bar & text
        let hud_x = 10.0;
        let hud_y = world_height() - 44.0;
        let hud_w = 200.0;
        let hud_h = 34.0;

        draw_rectangle(
            hud_x,
            hud_y,
            hud_w,
            hud_h,
            Color::new(0.06, 0.08, 0.12, 0.85),
        );
        draw_rectangle_lines(
            hud_x,
            hud_y,
            hud_w,
            hud_h,
            1.0,
            Color::new(0.30, 0.35, 0.45, 0.60),
        );

        draw_text(
            format!("Magnitude: {:.1} px", magnitude),
            hud_x + 8.0,
            hud_y + 15.0,
            13.0,
            WHITE,
        );

        // Magnitude bar track & fill
        let bar_x = hud_x + 8.0;
        let bar_y = hud_y + 20.0;
        let bar_max_w = hud_w - 16.0;
        let bar_w = (magnitude / (world_width() * 0.5) * bar_max_w).min(bar_max_w);

        draw_rectangle(
            bar_x,
            bar_y,
            bar_max_w,
            6.0,
            Color::new(0.18, 0.20, 0.25, 0.9),
        );
        draw_rectangle(bar_x, bar_y, bar_w, 6.0, GREEN);
    }
}
