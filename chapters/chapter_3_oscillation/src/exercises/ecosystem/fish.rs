use std::f32::consts::TAU;

use macroquad::{
    color::{BLACK, Color, WHITE},
    math::vec2,
    rand::gen_range,
    shapes::{draw_circle, draw_line, draw_triangle},
};
use runner::world_size;
use vec_math::Vec2;

use super::food::Plankton;

pub struct Fish {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub max_speed: f32,
    pub max_force: f32,
    pub size: f32,
    pub swim_phase: f32,
    pub tail_frequency: f32,
    pub color: Color,
    pub wander_angle: f32,
    pub eaten_count: usize,
}

impl Fish {
    pub fn new(position: Vec2) -> Self {
        let angle = gen_range(0.0, TAU);
        let speed = gen_range(30.0, 50.0);
        let size = gen_range(13.0, 18.0);

        // Vibrant aquatic fish color palette (goldfish, coral orange, electric cyan, jade)
        let hue_choice = gen_range(0, 4);
        let color = match hue_choice {
            0 => Color::new(1.0, 0.55, 0.15, 0.95),  // Coral Orange
            1 => Color::new(1.0, 0.80, 0.20, 0.95),  // Golden Yellow
            2 => Color::new(0.20, 0.85, 0.80, 0.95), // Tropical Turquoise
            _ => Color::new(0.35, 0.95, 0.45, 0.95), // Seafoam Jade
        };

        Self {
            position,
            velocity: Vec2::from_angle(angle) * speed,
            acceleration: Vec2::ZERO,
            max_speed: gen_range(80.0, 120.0),
            max_force: 140.0,
            size,
            swim_phase: gen_range(0.0, TAU),
            tail_frequency: 5.0,
            color,
            wander_angle: angle,
            eaten_count: 0,
        }
    }

    pub fn random_in_world(margin: f32) -> Self {
        let Vec2 {
            x: width,
            y: height,
        } = world_size();

        let x = gen_range(margin, (width - margin).max(margin + 10.0));
        let y = gen_range(margin, (height - margin).max(margin + 10.0));
        Self::new(Vec2::new(x, y))
    }

    pub fn update(&mut self, dt: f32, foods: &mut [Plankton]) {
        // Search for closest active food particle (within perception radius)
        let mut closest_idx: Option<usize> = None;
        let mut min_dist_sq = 220.0 * 220.0; // 220px perception radius

        for (i, food) in foods.iter().enumerate() {
            if !food.is_alive {
                continue;
            }
            let d_sq = self.position.distance_squared(food.position);
            if d_sq < min_dist_sq {
                min_dist_sq = d_sq;
                closest_idx = Some(i);
            }
        }

        if let Some(idx) = closest_idx {
            let target_pos = foods[idx].position;
            let dist = self.position.distance(target_pos);

            // Eat food when close enough
            if dist < self.size + 8.0 {
                foods[idx].is_alive = false;
                self.eaten_count += 1;
                // Brief acceleration boost from feeding
                self.velocity *= 1.25;
            } else {
                // Steer towards food
                let desired = (target_pos - self.position).normalized() * self.max_speed;
                let mut steer = desired - self.velocity;
                steer.limit(self.max_force);
                self.acceleration += steer;
            }

            // Hunting: tail beats faster!
            self.tail_frequency = 9.5;
        } else {
            // Smooth wandering cruise when no food is nearby
            self.wander_angle += gen_range(-0.6, 0.6);
            let wander_dir = Vec2::from_angle(self.wander_angle) * (self.max_speed * 0.6);
            let mut wander_steer = wander_dir - self.velocity;
            wander_steer.limit(self.max_force * 0.4);
            self.acceleration += wander_steer;

            // Cruising: tail wags at relaxed pace
            self.tail_frequency = 4.8;
        }

        //  Advance tail swimming oscillation phase
        self.swim_phase += self.tail_frequency * dt;

        // Semi-implicit Euler integration
        self.velocity += self.acceleration * dt;
        self.velocity.limit(self.max_speed);
        self.position += self.velocity * dt;
        self.acceleration = Vec2::ZERO;

        // Boundary wrapping
        let Vec2 {
            x: width,
            y: height,
        } = world_size();
        let margin = self.size * 2.0;

        if self.position.x < -margin {
            self.position.x = width + margin;
        } else if self.position.x > width + margin {
            self.position.x = -margin;
        }

        if self.position.y < -margin {
            self.position.y = height + margin;
        } else if self.position.y > height + margin {
            self.position.y = -margin;
        }
    }

