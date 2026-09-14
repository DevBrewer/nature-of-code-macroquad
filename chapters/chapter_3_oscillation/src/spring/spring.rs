#![allow(dead_code)]
use macroquad::{
    color::Color,
    shapes::{draw_circle, draw_circle_lines, draw_line},
};
use vec_math::Vec2;

use super::bob::Bob;

pub struct Spring {
    pub anchor: Vec2,
    pub rest_length: f32,
    pub k: f32, // Spring Constant
}

impl Spring {
    pub fn new(anchor: Vec2, rest_length: f32, k: f32) -> Self {
        Self {
            anchor,
            rest_length,
            k,
        }
    }

    pub fn connect(&mut self, bob: &mut Bob) {
        // Vector point from anchor to bob location
        let force_dir = bob.position - self.anchor;
        let current_length = force_dir.mag();

        if current_length > 0.0 {
            // Hooke's Law = Fₛ = -K * displacement
            let displacement = current_length - self.rest_length;
            let spring_force_magnitude = -self.k * displacement;

            // Unit Vector pointing along the  spring direction
            let spring_force = force_dir.normalized() * spring_force_magnitude;
            bob.apply_force(spring_force);
        }
    }

    /// Constrain spring between min and max length
    pub fn constrain_length(&self, bob: &mut Bob, min_len: f32, max_len: f32) {
        let dir = bob.position - self.anchor;
        let d = dir.mag();
        if d < min_len {
            bob.position = self.anchor + dir.normalized() * min_len;
            bob.velocity = Vec2::ZERO;
        } else if d > max_len {
            bob.position = self.anchor + dir.normalized() * max_len;
            bob.velocity = Vec2::ZERO;
        }
    }

    pub fn show(&self, bob: &Bob) {
        self.draw_anchor();
        self.draw_spring(bob);
    }

    fn draw_anchor(&self) {
        // Draw anchor
        draw_circle(self.anchor.x, self.anchor.y, 6.0, Color::from_hex(0x64748b));

        // Outer Stroke ring (#cbd5e1)
        draw_circle_lines(
            self.anchor.x,
            self.anchor.y,
            6.0,
            2.0,
            Color::from_hex(0xcbd5e1),
        );
    }

    fn draw_spring(&self, bob: &Bob) {
        // Point from anchor to bob
        let direction = bob.position - self.anchor;
        let total_length = direction.mag();

        if total_length == 0.0 {
            return;
        }

        // dy.atan2(dx) Angle pointing from anchor to bob
        let angle = direction.heading();

        //  Draw coiled spring dynamically
        let coils = 16;
        let coil_width = 14.0;
        let segment_length = total_length / coils as f32;

        let spring_color = Color::from_hex(0x38bdf8);
        let thickness = 4.0;

        // Transform local spring  point to world space using Vec2::rotate and Add
        let transform = |lx: f32, ly: f32| -> Vec2 {
            let local = Vec2::new(lx, ly);
            self.anchor + local.rotate(angle)
        };

        let mut prev_pt = transform(0.0, 0.0);

        // Initial lead-in
        let next_pt = transform(segment_length * 0.5, 0.0);
        draw_line(
            prev_pt.x,
            prev_pt.y,
            next_pt.x,
            next_pt.y,
            thickness,
            spring_color,
        );

        // Coil loop
        for i in 0..coils {
            let x = segment_length * (0.5 + i as f32);
            let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
            let y = sign * coil_width;

            let next_pt = transform(x, y);
            draw_line(
                prev_pt.x,
                prev_pt.y,
                next_pt.x,
                next_pt.y,
                thickness,
                spring_color,
            );
            prev_pt = next_pt;
        }
        // Lead-out to bob
        let final_pt = transform(total_length, 0.0);
        draw_line(
            prev_pt.x,
            prev_pt.y,
            final_pt.x,
            final_pt.y,
            thickness,
            spring_color,
        );
    }
}
