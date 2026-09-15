#![allow(dead_code)]
use macroquad::{
    color::{Color, GRAY, WHITE},
    input::{
        KeyCode, MouseButton, is_key_pressed, is_mouse_button_down, is_mouse_button_pressed,
        is_mouse_button_released,
    },
    shapes::{draw_circle, draw_circle_lines, draw_line},
};
use runner::{
    Example, draw_info_panel, draw_world_border, mouse_world_position, world_height, world_width,
};
use vec_math::Vec2;

use super::{bob::Bob, spring::Spring};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpringRenderMode {
    Coiled,
    ElasticLine,
    SmoothCurve,
}

/// Exercise 3.14: Multiple Bobs and Spring Connections
///
/// Simulates a system of connected bobs and springs (Hooke's law + Newton's third law).
/// Inspired by Chapter 3 and Coding Challenge #160 (Soft Spring / Spring Forces).
///
/// Features:
/// - Chain of multiple bobs connected sequentially by springs.
/// - Users can click and drag *any* bob in the system.
/// - Toggleable fixed anchor [P] to demonstrate systems with no fixed anchor.
/// - Multiple visual modes [SPACE]: Coiled springs, Elastic tension wires, and Soft spline curve.
/// - Gravity toggle [G] and reset [R].
pub struct MultipleBobExercise {
    pub bobs: Vec<Bob>,
    pub springs: Vec<Spring>,
    pub gravity: Vec2,
    pub pinned_anchor: bool,
    pub enable_gravity: bool,
    pub render_mode: SpringRenderMode,
    pub anchor_position: Vec2,
}

impl MultipleBobExercise {
    pub fn new() -> Self {
        let num_bobs = 10;
        let spacing = 20.0;
        let k = 0.15;
        let anchor_pos = Vec2::new(world_width() * 0.5, 60.0);

        let mut bobs = Vec::with_capacity(num_bobs);
        for i in 0..num_bobs {
            let pos = Vec2::new(anchor_pos.x, anchor_pos.y + (i as f32) * spacing);
            // Mass: tail bob is slightly heavier, creating realistic pendulum drag
            let mass = if i == num_bobs - 1 { 12.0 } else { 8.0 };
            bobs.push(Bob::new(pos, mass));
        }

        let mut springs = Vec::with_capacity(num_bobs - 1);
        for i in 1..num_bobs {
            springs.push(Spring::new(i - 1, i, k, spacing));
        }

        Self {
            bobs,
            springs,
            gravity: Vec2::new(0.0, 0.35),
            pinned_anchor: true,
            enable_gravity: true,
            render_mode: SpringRenderMode::Coiled,
            anchor_position: anchor_pos,
        }
    }

    /// Catmull-Rom spline interpolation between p1 and p2 given p0 and p3 controls
    fn catmull_rom(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f32) -> Vec2 {
        let t2 = t * t;
        let t3 = t2 * t;

        let v0 = (p2 - p0) * 0.5;
        let v1 = (p3 - p1) * 0.5;

        p1 * (2.0 * t3 - 3.0 * t2 + 1.0)
            + p2 * (-2.0 * t3 + 3.0 * t2)
            + v0 * (t3 - 2.0 * t2 + t)
            + v1 * (t3 - t2)
    }

    /// Render soft organic curve through all bobs (p5 curveVertex style)
    fn draw_smooth_curve(&self) {
        if self.bobs.len() < 2 {
            return;
        }

        let pts: Vec<Vec2> = self.bobs.iter().map(|b| b.position).collect();
        let n = pts.len();
        let segments_per_span = 12;
        let curve_color = Color::from_hex(0xfcee21); // Vivid yellow

        for i in 0..n - 1 {
            let p0 = if i == 0 { pts[0] } else { pts[i - 1] };
            let p1 = pts[i];
            let p2 = pts[i + 1];
            let p3 = if i + 2 < n { pts[i + 2] } else { pts[n - 1] };

            let mut prev = p1;
            for step in 1..=segments_per_span {
                let t = step as f32 / segments_per_span as f32;
                let curr = Self::catmull_rom(p0, p1, p2, p3, t);
                draw_line(prev.x, prev.y, curr.x, curr.y, 6.0, curve_color);
                prev = curr;
            }
        }
    }
}

impl Example for MultipleBobExercise {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        // Adjust anchor when window is resized
        self.anchor_position.x = world_width() * 0.5;

