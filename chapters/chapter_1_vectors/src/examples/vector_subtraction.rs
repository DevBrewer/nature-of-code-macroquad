use macroquad::color::{DARKGRAY, WHITE};
use runner::{
    Example,
    camera::{mouse_world_position, world_center, world_height},
    render::{draw_axes, draw_info_panel, draw_vector},
};

pub struct VectorSubtraction;

impl VectorSubtraction {
    pub fn new() -> Self {
        Self
    }
}

impl Example for VectorSubtraction {
    fn reset(&mut self) {}

    fn update(&mut self) {}

    fn draw(&self) {
        draw_axes();

        let center = world_center();
        let mouse = mouse_world_position();

        let direction = mouse - center;

        draw_vector(center, direction, DARKGRAY);

        let mag_str = format!("Distance: {:.1} px", direction.mag());
        draw_info_panel(10.0, world_height() - 32.0, 160.0, &[(&mag_str, WHITE)]);
    }
}


