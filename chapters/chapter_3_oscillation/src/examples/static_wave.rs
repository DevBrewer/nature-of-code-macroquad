use macroquad::{
    color::{Color, GRAY, WHITE},
    input::{KeyCode, is_key_pressed},
    shapes::{draw_circle, draw_circle_lines, draw_line},
    time::get_frame_time,
};
use runner::{
    Example, draw_info_panel, draw_world_border, world_center, world_height, world_width,
};

/// Example 3.8: Static Wave (with toggleable Example 3.9: Dynamic Wave)
///
/// Demonstrates drawing a wave pattern across space using a sine function:
/// `y = amplitude * sin(angle)`
///
/// In Example 3.8:
/// - The wave is plotted across the x-axis with a step of 24 pixels.
/// - For each consecutive x position, `angle` is incremented by `delta_angle`.
/// - The wave itself is static (does not change over time).
///
/// In Example 3.9 (Dynamic Wave):
/// - A `start_angle` progresses over time, creating an undulating motion.
/// - Press [Space] to toggle between static and animated modes.
/// - Press [Up] / [Down] arrows to adjust `delta_angle` (wavelength/frequency).
pub struct StaticWave {
    /// Starting angle of the wave (increments over time when animated)
    pub angle: f32,
    /// Change in angle between consecutive points along the x-axis (controls wavelength)
    pub delta_angle: f32,
    /// Amplitude of the wave (vertical displacement from center)
    pub amplitude: f32,
    /// Horizontal distance between adjacent circles
    pub x_spacing: f32,
    /// Radius of each circle
    pub circle_radius: f32,
}

impl StaticWave {
    pub fn new(angle: f32, delta_angle: f32, amplitude: f32) -> Self {
        Self {
            angle,
            delta_angle,
            amplitude,
            x_spacing: 24.0,
            circle_radius: 24.0, // Diameter 48 in p5.js -> radius 24
        }
    }
}

pub struct StaticWaveExample {
    pub wave: StaticWave,
    pub is_animated: bool,
    pub angular_velocity: f32,
}

impl StaticWaveExample {
    pub fn new() -> Self {
        Self {
            // In Example 3.8: deltaAngle is 0.2 and amplitude is 100.0
            wave: StaticWave::new(0.0, 0.2, 100.0),
            is_animated: false,
            angular_velocity: 0.02,
        }
    }
}

impl Example for StaticWaveExample {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        // Toggle animation mode (Example 3.8 Static Wave vs Example 3.9 Dynamic Wave)
        if is_key_pressed(KeyCode::Space) {
            self.is_animated = !self.is_animated;
        }

        // Adjust delta_angle to see different frequencies as shown in Figure 3.13
        if is_key_pressed(KeyCode::Up) {
            self.wave.delta_angle = (self.wave.delta_angle + 0.05).min(1.0);
        }
        if is_key_pressed(KeyCode::Down) {
            self.wave.delta_angle = (self.wave.delta_angle - 0.05).max(0.02);
        }

        // Advance starting angle only when animated
        if self.is_animated {
            self.wave.angle += self.angular_velocity * get_frame_time() * 60.0;
        }
    }

    fn draw(&self) {
        draw_world_border();
        let center = world_center();
        let w = world_width();

        // Center equilibrium line
        draw_line(
            0.0,
            center.y,
            w,
            center.y,
            1.0,
            Color::new(0.3, 0.3, 0.3, 0.5),
        );

        // Amplitude boundary lines
        draw_line(
            0.0,
            center.y - self.wave.amplitude,
            w,
            center.y - self.wave.amplitude,
            1.0,
            Color::new(0.2, 0.2, 0.2, 0.3),
        );
        draw_line(
            0.0,
            center.y + self.wave.amplitude,
            w,
            center.y + self.wave.amplitude,
            1.0,
            Color::new(0.2, 0.2, 0.2, 0.3),
        );

        // Initialize angle to the starting angle for this frame
        let mut current_angle = self.wave.angle;
        let mut x = 0.0;

        // Loop across the width of the canvas
        while x <= w {
            // Calculate y based on amplitude and sine of the current angle
            let y = self.wave.amplitude * current_angle.sin();
            let circle_y = center.y + y;

            // Draw circle at (x, center.y + y)
            draw_circle(
                x,
                circle_y,
                self.wave.circle_radius,
                Color::new(1.0, 1.0, 1.0, 0.3),
            );
            draw_circle_lines(x, circle_y, self.wave.circle_radius, 1.5, WHITE);

            // Increment angle by delta_angle for the next circle along x
            current_angle += self.wave.delta_angle;
            x += self.wave.x_spacing;
        }

        // Information HUD panel
        let mode_text = if self.is_animated {
            "Mode: Dynamic Wave (Ex 3.9) [Space: Pause]"
        } else {
            "Mode: Static Wave (Ex 3.8) [Space: Animate]"
        };

        let delta_text = format!("deltaAngle: {:.2} [Up/Down: Adjust]", self.wave.delta_angle);

        let lines = [
            ("EXAMPLE 3.8: STATIC WAVE", WHITE),
            ("y = amplitude * sin(angle)", GRAY),
            ("angle += delta_angle across x", GRAY),
            (mode_text, Color::new(0.4, 0.8, 1.0, 1.0)),
            (&delta_text, Color::new(0.8, 0.8, 0.8, 1.0)),
        ];
        draw_info_panel(10.0, world_height() - 120.0, 330.0, &lines);
    }
}
