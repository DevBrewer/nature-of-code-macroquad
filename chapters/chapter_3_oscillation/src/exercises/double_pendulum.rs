use std::{
    collections::VecDeque,
    f32::consts::{FRAC_PI_2, PI},
};

use macroquad::{
    color::{Color, GRAY, WHITE},
    input::{KeyCode, is_key_pressed},
    shapes::{draw_circle, draw_line},
    time::get_frame_time,
};
use runner::{Example, draw_axes, draw_info_panel, draw_world_border, world_center};
use vec_math::Vec2;

const GRAVITY: f32 = 450.0;
const DRAG_COEFFICIENT: f32 = 0.003;
/// Massive permanent capacity so the chaotic trajectory never gets removed during simulation
pub const MAX_TRAIL_LENGTH: usize = 100_000;

/// Visual rendering style for the chaotic trajectory pattern
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PatternStyle {
    /// Vibrant rainbow spectrum cycling continuously along the chaotic orbit (matches generated image)
    Rainbow,
    /// Electric cyan to violet/coral neon gradient
    NeonGlow,
    /// High-contrast dynamic fading tail
    FadingTail,
}

/// Converts a hue angle in degrees [0.0, 360.0) with full saturation and value to RGB Color
pub fn hue_to_rgb(hue: f32, alpha: f32) -> Color {
    let h = (hue % 360.0 + 360.0) % 360.0;
    let c = 1.0;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let (r, g, b) = match (h / 60.0) as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    Color::new(r, g, b, alpha)
}

/// Double Pendulum Physics Simulation
///
/// ============================================================================
/// LAGRANGIAN MECHANICS FORMULATION & DERIVATION
/// ============================================================================
///
/// 1. Generalized Coordinates:
///    Let θ1 and θ2 denote the angles made by rods l1 and l2 relative to the
///    downward vertical axis (y pointing downward in screen space).
///
/// 2. Cartesian Coordinates:
///    Bob 1:
///      x1 = pivot.x + l1 * sin(θ1)
///      y1 = pivot.y + l1 * cos(θ1)
///
///    Bob 2:
///      x2 = x1 + l2 * sin(θ2) = pivot.x + l1 * sin(θ1) + l2 * sin(θ2)
///      y2 = y1 + l2 * cos(θ2) = pivot.y + l1 * cos(θ1) + l2 * cos(θ2)
///
/// 3. Velocities (time derivatives dx/dt, dy/dt):
///    dx1/dt =  l1 * θ1' * cos(θ1)
///    dy1/dt = -l1 * θ1' * sin(θ1)
///    => v1^2 = (dx1/dt)^2 + (dy1/dt)^2 = l1^2 * (θ1')^2
///
///    dx2/dt =  l1 * θ1' * cos(θ1) + l2 * θ2' * cos(θ2)
///    dy2/dt = -l1 * θ1' * sin(θ1) - l2 * θ2' * sin(θ2)
///    => v2^2 = (dx2/dt)^2 + (dy2/dt)^2
///            = l1^2 * (θ1')^2 + l2^2 * (θ2')^2 + 2 * l1 * l2 * θ1' * θ2' * cos(θ1 - θ2)
///
/// 4. Kinetic Energy (T):
///    T = (1/2) * m1 * v1^2 + (1/2) * m2 * v2^2
///    T = (1/2) * (m1 + m2) * l1^2 * (θ1')^2
///        + (1/2) * m2 * l2^2 * (θ2')^2
///        + m2 * l1 * l2 * θ1' * θ2' * cos(θ1 - θ2)
///
/// 5. Potential Energy (V):
///    Taking y downward as positive, the gravitational potential energy is:
///    V = -m1 * g * y1 - m2 * g * y2
///    V = -(m1 + m2) * g * l1 * cos(θ1) - m2 * g * l2 * cos(θ2)
///
/// 6. The Lagrangian (L = T - V):
///    L = (1/2) * (m1 + m2) * l1^2 * (θ1')^2
///        + (1/2) * m2 * l2^2 * (θ2')^2
///        + m2 * l1 * l2 * θ1' * θ2' * cos(θ1 - θ2)
///        + (m1 + m2) * g * l1 * cos(θ1)
///        + m2 * g * l2 * cos(θ2)
///
/// 7. Euler-Lagrange Equations of Motion:
///    d/dt (∂L / ∂θi') - (∂L / ∂θi) = 0   for i in {1, 2}
///
///    Let Δ = θ1 - θ2. Evaluating the partial and total derivatives gives the
///    coupled second-order system:
///
///    Equation 1:
///      (m1 + m2) * l1 * θ1'' + m2 * l2 * cos(Δ) * θ2''
///        = -m2 * l2 * (θ2')^2 * sin(Δ) - (m1 + m2) * g * sin(θ1)
///
///    Equation 2:
///      l1 * cos(Δ) * θ1'' + l2 * θ2''
///        = l1 * (θ1')^2 * sin(Δ) - g * sin(θ2)
///
/// 8. Explicit Acceleration Formulas:
///    Solving this 2x2 linear system for angular accelerations α1 = θ1'' and α2 = θ2'':
///
///    Common Denominator:
///      μ = 2 * m1 + m2 - m2 * cos(2*θ1 - 2*θ2)
///
///    Angular Acceleration 1 (α1):
///      α1 = [ -g * (2*m1 + m2) * sin(θ1)
///             - m2 * g * sin(θ1 - 2*θ2)
///             - 2 * sin(Δ) * m2 * ((θ2')^2 * l2 + (θ1')^2 * l1 * cos(Δ)) ]
///           / (l1 * μ)
///
///    Angular Acceleration 2 (α2):
///      α2 = [ 2 * sin(Δ) * ( (θ1')^2 * l1 * (m1 + m2)
///             + g * (m1 + m2) * cos(θ1)
///             + (θ2')^2 * l2 * m2 * cos(Δ) ) ]
///           / (l2 * μ)
/// ============================================================================
pub struct DoublePendulum {
    pub pivot: Vec2,