    pub fn draw(&self) {
        // Heading rotation: fish rotates to face its velocity vector
        let heading = self.velocity.heading();

        // Helper to rotate local points into world coordinates
        let to_world = |pt: Vec2| self.position + pt.rotate(heading);

        // Local body keypoints:
        let tip = Vec2::new(self.size * 1.35, 0.0);
        let top = Vec2::new(0.0, -self.size * 0.48);
        let bottom = Vec2::new(0.0, self.size * 0.48);
        let tail_base = Vec2::new(-self.size * 0.95, 0.0);

        // Oscillating tail fin (Simple Harmonic Motion):
        // Tail wags with sine wave relative to the body spine
        let tail_wag = self.swim_phase.sin() * 0.48;
        let fin_len = self.size * 0.85;
        let fin_top = tail_base + Vec2::new(-fin_len, -self.size * 0.6).rotate(tail_wag);
        let fin_bottom = tail_base + Vec2::new(-fin_len, self.size * 0.6).rotate(tail_wag);

        let w_tip = to_world(tip);
        let w_top = to_world(top);
        let w_bottom = to_world(bottom);
        let w_tail = to_world(tail_base);
        let w_fin_top = to_world(fin_top);
        let w_fin_bottom = to_world(fin_bottom);

        // Colors
        let mut tail_color = self.color;
        tail_color.a = 0.75;

        let mut flank_darker = self.color;
        flank_darker.r *= 0.82;
        flank_darker.g *= 0.82;
        flank_darker.b *= 0.82;

        // Draw wagging fin
        draw_triangle(
            vec2(w_tail.x, w_tail.y),
            vec2(w_fin_top.x, w_fin_top.y),
            vec2(w_fin_bottom.x, w_fin_bottom.y),
            tail_color,
        );

        // Draw body front triangle (snout to flanks)
        draw_triangle(
            vec2(w_tip.x, w_tip.y),
            vec2(w_top.x, w_top.y),
            vec2(w_bottom.x, w_bottom.y),
            self.color,
        );

        // Draw body rear triangle (flanks to tail base)
        draw_triangle(
            vec2(w_tail.x, w_tail.y),
            vec2(w_top.x, w_top.y),
            vec2(w_bottom.x, w_bottom.y),
            flank_darker,
        );

        // Subtle outline lines along fish spine
        draw_line(w_tip.x, w_tip.y, w_top.x, w_top.y, 1.5, WHITE);
        draw_line(w_tip.x, w_tip.y, w_bottom.x, w_bottom.y, 1.5, flank_darker);

        // Symmetrical eyes positioned forward on the head near the snout
        let eye_forward = self.size * 0.95;
        let eye_lateral = self.size * 0.22;
        let eye_radius = self.size * 0.15;
        let pupil_radius = self.size * 0.08;

        // Left eye
        let left_eye = to_world(Vec2::new(eye_forward, -eye_lateral));
        draw_circle(left_eye.x, left_eye.y, eye_radius, WHITE);
        draw_circle(left_eye.x, left_eye.y, pupil_radius, BLACK);

        // Right eye
        let right_eye = to_world(Vec2::new(eye_forward, eye_lateral));
        draw_circle(right_eye.x, right_eye.y, eye_radius, WHITE);
        draw_circle(right_eye.x, right_eye.y, pupil_radius, BLACK);
    }
}
