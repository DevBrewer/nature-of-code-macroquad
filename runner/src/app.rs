use macroquad::{
    camera::set_default_camera,
    color::{Color, WHITE},
    input::{KeyCode, is_key_pressed},
    text::draw_text,
    window::{Conf, clear_background},
};

use crate::{camera::Camera, example::ExampleEntry, render::fade_world};

/// Constructs a standard Macroquad window configuration.
pub fn window_conf(title: &str, width: i32, height: i32) -> Conf {
    Conf {
        window_title: title.to_string(),
        window_width: width,
        window_height: height,
        high_dpi: false,
        sample_count: 4,
        window_resizable: false,
        ..Default::default()
    }
}

pub struct App {
    current: usize,
    examples: Vec<ExampleEntry>,
    camera: Camera,
    clear_canvas: bool,
}

impl App {
    /// Creates a new App with the specified world canvas dimensions (e.g. from main.rs).
    pub fn new(examples: Vec<ExampleEntry>, world_width: f32, world_height: f32) -> Self {
        Self {
            current: 0,
            examples,
            camera: Camera::new(world_width, world_height),
            clear_canvas: true,
        }
    }

    /// Creates a new App with default 400x400 canvas dimensions.
    pub fn default_size(examples: Vec<ExampleEntry>) -> Self {
        Self::new(
            examples,
            crate::camera::WORLD_WIDTH,
            crate::camera::WORLD_HEIGHT,
        )
    }

    /// Creates a new App where canvas dimensions dynamically match the physical screen / window.
    pub fn from_screen(examples: Vec<ExampleEntry>) -> Self {
        Self {
            current: 0,
            examples,
            camera: Camera::from_screen(),
            clear_canvas: true,
        }
    }

    pub fn update(&mut self) {
        if self.examples.is_empty() {
            return;
        }

        // -------------------------
        // Navigation
        // -------------------------

        if is_key_pressed(KeyCode::Right) {
            self.current = (self.current + 1) % self.examples.len();

            self.examples[self.current].example.reset();
            self.clear_canvas = true;
        }

        if is_key_pressed(KeyCode::Left) {
            self.current = if self.current == 0 {
                self.examples.len() - 1
            } else {
                self.current - 1
            };

            self.examples[self.current].example.reset();
            self.clear_canvas = true;
        }

        if is_key_pressed(KeyCode::R) {
            self.examples[self.current].example.reset();
            self.clear_canvas = true;
        }

        // -------------------------
        // Camera
        // -------------------------

        self.camera.update();

        // -------------------------
        // Example
        // -------------------------

        self.camera.apply();

        let example = &mut self.examples[self.current].example;

        if self.clear_canvas {
            clear_background(example.background_color());
            self.clear_canvas = false;
        } else if let Some(alpha) = example.fade_background() {
            let bg = example.background_color();
            fade_world(Color::new(bg.r, bg.g, bg.b, alpha));
        } else if example.clear_background() {
            clear_background(example.background_color());
        }

        example.update();
        example.draw();

        // -------------------------
        // UI & Screen Presentation
        // -------------------------

        // Return to physical screen coordinates.
        set_default_camera();
        clear_background(macroquad::color::DARKGRAY);

        // Present the world with distortion / scaling.
        self.camera.present();

        // UI is now in physical screen coordinates.
        self.draw_ui();
    }

    fn draw_ui(&self) {
        let current = &self.examples[self.current];
        let sw = macroquad::window::screen_width();

        // Sleek top header bar
        let header_height = 42.0;
        macroquad::shapes::draw_rectangle(
            0.0,
            0.0,
            sw,
            header_height,
            macroquad::color::Color::new(0.06, 0.08, 0.12, 0.88),
        );
        macroquad::shapes::draw_line(
            0.0,
            header_height,
            sw,
            header_height,
            1.0,
            macroquad::color::Color::new(0.25, 0.30, 0.40, 0.60),
        );

        // Example title
        draw_text(
            format!("{}. {}", current.number, current.title),
            10.0,
            18.0,
            16.0,
            WHITE,
        );

        // Navigation hints and counter
        draw_text(
            format!(
                "[<- / ->] Navigate   [R] Reset   ({}/{})",
                self.current + 1,
                self.examples.len(),
            ),
            10.0,
            34.0,
            14.0,
            macroquad::color::Color::new(0.65, 0.70, 0.78, 0.95),
        );
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }
}
