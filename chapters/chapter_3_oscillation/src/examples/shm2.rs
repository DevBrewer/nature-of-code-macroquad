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
/// Using angle_velocity and angle
/// `x = amplitude * sin(angle)`
///
/// Key SHM concepts:
/// - Amplitude (A): Maximum displacement from the equilibrium position.
pub struct Shm2Example {
    /// Radius of the oscillating bob
    pub radius: f32,
    /// Maximum displacement from the equilibrium center to either extreme
    pub amplitude: f32,
    pub angle: f32,
    pub angle_velocity: f32,
    /// Equilibrium anchor point (center of oscillation)
    pub origin: Vec2,
}

impl Shm2Example {
    pub fn new() -> Self {
        let center = world_center();

        Self {
            radius: 20.0,
            amplitude: 150.0,
            angle: 0.0,
            angle_velocity: 0.05,
            origin: Vec2::new(center.x, center.y),
        }
    }
}

impl Example for Shm2Example {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        // Inrease angle by an angular velocity
        self.angle += self.angle_velocity * get_frame_time() * 60.0;

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
        let x = self.origin.x + self.amplitude * self.angle.sin();

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
            ("SIMPLE HARMONIC MOTION II", WHITE),
            ("x = amplitude * sin(angle)", GRAY),
            ("Amplitude: 150 px | Period: 120 frames", GRAY),
        ];
        draw_info_panel(10.0, world_height() - 75.0, 310.0, &lines);
    }
}
