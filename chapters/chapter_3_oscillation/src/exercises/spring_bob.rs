use std::f32::consts::TAU;

use macroquad::{
    color::{Color, GRAY, WHITE},
    input::{KeyCode, is_key_pressed},
    shapes::{draw_circle, draw_line},
    time::get_frame_time,
};
use runner::{Example, draw_info_panel, draw_world_border, world_height, world_width};
use vec_math::Vec2;

/// Exercise 3.5 / 3.6: Spring Bob Simulation using `sin()` and `map()`
///
/// Simulates a weight (bob) suspended from a ceiling spring.
/// Rather than solving full Hooke's Law physics ($F = -k x$), this exercise
/// models the resulting Simple Harmonic Motion directly by mapping a sine wave
/// to the spring's vertical extension.
///
/// Formulas:
/// - `phase = (2*PI * frame_count) / period`
/// - `spring_length = map(sin(phase), -1.0, 1.0, min_len, max_len)`
/// - `bob_y = anchor.y + spring_length`
pub struct SpringForces {
    /// Ceiling anchor position where the spring is mounted
    pub position: Vec2,
    /// Radius of the hanging bob weight
    pub radius: f32,
    /// Minimum spring length at peak compression
    pub min_spring_length: f32,
    /// Maximum spring length at peak extension
    pub max_spring_length: f32,
    /// Period in frames for one complete oscillation cycle
    pub period: f32,
    /// Elapsed frame counter normalized by delta time
    pub frame_count: f32,
    /// Toggle between coiled spring and straight elastic wire (Space key)
    pub show_coils: bool,
}

impl SpringForces {
    pub fn new() -> Self {
        Self {
            position: Vec2::new(world_width() * 0.5, 30.0),
            radius: 22.0,
            min_spring_length: 80.0,
            max_spring_length: 360.0,
            period: 120.0,
            frame_count: 0.0,
            show_coils: false, // Default to clean straight line to eliminate 3D optical illusions
        }
    }

    /// Linear interpolation between `start` and `stop` by factor `time` in [0.0, 1.0].
    pub fn lerp(&self, start: f32, stop: f32, time: f32) -> f32 {
        start + time * (stop - start)
    }

    /// Maps a value `val` from range `[start1, stop1]` into target range `[start2, stop2]`.
    pub fn map(&self, val: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
        let t = (val - start1) / (stop1 - start1);
        self.lerp(start2, stop2, t)
    }
}

impl Example for SpringForces {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        // Toggle coil rendering with Space key
        if is_key_pressed(KeyCode::Space) {
            self.show_coils = !self.show_coils;
        }

        // Advance frame count normalized to 60 FPS using delta time
        self.frame_count += get_frame_time() * 60.0;

        // Keep anchor centered on screen resize
        self.position.x = world_width() * 0.5;
    }

    fn draw(&self) {
        draw_world_border();

        let anchor = self.position;

        // 1. Calculate the harmonic phase and vertical spring length:
        let phase = (TAU * self.frame_count) / self.period;
        let spring_length = self.map(
            phase.sin(),
            -1.0,
            1.0,
            self.min_spring_length,
            self.max_spring_length,
        );

        // Bob position is strictly vertical: anchor.y + spring_length
        let bob_x = anchor.x;
        let bob_y = anchor.y + spring_length;

        // 2. Draw equilibrium rest position guide
        let rest_length = (self.min_spring_length + self.max_spring_length) * 0.5;
        let rest_y = anchor.y + rest_length;
        draw_line(
            anchor.x - 40.0,
            rest_y,
            anchor.x + 40.0,
            rest_y,
            1.0,
            Color::new(0.4, 0.4, 0.4, 0.5),
        );

        // 3. Draw ceiling mount bracket
        draw_line(anchor.x - 45.0, anchor.y, anchor.x + 45.0, anchor.y, 4.0, GRAY);
        draw_circle(anchor.x, anchor.y, 4.0, WHITE);

        // 4. Draw spring connection
        if self.show_coils {
            // Draw coiled spring (zig-zag coils)
            let num_coils = 16;
            let coil_width = 14.0;
            let lead = 16.0;

            let spring_top = anchor.y + lead;
            let spring_bottom = bob_y - lead;
            let spring_height = (spring_bottom - spring_top).max(10.0);

            // Center vertical axis guide (keeps eye grounded to vertical motion)
            draw_line(anchor.x, anchor.y, bob_x, bob_y, 1.0, Color::new(0.25, 0.25, 0.25, 0.4));

            // Top lead
            draw_line(anchor.x, anchor.y, anchor.x, spring_top, 2.0, GRAY);

            // Zig-zag coils
            let mut prev_pt = Vec2::new(anchor.x, spring_top);
            for i in 1..=num_coils {
                let t = i as f32 / num_coils as f32;
                let py = spring_top + t * spring_height;
                let px = if i == num_coils {
                    anchor.x
                } else if i % 2 == 1 {
                    anchor.x - coil_width
                } else {
                    anchor.x + coil_width
                };
                let curr_pt = Vec2::new(px, py);
                draw_line(prev_pt.x, prev_pt.y, curr_pt.x, curr_pt.y, 2.5, WHITE);
                prev_pt = curr_pt;
            }

            // Bottom lead
            draw_line(prev_pt.x, prev_pt.y, bob_x, bob_y, 2.0, GRAY);
        } else {
            // Classic straight elastic spring line (Shiffman's original textbook representation)
            draw_line(anchor.x, anchor.y, bob_x, bob_y, 3.0, WHITE);
        }

        // 5. Draw bob (weight)
        draw_circle(bob_x, bob_y, self.radius, WHITE);
        draw_circle(bob_x, bob_y, 4.0, GRAY);

        // 6. Telemetry HUD panel
        let len_str = format!(
            "Spring Length: {:.1} px [min: {:.0}, max: {:.0}]",
            spring_length, self.min_spring_length, self.max_spring_length
        );
        let style_str = if self.show_coils {
            "Style: Coiled Spring [SPACE: switch to Wire]"
        } else {
            "Style: Straight Wire [SPACE: switch to Coiled]"
        };
        let lines = [
            ("SPRING BOB SIMULATION (SHM MAP)", WHITE),
            ("bob_y = anchor.y + map(sin(phase), -1, 1, min, max)", GRAY),
            (&len_str, GRAY),
            (style_str, GRAY),
        ];
        draw_info_panel(10.0, world_height() - 95.0, 390.0, &lines);
    }
}
