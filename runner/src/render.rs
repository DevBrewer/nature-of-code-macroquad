use macroquad::{
    color::{Color, GRAY},
    shapes::{draw_line, draw_rectangle, draw_rectangle_lines},
    text::draw_text,
};

use vec_math::Vec2;

use crate::camera::{world_height, world_width};

pub fn draw_axes() {
    let w = world_width();
    let h = world_height();
    let cx = w * 0.5;
    let cy = h * 0.5;

    draw_line(0.0, cy, w, cy, 1.0, Color::new(0.3, 0.3, 0.3, 1.0));
    draw_line(cx, 0.0, cx, h, 1.0, Color::new(0.3, 0.3, 0.3, 1.0));
}

pub fn draw_vector(origin: Vec2, vector: Vec2, color: Color) {
    let start = origin;
    let end = start + vector;

    // Shaft
    draw_line(start.x, start.y, end.x, end.y, 3.0, color);

    // Don't draw an arrowhead for a zero-length vector.
    if vector.mag_sq() <= f32::EPSILON {
        return;
    }

    let direction = vector.normalized();

    let arrow_length = 12.0;
    let arrow_angle = 0.5; // ~29°

    let left = end - direction.rotate(arrow_angle) * arrow_length;
    let right = end - direction.rotate(-arrow_angle) * arrow_length;

    draw_line(end.x, end.y, left.x, left.y, 3.0, color);
    draw_line(end.x, end.y, right.x, right.y, 3.0, color);
}

// Draw border
pub fn draw_world_border() {
    let w = world_width();
    let h = world_height();
    let thickness = 2.0;
    let half_t = thickness * 0.5;
    draw_rectangle_lines(
        half_t,
        half_t,
        w - thickness,
        h - thickness,
        thickness,
        GRAY,
    );
}

/// Renders a compact, translucent information panel at (x, y) with the provided text lines.
pub fn draw_info_panel(x: f32, y: f32, width: f32, lines: &[(&str, Color)]) {
    let line_height = 15.0;
    let padding = 6.0;
    let height = lines.len() as f32 * line_height + padding * 2.0;

    // Translucent background
    draw_rectangle(x, y, width, height, Color::new(0.06, 0.08, 0.12, 0.85));
    // Crisp border
    draw_rectangle_lines(x, y, width, height, 1.0, Color::new(0.30, 0.35, 0.45, 0.60));

    for (i, (text, color)) in lines.iter().enumerate() {
        let text_y = y + padding + (i as f32 + 1.0) * line_height - 3.0;
        draw_text(text, x + padding + 2.0, text_y, 16.0, *color);
    }
}

pub fn fade_world(color: Color) {
    draw_rectangle(0.0, 0.0, world_width(), world_height(), color);
}
