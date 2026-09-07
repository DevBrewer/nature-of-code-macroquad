use std::f32::consts::TAU;
use vec_math::Vec2;

use macroquad::{
    color::{Color, GRAY, WHITE},
    shapes::{draw_circle, draw_line},
    time::get_frame_time,
};
use runner::{Example, draw_info_panel, draw_world_border, world_center, world_height};

/// Example 3.5.1: Simple Harmonic Motion (SHM)
///
/// Demonstrates periodic oscillation modeled by a sine function:
/// `x = amplitude * sin(2*PI * t / period)`
///
/// Key SHM concepts:
/// - Amplitude (A): Maximum displacement from the equilibrium position.
/// - Period (T): Time/frames required to complete one full cycle of motion.
/// - Frequency (f = 1/T): Number of cycles completed per unit time.
pub struct ShmExample {
    /// Radius of the oscillating bob
    pub radius: f32,
    /// Maximum displacement from the equilibrium center to either extreme
    pub amplitude: f32,
    /// Duration (in frames) for one complete cycle of motion
    pub period: f32,
    /// Equilibrium anchor point (center of oscillation)
    pub origin: Vec2,
    /// Elapsed frame counter (scaled by delta time)
    pub frame_count: f32,
}

impl ShmExample {
    pub fn new() -> Self {
        let center = world_center();

        Self {
            radius: 20.0,
            amplitude: 150.0,
            period: 120.0,
            frame_count: 0.0,
            origin: Vec2::new(center.x, center.y),
        }
    }
}

impl Example for ShmExample {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        // Advance frame count normalized to 60 FPS using delta time
        self.frame_count += get_frame_time() * 60.0;
        // Keep origin updated to canvas center if resized
        self.origin = world_center();
    }

    fn draw(&self) {
        draw_world_border();

        // Simple Harmonic Motion equation:
        // x = origin.x + amplitude * sin(2*PI * t / period)
        //
        // The sine function oscillates smoothly between -1.0 and +1.0.
        // Multiplying by amplitude scales the displacement to [-amplitude, +amplitude].
        let phase = (TAU * self.frame_count) / self.period;
        let x = self.origin.x + self.amplitude * phase.sin();

        // Draw oscillation range track (amplitude bounds)
        draw_line(
            self.origin.x - self.amplitude,
            self.origin.y,
            self.origin.x + self.amplitude,
            self.origin.y,
            1.0,
            Color::new(0.3, 0.3, 0.3, 0.6),
        );

        // Draw connecting line from equilibrium center to current position
        draw_line(self.origin.x, self.origin.y, x, self.origin.y, 2.0, WHITE);

        // Draw equilibrium anchor point
        draw_circle(self.origin.x, self.origin.y, 4.0, GRAY);

        // Draw oscillating bob circle
        draw_circle(x, self.origin.y, self.radius, WHITE);

        // Information HUD panel
        let lines = [
            ("SIMPLE HARMONIC MOTION", WHITE),
            ("x = amplitude * sin(2*PI * t / period)", GRAY),
            ("Amplitude: 150 px | Period: 120 frames", GRAY),
        ];
        draw_info_panel(10.0, world_height() - 75.0, 310.0, &lines);
    }
}