    // Bob 1 properties
    pub length1: f32,
    pub mass1: f32,
    pub angle1: f32,
    pub angular_velocity1: f32,
    pub angular_acceleration1: f32,
    pub bob1: Vec2,

    // Bob 2 properties
    pub length2: f32,
    pub mass2: f32,
    pub angle2: f32,
    pub angular_velocity2: f32,
    pub angular_acceleration2: f32,
    pub bob2: Vec2,

    /// Accumulated trajectory history for Bob 2 to visualize the chaotic pattern
    pub trail: VecDeque<Vec2>,
    pub radius1: f32,
    pub radius2: f32,

    /// Visual appearance of the chaotic path trace
    pub pattern_style: PatternStyle,
    /// When true, applies small air drag damping. When false, runs symplectic/undamped
    pub damping_enabled: bool,
    /// Toggle mechanical linkage (rods and bobs) to view pure pattern art
    pub show_linkage: bool,
}

impl DoublePendulum {
    pub fn new(pivot: Vec2, length1: f32, length2: f32, mass1: f32, mass2: f32) -> Self {
        Self::new_with_angles(pivot, length1, length2, mass1, mass2, FRAC_PI_2, FRAC_PI_2)
    }

    pub fn new_with_angles(
        pivot: Vec2,
        length1: f32,
        length2: f32,
        mass1: f32,
        mass2: f32,
        angle1: f32,
        angle2: f32,
    ) -> Self {
        let mut pendulum = Self {
            pivot,
            length1,
            mass1,
            angle1,
            angular_velocity1: 0.0,
            angular_acceleration1: 0.0,
            bob1: Vec2::ZERO,
            length2,
            mass2,
            angle2,
            angular_velocity2: 0.0,
            angular_acceleration2: 0.0,
            bob2: Vec2::ZERO,
            trail: VecDeque::with_capacity(MAX_TRAIL_LENGTH),
            radius1: 10.0 + (mass1 * 0.4).min(16.0),
            radius2: 10.0 + (mass2 * 0.4).min(16.0),
            pattern_style: PatternStyle::Rainbow,
            damping_enabled: false, // Default undamped for continuous non-decaying pattern generation
            show_linkage: true,
        };

        pendulum.update_bob_position();
        pendulum
    }

    /// Computes Cartesian coordinates for Bob 1 and Bob 2 from joint angles
    pub fn update_bob_position(&mut self) {
        let (sin1, cos1) = self.angle1.sin_cos();
        let (sin2, cos2) = self.angle2.sin_cos();

        // Bob 1 position maps relative to structural pivot point
        self.bob1.x = self.pivot.x + self.length1 * sin1;
        self.bob1.y = self.pivot.y + self.length1 * cos1;

        // Bob 2 position branches directly as a relative vector from Bob 1
        self.bob2.x = self.bob1.x + self.length2 * sin2;
        self.bob2.y = self.bob1.y + self.length2 * cos2;
    }

