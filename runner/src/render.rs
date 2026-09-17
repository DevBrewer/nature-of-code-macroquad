use std::sync::OnceLock;

use macroquad::{
    color::Color,
    input::{MouseButton, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released},
    shapes::{draw_line, draw_rectangle, draw_rectangle_lines},
    text::{Font, TextParams, draw_text_ex, load_ttf_font_from_bytes},
};

use vec_math::Vec2;

use crate::camera::{mouse_world_position, world_height, world_width};

static FONT_DATA: &[u8] = include_bytes!("../assets/font.ttf");
static UI_FONT: OnceLock<Font> = OnceLock::new();

pub fn get_ui_font() -> &'static Font {
    UI_FONT.get_or_init(|| load_ttf_font_from_bytes(FONT_DATA).expect("Failed to load TTF font"))
}

pub fn draw_clean_text(text: &str, x: f32, y: f32, size: f32, color: Color) {
    let font = get_ui_font();
    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font: Some(font),
            font_size: size as u16,
            font_scale: 1.0,
            color,
            ..Default::default()
        },
    );
}

static mut PANEL_OFFSET_X: f32 = 0.0;
static mut PANEL_OFFSET_Y: f32 = 0.0;
static mut IS_DRAGGING_PANEL: bool = false;
static mut DRAG_START_MOUSE: (f32, f32) = (0.0, 0.0);
static mut DRAG_START_OFFSET: (f32, f32) = (0.0, 0.0);
static mut HUD_VISIBLE: bool = true;

/// Returns whether the HUD info panel overlay is visible.
pub fn is_hud_visible() -> bool {
    unsafe { HUD_VISIBLE }
}

/// Sets the HUD info panel overlay visibility.
pub fn set_hud_visible(visible: bool) {
    unsafe {
        HUD_VISIBLE = visible;
    }
}

/// Toggles the HUD info panel overlay visibility.
pub fn toggle_hud() -> bool {
    unsafe {
        HUD_VISIBLE = !HUD_VISIBLE;
        HUD_VISIBLE
    }
}

/// Resets the HUD info panel drag position to default (0, 0).
pub fn reset_info_panel_offset() {
    unsafe {
        PANEL_OFFSET_X = 0.0;
        PANEL_OFFSET_Y = 0.0;
        IS_DRAGGING_PANEL = false;
    }
}

pub fn draw_axes() {
    let w = world_width();
    let h = world_height();
    let cx = w * 0.5;
    let cy = h * 0.5;

    draw_line(0.0, cy, w, cy, 1.0, Color::new(0.3, 0.3, 0.3, 1.0));
    draw_line(cx, 0.0, cx, h, 1.0, Color::new(0.3, 0.3, 0.3, 1.0));
}

pub fn draw_vector(origin: Vec2, vector: Vec2, color: Color, label: Option<&str>) {
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

    // Draw text label next to vector tip if provided
    if let Some(text) = label {
        draw_clean_text(text, end.x + 8.0, end.y + 4.0, 16.0, color);
    }
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
        Color::new(0.2, 0.4, 0.6, 0.8),
    );
}

/// Renders a modern, interactive, and draggable glassmorphic information panel.
pub fn draw_info_panel(x: f32, y: f32, width: f32, lines: &[(&str, Color)]) {
    if !is_hud_visible() {
        return;
    }

    let padding_x = 14.0;
    let padding_y = 12.0;
    let line_height = 20.0;
    let header_margin = 8.0;

    let height = padding_y * 2.0 + header_margin + (lines.len() as f32) * line_height;

    // Apply interactive drag offset
    let (offset_x, offset_y) = unsafe { (PANEL_OFFSET_X, PANEL_OFFSET_Y) };
    let mut panel_x = x + offset_x;
    let mut panel_y = y + offset_y;

    // Clamp panel inside world boundaries
    let max_x = (world_width() - width).max(0.0);
    let max_y = (world_height() - height).max(0.0);
    panel_x = panel_x.clamp(0.0, max_x);
    panel_y = panel_y.clamp(0.0, max_y);

    // Handle mouse drag interaction
    let mouse = mouse_world_position();
    let mouse_over = mouse.x >= panel_x
        && mouse.x <= panel_x + width
        && mouse.y >= panel_y
        && mouse.y <= panel_y + height;

    unsafe {
        if is_mouse_button_pressed(MouseButton::Left) && mouse_over {
            IS_DRAGGING_PANEL = true;
            DRAG_START_MOUSE = (mouse.x, mouse.y);
            DRAG_START_OFFSET = (PANEL_OFFSET_X, PANEL_OFFSET_Y);
        }

        if IS_DRAGGING_PANEL {
            if is_mouse_button_down(MouseButton::Left) {
                PANEL_OFFSET_X = DRAG_START_OFFSET.0 + (mouse.x - DRAG_START_MOUSE.0);
                PANEL_OFFSET_Y = DRAG_START_OFFSET.1 + (mouse.y - DRAG_START_MOUSE.1);
            }

            if is_mouse_button_released(MouseButton::Left) {
                IS_DRAGGING_PANEL = false;
            }
        }
    }

    // Frosted / dark panel background
    draw_rectangle(
        panel_x,
        panel_y,
        width,
        height,
        Color::new(0.05, 0.08, 0.14, 0.90),
    );

    // Top accent line
    draw_rectangle(
        panel_x,
        panel_y,
        width,
        3.0,
        Color::new(0.25, 0.55, 0.95, 0.85),
    );

    // Crisp border (highlighted cyan when dragging)
    let border_color = if unsafe { IS_DRAGGING_PANEL } {
        Color::new(0.40, 0.75, 1.0, 0.95)
    } else {
        Color::new(0.30, 0.40, 0.55, 0.70)
    };
    draw_rectangle_lines(panel_x, panel_y, width, height, 1.2, border_color);

    // Move handle hint icon in top right
    draw_clean_text(
        "[Move]",
        panel_x + width - 46.0,
        panel_y + 14.0,
        12.0,
        Color::new(0.50, 0.60, 0.75, 0.70),
    );

    for (i, (text, color)) in lines.iter().enumerate() {
        let text_y = panel_y + padding_y + (i as f32 + 1.0) * line_height - 3.0;
        draw_clean_text(text, panel_x + padding_x, text_y, 12.0, *color);
    }
}

pub fn fade_world(color: Color) {
    draw_rectangle(0.0, 0.0, world_width(), world_height(), color);
}
