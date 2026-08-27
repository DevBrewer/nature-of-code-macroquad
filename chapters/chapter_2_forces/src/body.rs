#![allow(dead_code)]
use std::sync::{LazyLock, RwLock};

use macroquad::{color::WHITE, shapes::draw_circle};
use vec_math::Vec2;

static GRAVITY: LazyLock<RwLock<f32>> = LazyLock::new(|| RwLock::new(1.0));

pub struct Body {
    pub position: Vec2,
    pub velocity: Vec2,
    pub accleration: Vec2,

    pub mass: f32,
    pub radius: f32,
}

impl Body {
    pub fn new(position: Vec2, mass: f32) -> Self {
        let mass = mass.max(f32::EPSILON);

        Self {
            position,
            velocity: Vec2::ZERO,
            accleration: Vec2::ZERO,
            mass,
            radius: mass * 4.0,
        }
    }

    // set Gravitional Constant
    pub fn set_gravity(value: f32) {
        let mut gravity = GRAVITY.write().expect("Gravity constant not set");
        *gravity = value;
    }

    // get Gravitional Constant
    pub fn get_gravity() -> f32 {
        *GRAVITY.read().expect("No gravity constant found.")
    }

    // Set velocity
    pub fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
    }

    pub fn get_velocity(&self) -> Vec2 {
        self.velocity
    }

    /// Apply a force to this body
    /// Newton's second law F=ma => a = f/m
    pub fn apply_force(&mut self, force: Vec2) {
        self.accleration += force / self.mass
    }

    // Euler integration
    pub fn update(&mut self) {
        self.velocity += self.accleration;
        self.position += self.velocity;

        // Force are accumulated during the frame.
        // Start the next frame with no accumulated force
        self.accleration = Vec2::ZERO;
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, WHITE);
    }
}

impl Body {
    /// Newton's law of universal gravitation:
    /// F = G * (m1 * m2) / r²
    // The force points from body_a toward body_b.
    pub fn gravitational_force(body_a: &Body, body_b: &Body) -> Vec2 {
        let mut direction = body_b.position - body_a.position;
        let distance = direction.mag().max(5.0);

        let g = Self::get_gravity();
        let strength = g * body_a.mass * body_b.mass / (distance * distance);

        direction = direction.normalized();
        direction * strength
    }

    pub fn gravitational_force_from_mass(body: &Body, mass: f32, position: Vec2) -> Vec2 {
        let mut direction = position - body.position;
        let distance = direction.mag().max(5.0);

        let g = Body::get_gravity();
        let strength = g * body.mass * mass / (distance * distance);

        direction = direction.normalized();
        direction * strength
    }
}