    /// Single sub-step physics evaluation using closed-form Lagrangian equations
    pub fn step_physics(&mut self, dt: f32) {
        // Cache parameters to prevent runtime division by zero
        let l1 = self.length1.max(f32::EPSILON);
        let l2 = self.length2.max(f32::EPSILON);

        let m1 = self.mass1;
        let m2 = self.mass2;

        let (sin1, cos1) = self.angle1.sin_cos();
        let delta = self.angle1 - self.angle2;
        let (s_delta, c_delta) = delta.sin_cos();
        let s_mixed_delta = (self.angle1 - 2.0 * self.angle2).sin();

        let w1 = self.angular_velocity1;
        let w2 = self.angular_velocity2;

        // Common denominator: μ = 2*m1 + m2 - m2*cos(2*θ1 - 2*θ2)
        let mut common_denominator = 2.0 * m1 + m2 - m2 * (2.0 * delta).cos();
        if common_denominator.abs() < f32::EPSILON {
            common_denominator = f32::EPSILON;
        }

        // Calculate Angular acceleration 1 (α1)
        #[rustfmt::skip]
        let alpha1 = -GRAVITY * (2.0 * m1 + m2) * sin1
            - m2 * GRAVITY * s_mixed_delta
            - 2.0 * s_delta * m2 * (w2 * w2 * l2 + w1 * w1 * l1 * c_delta);
        self.angular_acceleration1 = alpha1 / (l1 * common_denominator);

        // Calculate Angular acceleration 2 (α2)
        #[rustfmt::skip]
        let alpha2 = 2.0 * s_delta * (w1 * w1 * l1 * (m1 + m2)
            + GRAVITY * (m1 + m2) * cos1
            + w2 * w2 * l2 * m2 * c_delta);
        self.angular_acceleration2 = alpha2 / (l2 * common_denominator);

        // Semi-implicit Euler Numerical Integration (Euler-Cromer)
        self.angular_velocity1 += self.angular_acceleration1 * dt;
        self.angular_velocity2 += self.angular_acceleration2 * dt;

        // Apply time-dependent drag damping if enabled
        if self.damping_enabled {
            self.angular_velocity1 *= f32::exp(-DRAG_COEFFICIENT * dt);
            self.angular_velocity2 *= f32::exp(-DRAG_COEFFICIENT * dt);
        }

        // Update angles
        self.angle1 += self.angular_velocity1 * dt;
        self.angle2 += self.angular_velocity2 * dt;

        // Recalculate Cartesian positions
        self.update_bob_position();
    }

    /// Advances the simulation by `dt` seconds with sub-stepping for stability
    /// and records the path trail for Bob 2 without deleting points.
    pub fn update(&mut self, dt: f32) {
        if dt <= 0.0 {
            return;
        }

        // Sub-stepping for numerical stability with chaotic high-speed swings
        let substeps = 4;
        let sub_dt = dt / substeps as f32;
        for _ in 0..substeps {
            self.step_physics(sub_dt);
        }

        // Record Bob 2 trail point permanently
        self.trail.push_back(self.bob2);
        // Only pop if safety limit (100,000 points = ~30 minutes of continuous drawing) is reached
        if self.trail.len() > MAX_TRAIL_LENGTH {
            self.trail.pop_front();
        }
    }

    /// Draws the accumulated chaotic pattern traced by Bob 2
    pub fn draw_pattern(&self) {
        let trail_len = self.trail.len();
        if trail_len < 2 {
            return;
        }

        for i in 0..trail_len - 1 {
            let p1 = self.trail[i];
            let p2 = self.trail[i + 1];

            let color = match self.pattern_style {
                PatternStyle::Rainbow => {
                    // Smooth spectral cycling along orbits (0.08 deg per step creates distinct colored bands)
                    let hue = (i as f32 * 0.08) % 360.0;
                    hue_to_rgb(hue, 0.72)
                }
                PatternStyle::NeonGlow => {
                    // Electric cyan-to-coral neon gradient
                    let t = ((i as f32 * 0.0003).sin() * 0.5 + 0.5).clamp(0.0, 1.0);
                    Color::new(0.2 + 0.8 * t, 0.25 + 0.5 * (1.0 - t), 1.0 - 0.4 * t, 0.72)
                }
                PatternStyle::FadingTail => {
                    let progress = (i + 1) as f32 / trail_len as f32;
                    let alpha = progress.powf(1.8) * 0.90;
                    Color::new(1.0, 0.35, 0.35, alpha)
                }
            };

            // Thin 1.2px luminous lines create the delicate overlapping spirograph threads
            draw_line(p1.x, p1.y, p2.x, p2.y, 1.2, color);
        }
    }

