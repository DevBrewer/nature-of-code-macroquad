use macroquad::{
    color::{Color, GRAY},
    input::{MouseButton, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released, mouse_position},
    shapes::{draw_line, draw_rectangle, draw_rectangle_lines},
    text::draw_text,
};

use vec_math::Vec2;

use crate::camera::{world_height, world_width};

static mut PANEL_OFFSET_X: f32 = 0.0;
static mut PANEL_OFFSET_Y: f32 = 0.0;
static mut IS_DRAGGING_PANEL: bool = false;
static mut DRAG_START_MOUSE: (f32, f32) = (0.0, 0.0);
static mut DRAG_START_OFFSET: (f32, f32) = (0.0, 0.0);

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

/// Renders a modern, interactive, and draggable glassmorphic information panel.
pub fn draw_info_panel(x: f32, y: f32, width: f32, lines: &[(&str, Color)]) {
    let line_height = 18.0;
    let padding_x = 10.0;
    let padding_y = 8.0;
    let height = lines.len() as f32 * line_height + padding_y * 2.0;

    let (mx, my) = mouse_position();

    unsafe {
        let current_draw_x = x + PANEL_OFFSET_X;
        let current_draw_y = y + PANEL_OFFSET_Y;

        // Check for Mouse Dragging on Info Panel
        if is_mouse_button_pressed(MouseButton::Left) {
            if mx >= current_draw_x && mx <= current_draw_x + width && my >= current_draw_y && my <= current_draw_y + height {
                IS_DRAGGING_PANEL = true;
                DRAG_START_MOUSE = (mx, my);
                DRAG_START_OFFSET = (PANEL_OFFSET_X, PANEL_OFFSET_Y);
            }
        }

        if IS_DRAGGING_PANEL {
            if is_mouse_button_down(MouseButton::Left) {
                PANEL_OFFSET_X = DRAG_START_OFFSET.0 + (mx - DRAG_START_MOUSE.0);
                PANEL_OFFSET_Y = DRAG_START_OFFSET.1 + (my - DRAG_START_MOUSE.1);
            } else {
                IS_DRAGGING_PANEL = false;
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            IS_DRAGGING_PANEL = false;
        }

        let panel_x = x + PANEL_OFFSET_X;
        let panel_y = y + PANEL_OFFSET_Y;

        // Drop shadow for depth
        draw_rectangle(panel_x + 3.0, panel_y + 3.0, width, height, Color::new(0.0, 0.0, 0.0, 0.35));

        // Dark glassmorphic background
        draw_rectangle(panel_x, panel_y, width, height, Color::new(0.05, 0.07, 0.12, 0.90));

        // Top accent line
        draw_rectangle(panel_x, panel_y, width, 3.0, Color::new(0.25, 0.55, 0.95, 0.85));

        // Crisp border (highlighted cyan when dragging)
        let border_color = if IS_DRAGGING_PANEL {
            Color::new(0.40, 0.75, 1.0, 0.95)
        } else {
            Color::new(0.30, 0.40, 0.55, 0.70)
        };
        draw_rectangle_lines(panel_x, panel_y, width, height, 1.2, border_color);

        // Move handle hint icon in top right
        draw_text("::: Move", panel_x + width - 54.0, panel_y + 13.0, 11.0, Color::new(0.50, 0.60, 0.75, 0.60));

        for (i, (text, color)) in lines.iter().enumerate() {
            let text_y = panel_y + padding_y + (i as f32 + 1.0) * line_height - 3.0;
            draw_text(text, panel_x + padding_x, text_y, 15.0, *color);
        }
    }
}

pub fn fade_world(color: Color) {
    draw_rectangle(0.0, 0.0, world_width(), world_height(), color);
}
