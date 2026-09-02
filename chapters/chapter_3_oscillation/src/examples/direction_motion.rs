use macroquad::{
    color::{GREEN, RED, WHITE, YELLOW},
    shapes::{draw_circle, draw_line},
    time::get_frame_time,
};
use runner::{
    Example, draw_info_panel, draw_vector, draw_world_border, mouse_world_position, world_center,
    world_height,
};
use vec_math::Vec2;

/// A mover object that accelerates towards the mouse and rotates to point
/// in the direction of its motion vector (velocity heading).
pub struct Mover {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub top_speed: f32,
    pub acceleration_strength: f32,
    pub angle: f32,
}

impl Mover {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            top_speed: 300.0,             // Top speed in pixels per second
            acceleration_strength: 500.0, // Acceleration strength (px/s^2)
            angle: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Calculate direction vector pointing from mover to mouse position.
        let mouse = mouse_world_position();
        let direction = mouse - self.position;

        // Normalize direction vector so acceleration magnitude stays consistent regardless of distance.
        let dir_normalized = direction.normalized();

        // Acceleration points toward mouse scaled by strength.
        self.acceleration = dir_normalized * self.acceleration_strength;

        // Euler Integration: update velocity with acceleration over frame delta time.
        self.velocity += self.acceleration * dt;

        // Limit maximum velocity to prevent excessive speed.
        self.velocity.limit(self.top_speed);

        // Update position based on limited velocity.
        self.position += self.velocity * dt;

        // Calculate rotation angle pointing in direction of motion (velocity heading).
        // atan2(vy, vx) gives heading angle in radians.
        if self.velocity.mag_sq() > 0.001 {
            self.angle = self.velocity.heading();
        }

        // Reset acceleration after updating velocity (accumulated forces reset per frame).
        self.acceleration = Vec2::ZERO;
    }

    /// Renders the mover transformed by its current velocity heading angle.
    pub fn draw(&self) {
        // Draw target indicator at mouse position
        let mouse = mouse_world_position();
        draw_circle(mouse.x, mouse.y, 4.0, RED);

        // Define vehicle shape in local space (pointing right along +X axis when angle = 0):
        // Tip: front nose point (+18, 0)
        // Rear left corner: (-12, -8)
        // Rear right corner: (-12, 8)
        // Rear inner indent: (-6, 0)
        let local_tip = Vec2::new(18.0, 0.0);
        let local_back_left = Vec2::new(-12.0, -8.0);
        let local_back_right = Vec2::new(-12.0, 8.0);
        let local_indent = Vec2::new(-6.0, 0.0);

        // Transform local vertices to world space by rotating by `self.angle` and translating by `self.position`.
        let tip = self.position + local_tip.rotate(self.angle);
        let back_left = self.position + local_back_left.rotate(self.angle);
        let back_right = self.position + local_back_right.rotate(self.angle);
        let indent = self.position + local_indent.rotate(self.angle);

        // Draw vehicle body outline
        draw_line(tip.x, tip.y, back_left.x, back_left.y, 2.0, WHITE);
        draw_line(back_left.x, back_left.y, indent.x, indent.y, 2.0, WHITE);
        draw_line(indent.x, indent.y, back_right.x, back_right.y, 2.0, WHITE);
        draw_line(back_right.x, back_right.y, tip.x, tip.y, 2.0, WHITE);

        // Render velocity vector arrow in direction of motion
        draw_vector(self.position, self.velocity * 0.25, GREEN);
    }
}

pub struct DirectionMotion {
    mover: Mover,
}

impl DirectionMotion {
    pub fn new() -> Self {
        let center = world_center();
        Self {
            mover: Mover::new(Vec2::new(center.x, center.y)),
        }
    }
}

impl Example for DirectionMotion {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        let dt = get_frame_time();
        self.mover.update(dt);
    }

    fn draw(&self) {
        draw_world_border();

        self.mover.draw();

        let speed = self.mover.velocity.mag();

        let speed_str = format!(
            "Speed: {:.1} px/s (Max: {:.0})",
            speed, self.mover.top_speed
        );
        let heading_str = format!("Heading (angle): ({:.1}°)", self.mover.angle);

        let lines = [
            ("EXAMPLE 3.3: POINTING IN DIRECTION OF MOTION", WHITE),
            (&speed_str, YELLOW),
            (&heading_str, YELLOW),
        ];

        draw_info_panel(10.0, world_height() - 80.0, 340.0, &lines);
    }
}