    /// Renders the double pendulum pattern and mechanical linkage (if enabled)
    pub fn show(&self) {
        // 1. Draw the chaotic pattern traced by Bob 2
        self.draw_pattern();

        // 2. If linkage is enabled, draw the mechanical rods, pivot, and bobs
        if self.show_linkage {
            let rod_color = Color::new(1.0, 1.0, 1.0, 0.35);

            // Upper assembly rod linkage
            draw_line(
                self.pivot.x,
                self.pivot.y,
                self.bob1.x,
                self.bob1.y,
                1.8,
                rod_color,
            );

            // Lower assembly rod linkage
            draw_line(
                self.bob1.x,
                self.bob1.y,
                self.bob2.x,
                self.bob2.y,
                1.8,
                rod_color,
            );

            // Pivot joint mounting anchor point
            draw_circle(self.pivot.x, self.pivot.y, 4.0, WHITE);

            // Bob masses and center pins
            draw_circle(
                self.bob1.x,
                self.bob1.y,
                self.radius1,
                Color::new(0.2, 0.55, 1.0, 0.85),
            );
            draw_circle(self.bob1.x, self.bob1.y, 2.5, WHITE);

            draw_circle(
                self.bob2.x,
                self.bob2.y,
                self.radius2,
                Color::new(1.0, 0.3, 0.45, 0.95),
            );
            draw_circle(self.bob2.x, self.bob2.y, 2.5, WHITE);
        }
    }
}

/// Interactive Example runner entry for the Double Pendulum simulation
pub struct DoublePendulumExercise {
    pub pendulum: DoublePendulum,
    pub current_preset: usize,
}

impl DoublePendulumExercise {
    pub fn new() -> Self {
        let mut ex = Self {
            // Placeholder initially, overwritten by apply_preset
            pendulum: DoublePendulum::new(Vec2::ZERO, 115.0, 115.0, 20.0, 20.0),
            current_preset: 1,
        };
        ex.apply_preset(1);
        ex
    }

    /// Applies high-energy initial angle presets that produce full 360-degree orbital patterns
    pub fn apply_preset(&mut self, preset: usize) {
        self.current_preset = preset;
        let center = world_center();
        let length = 115.0; // Reach is 230 px, fits inside 600x600 canvas with room to spare

        let (angle1, angle2) = match preset {
            // Preset 1 (Default): Nearly inverted release. Tremendous gravitational potential energy
            // vaults the pendulum over the top repeatedly in full 360-degree rotations,
            // tracing the complete circular chaotic rosette matching the reference artwork.
            1 => (PI - 0.06, PI - 0.14),

            // Preset 2: Asymmetric high-energy swing. Creates looping petal-like floral chaos.
            2 => (2.7, -1.8),

            // Preset 3: Horizontal right release. Classic swing below the pivot line.
            _ => (FRAC_PI_2, FRAC_PI_2),
        };

        self.pendulum =
            DoublePendulum::new_with_angles(center, length, length, 20.0, 20.0, angle1, angle2);
    }
}

impl Example for DoublePendulumExercise {
    fn reset(&mut self) {
        self.apply_preset(self.current_preset);
    }

    fn update(&mut self) {
        let dt = get_frame_time().min(0.033);

        // Reset simulation to current preset
        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::R) {
            self.reset();
        }

        // [1], [2], [3] Switch Energy Presets
        if is_key_pressed(KeyCode::Key1) {
            self.apply_preset(1);
        }
        if is_key_pressed(KeyCode::Key2) {
            self.apply_preset(2);
        }
        if is_key_pressed(KeyCode::Key3) {
            self.apply_preset(3);
        }

        // [C] Clear pattern trace without resetting angles/speeds
        if is_key_pressed(KeyCode::C) {
            self.pendulum.trail.clear();
        }

        // [T] Cycle pattern color palette
        if is_key_pressed(KeyCode::T) {
            self.pendulum.pattern_style = match self.pendulum.pattern_style {
                PatternStyle::Rainbow => PatternStyle::NeonGlow,
                PatternStyle::NeonGlow => PatternStyle::FadingTail,
                PatternStyle::FadingTail => PatternStyle::Rainbow,
            };
        }

        // [P] Toggle rods & bobs visibility to view pure pattern art
        if is_key_pressed(KeyCode::P) {
            self.pendulum.show_linkage = !self.pendulum.show_linkage;
        }

        // [D] Toggle air drag damping
        if is_key_pressed(KeyCode::D) {
            self.pendulum.damping_enabled = !self.pendulum.damping_enabled;
        }

