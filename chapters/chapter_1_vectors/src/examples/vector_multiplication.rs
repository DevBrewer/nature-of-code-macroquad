use macroquad::color::{GRAY, RED};
use runner::{
    Example,
    camera::{mouse_world_position, world_center, world_height},
    render::{draw_axes, draw_info_panel, draw_vector},
};

pub struct VectorMultiplication;

impl VectorMultiplication {
    pub fn new() -> Self {
        Self
    }
}

impl Example for VectorMultiplication {
    fn reset(&mut self) {}

    fn update(&mut self) {}

    fn draw(&self) {
        draw_axes();

        // Window Center
        let center = world_center();

        // Mouse position
        let mouse = mouse_world_position();

        let vector = mouse - center;
        let scaled = vector * 0.5;

        draw_vector(center, vector, GRAY);
        draw_vector(center, scaled, RED);

        let orig_str = format!("Original (GRAY): {:.1} px", vector.mag());
        let scaled_str = format!("Scaled (RED)  : {:.1} px (0.5x)", scaled.mag());

        draw_info_panel(
            10.0,
            world_height() - 48.0,
            230.0,
            &[
                (&orig_str, GRAY),
                (&scaled_str, RED),
            ],
        );
    }
}


