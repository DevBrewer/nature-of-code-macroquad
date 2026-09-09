use std::f32::consts::TAU;

use macroquad::{
    color::{Color, WHITE},
    input::{KeyCode, is_key_pressed},
    rand::gen_range,
    shapes::{draw_circle, draw_line},
    time::get_frame_time,
};
use runner::{
    Example, draw_info_panel, draw_world_border, world_center, world_height, world_width,
};

/// Exercise 3.12: Additive Waves
///
/// Demonstrates wave superposition by adding multiple sine waves together to produce
/// a complex, undulating waveform:
/// `y_total(x) = sum( amplitude_i * sin(phase_i + TAU * x / period_i) )`
///
/// Each wave carries its own amplitude, period, and phase.
/// As the phases advance over time, the composite wave exhibits dynamic constructive
/// and destructive interference.
///
/// Controls:
/// - [Space]: Toggle animation (Pause / Resume)
/// - [Up] / [Down]: Add / remove waves (1 to 10)
/// - [L]: Toggle vertical stem lines to equilibrium
/// - [R]: Re-randomize wave components
#[derive(Clone, Debug)]
pub struct Wave {
    pub phase: f32,
    pub amplitude: f32,
    pub period: f32,
    pub phase_velocity: f32,
}

impl Wave {
    pub fn new(phase: f32, amplitude: f32, period: f32, phase_velocity: f32) -> Self {
        Self {
            phase,
            amplitude,
            period,
            phase_velocity,
        }
    }

    /// Generates a randomized wave component with sensible defaults
    pub fn random() -> Self {
        Self::new(
            gen_range(0.0, TAU),
            // Individual amplitudes are scaled so the superposition fits comfortably on canvas
            gen_range(12.0, 36.0),
            gen_range(150.0, 550.0),
            gen_range(0.01, 0.025),
        )
    }

    /// Evaluates the vertical displacement of this wave at horizontal position `x`
    pub fn evaluate(&self, x: f32) -> f32 {
        let angle = TAU * x / self.period;
        (self.phase + angle).sin() * self.amplitude
    }

    /// Advances the wave phase over time
    pub fn update(&mut self, dt_scale: f32) {
        self.phase += self.phase_velocity * dt_scale;
    }
}

/// Computes a color along the multi-stop gradient defined in Exercise 3.12:
/// - 0.00: #8bacf6 (rgb: 139, 172, 246)
/// - 0.33: #fffff9 (rgb: 255, 255, 249)
/// - 0.66: #fffd93 (rgb: 255, 253, 147)
/// - 1.00: #ffb544 (rgb: 255, 181, 68)
pub fn sample_gradient(t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);

    const C0: Color = Color::new(139.0 / 255.0, 172.0 / 255.0, 246.0 / 255.0, 1.0);
    const C1: Color = Color::new(255.0 / 255.0, 255.0 / 255.0, 249.0 / 255.0, 1.0);
    const C2: Color = Color::new(255.0 / 255.0, 253.0 / 255.0, 147.0 / 255.0, 1.0);
    const C3: Color = Color::new(255.0 / 255.0, 181.0 / 255.0, 68.0 / 255.0, 1.0);

    if t <= 0.33 {
        let factor = t / 0.33;
        lerp_color(C0, C1, factor)
    } else if t <= 0.66 {
        let factor = (t - 0.33) / 0.33;
        lerp_color(C1, C2, factor)
    } else {
        let factor = (t - 0.66) / 0.34;
        lerp_color(C2, C3, factor)
    }
}

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color::new(
        a.r + (b.r - a.r) * t,
        a.g + (b.g - a.g) * t,
        a.b + (b.b - a.b) * t,
        a.a + (b.a - a.a) * t,
    )
}

pub struct AdditiveWavesExample {
    pub waves: Vec<Wave>,
    pub is_animated: bool,
    pub show_stems: bool,
    pub x_spacing: f32,
    pub radius: f32,
}