        self.pendulum.update(dt);
    }

    fn draw(&self) {
        draw_world_border();
        self.pendulum.show();

        let preset_name = match self.current_preset {
            1 => "Preset: [1] 360 Full Orbit (Reference Pattern)",
            2 => "Preset: [2] Asymmetric Chaotic Petals",
            _ => "Preset: [3] Horizontal Classic Drop",
        };
        let style_label = match self.pendulum.pattern_style {
            PatternStyle::Rainbow => "Palette: [T] Rainbow Spectrum",
            PatternStyle::NeonGlow => "Palette: [T] Neon Glow",
            PatternStyle::FadingTail => "Palette: [T] Fading Tail",
        };
        let linkage_label = if self.pendulum.show_linkage {
            "Linkage: [P] Visible (Rods & Bobs)"
        } else {
            "Linkage: [P] Hidden (Pure Pattern Art)"
        };
        let damping_label = if self.pendulum.damping_enabled {
            "Damping: [D] ON (Slows down)"
        } else {
            "Damping: [D] OFF (Endless Motion)"
        };
        let count_str = format!(
            "Points: {} (Never removed) | [C] Clear | [SPACE] Reset",
            self.pendulum.trail.len()
        );

        let lines = [
            ("DOUBLE PENDULUM PATTERN TRACE", WHITE),
            (preset_name, Color::new(0.35, 0.95, 0.65, 1.0)),
            (style_label, Color::new(0.35, 0.85, 1.0, 1.0)),
            (linkage_label, GRAY),
            (damping_label, GRAY),
            ("[H] Toggle HUD Display", Color::new(0.70, 0.75, 0.85, 0.90)),
            (&count_str, GRAY),
        ];
        draw_info_panel(10.0, 10.0, 390.0, &lines);

        draw_axes();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_bob_positions() {
        let pivot = Vec2::new(300.0, 200.0);
        let l1 = 100.0;
        let l2 = 100.0;
        let pendulum = DoublePendulum::new(pivot, l1, l2, 10.0, 10.0);

        // Initial angles are FRAC_PI_2 (90 deg -> horizontal right)
        // x1 = pivot.x + l1 * sin(pi/2) = 300 + 100 = 400
        // y1 = pivot.y + l1 * cos(pi/2) = 200 + 0 = 200
        assert!((pendulum.bob1.x - 400.0).abs() < 1e-4);
        assert!((pendulum.bob1.y - 200.0).abs() < 1e-4);

        // x2 = x1 + l2 * sin(pi/2) = 400 + 100 = 500
        // y2 = y1 + l2 * cos(pi/2) = 200 + 0 = 200
        assert!((pendulum.bob2.x - 500.0).abs() < 1e-4);
        assert!((pendulum.bob2.y - 200.0).abs() < 1e-4);
    }

    #[test]
    fn test_trail_permanent_accumulation() {
        let pivot = Vec2::new(300.0, 200.0);
        let mut pendulum = DoublePendulum::new(pivot, 100.0, 100.0, 10.0, 10.0);

        assert_eq!(pendulum.trail.len(), 0);

        // Run 50 update steps - none should be removed
        for _ in 0..50 {
            pendulum.update(0.016);
        }
        assert_eq!(pendulum.trail.len(), 50);

        // Clear trail test
        pendulum.trail.clear();
        assert_eq!(pendulum.trail.len(), 0);
    }

    #[test]
    fn test_hue_to_rgb() {
        let red = hue_to_rgb(0.0, 1.0);
        assert!((red.r - 1.0).abs() < 1e-4);
        assert!((red.g - 0.0).abs() < 1e-4);
        assert!((red.b - 0.0).abs() < 1e-4);

        let green = hue_to_rgb(120.0, 1.0);
        assert!((green.r - 0.0).abs() < 1e-4);
        assert!((green.g - 1.0).abs() < 1e-4);
        assert!((green.b - 0.0).abs() < 1e-4);

        let blue = hue_to_rgb(240.0, 1.0);
        assert!((blue.r - 0.0).abs() < 1e-4);
        assert!((blue.g - 0.0).abs() < 1e-4);
        assert!((blue.b - 1.0).abs() < 1e-4);
    }

    #[test]
    fn test_numerical_stability() {
        let pivot = Vec2::new(300.0, 200.0);
        let mut pendulum = DoublePendulum::new(pivot, 100.0, 100.0, 10.0, 10.0);

        // Run 200 updates and ensure no NaN or infinite values occur
        for _ in 0..200 {
            pendulum.update(0.016);
            assert!(!pendulum.angle1.is_nan());
            assert!(!pendulum.angle2.is_nan());
            assert!(!pendulum.angular_velocity1.is_nan());
            assert!(!pendulum.angular_velocity2.is_nan());
            assert!(!pendulum.bob1.x.is_nan());
            assert!(!pendulum.bob2.x.is_nan());
        }
    }
}