        // Key interactions
        if is_key_pressed(KeyCode::Space) {
            self.render_mode = match self.render_mode {
                SpringRenderMode::Coiled => SpringRenderMode::ElasticLine,
                SpringRenderMode::ElasticLine => SpringRenderMode::SmoothCurve,
                SpringRenderMode::SmoothCurve => SpringRenderMode::Coiled,
            };
        }

        if is_key_pressed(KeyCode::P) {
            self.pinned_anchor = !self.pinned_anchor;
        }

        if is_key_pressed(KeyCode::G) {
            self.enable_gravity = !self.enable_gravity;
        }

        if is_key_pressed(KeyCode::R) {
            self.reset();
            return;
        }

        // Mouse interactions: click & drag any bob
        let mouse = mouse_world_position();

        if is_mouse_button_pressed(MouseButton::Left) {
            for bob in self.bobs.iter_mut() {
                bob.handle_mouse_press(mouse);
            }
        }

        if is_mouse_button_down(MouseButton::Left) {
            for bob in self.bobs.iter_mut() {
                bob.handle_mouse_drag(mouse);
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            for bob in self.bobs.iter_mut() {
                bob.handle_mouse_release();
            }
        }

        // 1. Calculate & apply Hooke's Law spring forces across all connections
        for spring in &self.springs {
            spring.update(&mut self.bobs);
        }

        // 2. Apply external forces (gravity) & integrate physics
        for (i, bob) in self.bobs.iter_mut().enumerate() {
            // Anchor bob pinning
            if i == 0 && self.pinned_anchor && !bob.is_dragging {
                bob.position = self.anchor_position;
                bob.velocity = Vec2::ZERO;
                bob.acceleration = Vec2::ZERO;
                continue;
            }

            if !bob.is_dragging {
                if self.enable_gravity {
                    // Apply gravity scaled by mass so all bobs experience identical gravitational acceleration
                    bob.apply_force(self.gravity * bob.mass);
                }
                bob.update();
            }
        }
    }

    fn draw(&self) {
        draw_world_border();

        // 1. Draw anchor bracket if pinned
        if self.pinned_anchor {
            let anchor = self.anchor_position;
            draw_line(
                anchor.x - 30.0,
                anchor.y - 12.0,
                anchor.x + 30.0,
                anchor.y - 12.0,
                4.0,
                GRAY,
            );
            draw_circle(anchor.x, anchor.y, 6.0, Color::from_hex(0x64748b));
            draw_circle_lines(anchor.x, anchor.y, 6.0, 2.0, Color::from_hex(0xcbd5e1));
        }

        // 2. Render springs according to current display mode
        match self.render_mode {
            SpringRenderMode::Coiled => {
                for spring in &self.springs {
                    spring.show_coiled(&self.bobs);
                }
            }
            SpringRenderMode::ElasticLine => {
                for spring in &self.springs {
                    spring.show(&self.bobs);
                }
            }
            SpringRenderMode::SmoothCurve => {
                self.draw_smooth_curve();
            }
        }

        // 3. Render all bobs
        for bob in &self.bobs {
            bob.show();
        }

        // 4. Telemetry HUD
        let anchor_status = if self.pinned_anchor {
            "Anchor: PINNED (P to release)"
        } else {
            "Anchor: FREE FLOATING (P to pin)"
        };
        let grav_status = if self.enable_gravity {
            "Gravity: ON (G to toggle)"
        } else {
            "Gravity: OFF (G to toggle)"
        };
        let mode_name = match self.render_mode {
            SpringRenderMode::Coiled => "Coiled Springs",
            SpringRenderMode::ElasticLine => "Elastic Tension Wires",
            SpringRenderMode::SmoothCurve => "Smooth Spline Curve",
        };
        let style_status = format!("Style: {} [SPACE to cycle]", mode_name);

        let lines = [
            ("EXERCISE 3.14: MULTIPLE BOBS & SPRINGS", WHITE),
            ("Hooke's Law F = -k * x with Newton's 3rd Law", GRAY),
            ("Interaction: Drag ANY bob with Left Mouse Button", WHITE),
            (anchor_status, Color::from_hex(0x38bdf8)),
            (grav_status, Color::from_hex(0x4ade80)),
            (&style_status, Color::from_hex(0xfacc15)),
        ];
        draw_info_panel(12.0, world_height() - 135.0, 430.0, &lines);
    }
}
