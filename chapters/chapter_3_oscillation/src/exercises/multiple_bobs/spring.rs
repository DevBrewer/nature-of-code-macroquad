#![allow(dead_code)]
use macroquad::{
    color::Color,
    shapes::draw_line,
};
use vec_math::Vec2;

use crate::exercises::multiple_bobs::bob::Bob;

#[derive(Debug, Clone, Copy)]
pub struct Spring {
    pub a: usize,
    pub b: usize,
    pub k: f32,
    pub rest_length: f32,
}

impl Spring {
    pub fn new(a: usize, b: usize, k: f32, rest_length: f32) -> Self {
        Self {
            a,
            b,
            k,
            rest_length,
        }
    }

    /// Calculate and apply spring force between two bobs (Hooke's Law: F = -k * x)
    pub fn connect(&self, a: &mut Bob, b: &mut Bob) {
        // Vector pointing from Bob a to Bob b
        let force_dir = b.position - a.position;
        let current_length = force_dir.mag();

        if current_length > 0.0 {
            let stretch = current_length - self.rest_length;
            // Hooke's Law: F = k * stretch along the direction vector
            let force = force_dir.normalized() * (self.k * stretch);

            // Newton's Third Law (equal and opposite forces):
            // Bob a is pulled towards Bob b (+force)
            // Bob b is pulled towards Bob a (-force)
            a.apply_force(force);
            b.apply_force(-force);
        }
    }

    /// Safely apply spring force to the connected pair in a slice of bobs
    pub fn update(&self, bobs: &mut [Bob]) {
        if self.a == self.b || self.a >= bobs.len() || self.b >= bobs.len() {
            return;
        }

        let (bob_a, bob_b) = if self.a < self.b {
            let (left, right) = bobs.split_at_mut(self.b);
            (&mut left[self.a], &mut right[0])
        } else {
            let (left, right) = bobs.split_at_mut(self.a);
            (&mut right[0], &mut left[self.b])
        };

        self.connect(bob_a, bob_b);
    }

    /// Render elastic spring line with dynamic tension-based color gradient
    pub fn show(&self, bobs: &[Bob]) {
        if self.a >= bobs.len() || self.b >= bobs.len() {
            return;
        }

        let pos_a = bobs[self.a].position;
        let pos_b = bobs[self.b].position;
        let d = (pos_b - pos_a).mag();
        let stretch = d - self.rest_length;

        // Dynamic color based on tension:
        // Blue/cyan when compressed, neutral cyan/white at rest, warm coral/orange when stretched
        let tension_ratio = (stretch / self.rest_length).clamp(-1.0, 1.0);
        let color = if tension_ratio > 0.0 {
            // Stretched: blend from bright sky blue (0x38bdf8) to vivid orange/coral
            Color::new(
                0.22 + 0.75 * tension_ratio,
                0.74 - 0.29 * tension_ratio,
                0.97 - 0.88 * tension_ratio,
                0.9,
            )
        } else {
            // Compressed: blend from sky blue to lavender/indigo
            let comp = -tension_ratio;
            Color::new(
                0.22 + 0.28 * comp,
                0.74 - 0.19 * comp,
                0.97,
                0.9,
            )
        };

        draw_line(pos_a.x, pos_a.y, pos_b.x, pos_b.y, 3.5, color);
    }

    /// Render coiled zig-zag spring wire
    pub fn show_coiled(&self, bobs: &[Bob]) {
        if self.a >= bobs.len() || self.b >= bobs.len() {
            return;
        }

        let pos_a = bobs[self.a].position;
        let pos_b = bobs[self.b].position;
        let dir = pos_b - pos_a;
        let total_length = dir.mag();
        if total_length == 0.0 {
            return;
        }

        let angle = dir.heading();
        let coils = 10;
        let coil_width = 8.0;
        let segment_length = total_length / coils as f32;
        let spring_color = Color::from_hex(0x38bdf8);
        let thickness = 2.5;

        let transform = |lx: f32, ly: f32| -> Vec2 {
            let local = Vec2::new(lx, ly);
            pos_a + local.rotate(angle)
        };

        let mut prev_pt = transform(0.0, 0.0);
        let next_pt = transform(segment_length * 0.5, 0.0);
        draw_line(prev_pt.x, prev_pt.y, next_pt.x, next_pt.y, thickness, spring_color);
        prev_pt = next_pt;

        for i in 0..coils {
            let x = segment_length * (0.5 + i as f32);
            let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
            let y = sign * coil_width;
            let next_pt = transform(x, y);
            draw_line(prev_pt.x, prev_pt.y, next_pt.x, next_pt.y, thickness, spring_color);
            prev_pt = next_pt;
        }

        let final_pt = transform(total_length, 0.0);
        draw_line(prev_pt.x, prev_pt.y, final_pt.x, final_pt.y, thickness, spring_color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spring_stretched_applies_restoring_force() {
        let mut bob_a = Bob::new(Vec2::new(0.0, 0.0), 10.0);
        let mut bob_b = Bob::new(Vec2::new(100.0, 0.0), 10.0);
        let spring = Spring::new(0, 1, 0.1, 50.0);

        spring.connect(&mut bob_a, &mut bob_b);

        // Bob A should be pulled towards +x (Bob B)
        // Stretch = 100 - 50 = 50. Force = 0.1 * 50 = 5.0. Accel = 5.0 / 10.0 = 0.5
        assert!((bob_a.acceleration.x - 0.5).abs() < 1e-5);
        // Bob B should be pulled towards -x (Bob A)
        assert!((bob_b.acceleration.x - (-0.5)).abs() < 1e-5);
    }

    #[test]
    fn test_spring_compressed_applies_repulsive_force() {
        let mut bob_a = Bob::new(Vec2::new(0.0, 0.0), 10.0);
        let mut bob_b = Bob::new(Vec2::new(25.0, 0.0), 10.0);
        let spring = Spring::new(0, 1, 0.2, 50.0);

        spring.connect(&mut bob_a, &mut bob_b);

        // Stretch = 25 - 50 = -25. Force = 0.2 * -25 = -5.0.
        // Bob A pushed in -x: accel = -0.5
        assert!((bob_a.acceleration.x - (-0.5)).abs() < 1e-5);
        // Bob B pushed in +x: accel = +0.5
        assert!((bob_b.acceleration.x - 0.5).abs() < 1e-5);
    }

    #[test]
    fn test_spring_equilibrium_zero_force() {
        let mut bob_a = Bob::new(Vec2::new(0.0, 0.0), 10.0);
        let mut bob_b = Bob::new(Vec2::new(50.0, 0.0), 10.0);
        let spring = Spring::new(0, 1, 0.1, 50.0);

        spring.connect(&mut bob_a, &mut bob_b);

        assert_eq!(bob_a.acceleration, Vec2::ZERO);
        assert_eq!(bob_b.acceleration, Vec2::ZERO);
    }

    #[test]
    fn test_spring_update_slice() {
        let mut bobs = vec![
            Bob::new(Vec2::new(0.0, 0.0), 10.0),
            Bob::new(Vec2::new(100.0, 0.0), 10.0),
        ];
        let spring = Spring::new(0, 1, 0.1, 50.0);
        spring.update(&mut bobs);

        assert!((bobs[0].acceleration.x - 0.5).abs() < 1e-5);
        assert!((bobs[1].acceleration.x - (-0.5)).abs() < 1e-5);
    }
}