/// Alias for compatibility
#[allow(dead_code)]
pub type WaveExample = AdditiveWavesExample;

impl AdditiveWavesExample {
    pub fn new() -> Self {
        let initial_wave_count = 5;
        let waves = (0..initial_wave_count).map(|_| Wave::random()).collect();

        Self {
            waves,
            is_animated: true,
            show_stems: true,
            x_spacing: 10.0,
            radius: 4.0,
        }
    }
}

impl Example for AdditiveWavesExample {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        // Toggle animation pause/resume
        if is_key_pressed(KeyCode::Space) {
            self.is_animated = !self.is_animated;
        }

        // Toggle vertical equilibrium stems
        if is_key_pressed(KeyCode::L) {
            self.show_stems = !self.show_stems;
        }

        // Re-randomize waves
        if is_key_pressed(KeyCode::R) {
            let count = self.waves.len();
            self.waves = (0..count).map(|_| Wave::random()).collect();
        }

        // Add wave (up to 10)
        if is_key_pressed(KeyCode::Up) && self.waves.len() < 10 {
            self.waves.push(Wave::random());
        }

        // Remove wave (down to 1)
        if is_key_pressed(KeyCode::Down) && self.waves.len() > 1 {
            self.waves.pop();
        }

        // Advance wave phases when animated
        if self.is_animated {
            let dt_scale = get_frame_time() * 60.0;
            for wave in &mut self.waves {
                wave.update(dt_scale);
            }
        }
    }

    fn draw(&self) {
        draw_world_border();
        let w = world_width();
        let center = world_center();

        // Equilibrium center line
        draw_line(
            0.0,
            center.y,
            w,
            center.y,
            1.0,
            Color::new(0.3, 0.3, 0.3, 0.5),
        );

        // Precompute wave values across screen to draw stems, connected lines, and circles
        let mut prev_point: Option<(f32, f32)> = None;
        let mut x = 0.0;

        while x <= w {
            // Superposition: sum displacements from all waves
            let y: f32 = self.waves.iter().map(|wave| wave.evaluate(x)).sum();
            let circle_y = center.y + y;

            // Color from the linear horizontal gradient
            let t = x / w;
            let col = sample_gradient(t);

            // Optional vertical stems connecting equilibrium center to wave displacement
            if self.show_stems {
                draw_line(
                    x,
                    center.y,
                    x,
                    circle_y,
                    1.0,
                    Color::new(col.r, col.g, col.b, 0.25),
                );
            }

            // Connecting line between consecutive points for a smooth wave ribbon
            if let Some((px, py)) = prev_point {
                draw_line(
                    px,
                    py,
                    x,
                    circle_y,
                    2.0,
                    Color::new(col.r, col.g, col.b, 0.85),
                );
            }
            prev_point = Some((x, circle_y));

            // Circle marker at the wave height
            draw_circle(x, circle_y, self.radius, col);

            x += self.x_spacing;
        }

        // Information HUD panel
        let mode_text = if self.is_animated {
            "Mode: Dynamic [Space: Pause]"
        } else {
            "Mode: Static / Paused [Space: Resume]"
        };

        let count_text = format!("Waves: {} [Up/Down: Adjust (1-10)]", self.waves.len());
        let stems_text = if self.show_stems {
            "Stems: Visible [L: Hide]"
        } else {
            "Stems: Hidden [L: Show]"
        };

        let lines = [
            ("y = sum( A_i * sin(phase_i + 2*PI*x / period_i) )", WHITE),
            (mode_text, Color::new(0.4, 0.8, 1.0, 1.0)),
            (&count_text, Color::new(0.8, 0.8, 0.8, 1.0)),
            (stems_text, Color::new(0.7, 0.7, 0.7, 1.0)),
            ("[R] Re-randomize waves", WHITE),
        ];

        draw_info_panel(10.0, world_height() - 165.0, 360.0, &lines);
    }
}
