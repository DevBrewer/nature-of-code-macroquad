use macroquad::color::{GRAY, GREEN};
use runner::{
    Example,
    camera::{mouse_world_position, world_center, world_height},
    render::{draw_axes, draw_info_panel, draw_vector},
};

pub struct VectorNormalization;

impl VectorNormalization {
    pub fn new() -> Self {
        Self
    }
}

impl Example for VectorNormalization {
    fn reset(&mut self) {}

    fn update(&mut self) {}

    fn draw(&self) {
        draw_axes();

        // Window center
        let center = world_center();

        // Mouse Position in world coordinates
        let mouse = mouse_world_position();

        // Vector from center to mouse
        let vector = mouse - center;

        // Unit vector
        let normalized = vector.normalized();

        // Original vector
        draw_vector(center, vector, GRAY);

        // Normalized vector (scaled for visibility)
        draw_vector(center, normalized * 100.0, GREEN);

        let orig_str = format!("Original Mag (GRAY) : {:.1} px", vector.mag());
        let norm_str = format!("Unit Vector (GREEN)  : {:.2} (len = 1.0)", normalized.mag());

        draw_info_panel(
            10.0,
            world_height() - 48.0,
            240.0,
            &[
                (&orig_str, GRAY),
                (&norm_str, GREEN),
            ],
        );
    }
}


