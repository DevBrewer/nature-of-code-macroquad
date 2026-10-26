use std::f32::consts::{PI, TAU};

use macroquad::{
    color::Color,
    math::vec2,
    rand::gen_range,
    shapes::{draw_circle, draw_line, draw_triangle},
};
use runner::world_size;
use vec_math::Vec2;

pub struct Jellyfish {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub bell_radius: f32,
    pub pulse_phase: f32, // Advances with time to drive contraction
    pub pulse_rate: f32,  // Speed of pulsing (frequency)
    pub tentacle_count: usize,
    pub tentacle_segments: usize,
    pub color: Color,
}

impl Jellyfish {
    pub fn new(position: Vec2) -> Self {
        let pulse_rate = gen_range(1.6, 2.6);
        let bell_radius = gen_range(14.0, 22.0);
        let pulse_phase = gen_range(0.0, TAU);

        // Bioluminescent underwater palette: lavender, cyan, or sky-blue
        let hue_type = gen_range(0, 3);
        let color = match hue_type {
            0 => Color::new(0.88, 0.48, 0.95, 0.55), // Translucent Lavender / Pink
            1 => Color::new(0.25, 0.90, 0.95, 0.55), // Translucent Cyan / Aqua
            _ => Color::new(0.40, 0.65, 1.00, 0.55), // Translucent Sky Blue
        };

        Self {
            position,
            velocity: Vec2::new(gen_range(-4.0, 4.0), gen_range(-8.0, -2.0)),
            acceleration: Vec2::ZERO,
            bell_radius,
            pulse_phase,
            pulse_rate,
            tentacle_count: 5,
            tentacle_segments: 8,
            color,
        }
    }

    pub fn random_in_world(margin: f32) -> Self {
        let Vec2 {
            x: width,
            y: height,
        } = world_size();

        let x = gen_range(margin, (width - margin).max(margin + 10.0));
        let y = gen_range(margin + 40.0, (height - margin).max(margin + 50.0));
        Self::new(Vec2::new(x, y))
    }

    pub fn update(&mut self, dt: f32) {
        self.pulse_phase += self.pulse_rate * dt;

        // Oscillating pulse dynamics:
        // Contraction (propulsion stroke): when sin(phase) is high, pulse thrust pushes upward
        let pulse = self.pulse_phase.sin();
        let contraction = pulse.max(0.0);
        let thrust_force = Vec2::new(
            (self.pulse_phase * 0.4).sin() * 6.0, // slight gentle sway left/right
            -50.0 * contraction * contraction,    // upward propulsion burst
        );
        self.acceleration += thrust_force;

        // Buoyancy / sinking: gentle gravity pulls downward during relaxation
        self.acceleration += Vec2::new(0.0, 14.0);

        // Fluid drag resisting motion
        self.acceleration += self.velocity * -1.2;

        // Euler semi-implicit integration
        self.velocity += self.acceleration * dt;
        self.position += self.velocity * dt;
        self.acceleration = Vec2::ZERO;

        // Boundary wrapping
        let Vec2 {
            x: width,
            y: height,
        } = world_size();
        let wrap_margin = self.bell_radius * 2.5;

        if self.position.y < -wrap_margin {
            self.position.y = height + wrap_margin;
        } else if self.position.y > height + wrap_margin + 60.0 {
            self.position.y = -wrap_margin;
        }

        if self.position.x < -wrap_margin {
            self.position.x = width + wrap_margin;
        } else if self.position.x > width + wrap_margin {
            self.position.x = -wrap_margin;
        }
    }

    pub fn draw(&self) {
        let pulse = self.pulse_phase.sin();
        // Bell morphs: narrows horizontally & elongates vertically when contracting
        let radius_x = self.bell_radius * (1.0 - pulse * 0.22);
        let radius_y = self.bell_radius * (0.9 + pulse * 0.25);

        // Draw undulating tentacles trailing below the bell
        if self.tentacle_count > 1 {
            let spacing = (2.0 * radius_x) / (self.tentacle_count - 1) as f32;
            for t_idx in 0..self.tentacle_count {
                let start_x = (self.position.x - radius_x) + (t_idx as f32 * spacing);
                let mut prev = Vec2::new(start_x, self.position.y);

                let seg_len = self.bell_radius * 0.42;
                for s_idx in 1..=self.tentacle_segments {
                    // Traveling sine wave down each tentacle
                    let wave_phase =
                        self.pulse_phase * 1.8 - (s_idx as f32 * 0.55) + (t_idx as f32 * 0.3);
                    let sway = wave_phase.sin() * (2.5 + s_idx as f32 * 1.0);
                    let curr = Vec2::new(prev.x + sway * 0.3, prev.y + seg_len);

                    let alpha = (self.color.a
                        * (1.0 - (s_idx as f32 / (self.tentacle_segments as f32 + 2.0))))
                        .max(0.15);
                    let mut seg_color = self.color;
                    seg_color.a = alpha;

                    draw_line(prev.x, prev.y, curr.x, curr.y, 1.8, seg_color);
                    prev = curr;
                }
            }
        }

        // Draw bell dome (fan of triangles for semi-ellipse)
        let segments = 16;
        let mut prev_pt = Vec2::new(self.position.x - radius_x, self.position.y);
        let rim_color = Color::new(
            (self.color.r * 1.2).min(1.0),
            (self.color.g * 1.2).min(1.0),
            (self.color.b * 1.2).min(1.0),
            (self.color.a * 1.3).min(1.0),
        );

        for i in 1..=segments {
            let angle = PI + (PI * (i as f32) / (segments as f32));
            let curr_pt = Vec2::new(
                self.position.x + radius_x * angle.cos(),
                self.position.y + radius_y * angle.sin(),
            );

            // Filled triangle segment
            draw_triangle(
                vec2(self.position.x, self.position.y),
                vec2(prev_pt.x, prev_pt.y),
                vec2(curr_pt.x, curr_pt.y),
                self.color,
            );

            // Outer dome perimeter line
            draw_line(prev_pt.x, prev_pt.y, curr_pt.x, curr_pt.y, 2.0, rim_color);
            prev_pt = curr_pt;
        }

        // Bottom rim line
        draw_line(
            self.position.x - radius_x,
            self.position.y,
            self.position.x + radius_x,
            self.position.y,
            2.0,
            rim_color,
        );

        // Inner glowing bioluminescent core organ
        let core_pulse = (self.pulse_phase * 2.0).sin() * 0.5 + 0.5;
        let core_color = Color::new(1.0, 1.0, 1.0, 0.35 + core_pulse * 0.35);
        draw_circle(
            self.position.x,
            self.position.y - radius_y * 0.35,
            radius_x * 0.22,
            core_color,
        );
    }
}
