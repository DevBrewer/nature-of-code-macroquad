use macroquad::color::{BLACK, Color};

pub trait Example {
    fn reset(&mut self);
    fn update(&mut self);
    fn draw(&self);

    fn clear_background(&self) -> bool {
        true
    }

    fn background_color(&self) -> Color {
        BLACK
    }

    // Draw a translucent layer instead of completely
    // Clearing the previous frame
    fn fade_background(&self) -> Option<f32> {
        None
    }
}

pub struct ExampleEntry {
    pub chapter: u8,
    pub number: &'static str,
    pub title: &'static str,
    pub example: Box<dyn Example>,
}
