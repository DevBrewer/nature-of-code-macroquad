use macroquad::{
    color::{GRAY, WHITE},
    shapes::{draw_circle, draw_line},
    time::get_frame_time,
};
use runner::{Example, draw_info_panel, draw_world_border, world_center, world_height};

/// Example 3.4.1: Polar Oscillation
///
/// Demonstrates converting polar coordinates (radial distance `r` and angle `theta`)
/// to Cartesian coordinates (x, y) to produce smooth circular oscillation.
///
/// Polar to Cartesian Formulas:
/// - `x = r * cos(theta)`
/// - `y = r * sin(theta)`
pub struct OscillationExample {
    /// Current angular position in radians (theta)
    pub theta: f32,
    /// Radial distance from the pivot center (r)
    pub height: f32,
    /// Radius of the oscillating bob
    pub radius: f32,
}

impl OscillationExample {
    pub fn new() -> Self {
        Self {
            theta: 0.0,
            height: world_height() * 0.25,
            radius: 20.0,
        }
    }
}

impl Example for OscillationExample {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        // Increment theta smoothly using delta time (approx 0.02 rad per 60 FPS frame)
        self.theta += 1.2 * get_frame_time();
    }

    fn draw(&self) {
        draw_world_border();

        // Get center coordinates of the simulation canvas
        let center = world_center();

        // Convert polar coordinates (height as radial distance r, and theta) to Cartesian (x, y)
        // x = r * cos(theta)
        // y = r * sin(theta)
        let x = center.x + self.height * self.theta.cos();
        let y = center.y + self.height * self.theta.sin();

        // Draw connecting arm line from center pivot to bob
        draw_line(center.x, center.y, x, y, 2.0, WHITE);

        // Draw center pivot anchor point
        draw_circle(center.x, center.y, 4.0, GRAY);

        // Draw oscillating bob circle
        draw_circle(x, y, self.radius, WHITE);

        // Information HUD panel
        let lines = [
            ("POLAR OSCILLATION", WHITE),
            ("x = r * cos(theta)", GRAY),
            ("y = r * sin(theta)", GRAY),
        ];
        draw_info_panel(10.0, world_height() - 75.0, 240.0, &lines);
    }
}
