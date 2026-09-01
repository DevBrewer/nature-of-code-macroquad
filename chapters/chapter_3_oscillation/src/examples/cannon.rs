use macroquad::{
    color::{DARKGRAY, GRAY, GREEN, WHITE},
    input::{KeyCode, MouseButton, is_key_pressed, is_mouse_button_pressed},
    shapes::{draw_circle, draw_line},
    time::get_frame_time,
};
use runner::{Example, draw_info_panel, draw_world_border, world_height};
use vec_math::Vec2;

use crate::cannon_ball::CannonBall;

pub struct CannonSimulation {
    ball: CannonBall,
    gravity: Vec2,
    cannon_origin: Vec2,
}

impl CannonSimulation {
    pub fn new() -> Self {
        let cannon_origin = Vec2::new(60.0, world_height() - 60.0);
        let gravity = Vec2::new(0.0, 450.0); // Constant downward gravity force

        let mut ball = CannonBall::new(cannon_origin);

        // Fire initial shot
        let launch_force = Vec2::new(350.0, -450.0);
        ball.shoot(launch_force);

        Self {
            ball,
            gravity,
            cannon_origin,
        }
    }
}

impl Example for CannonSimulation {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        let dt = get_frame_time();

        // Allow user to re-fire the cannon with Space or Mouse Click
        if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
            self.ball = CannonBall::new(self.cannon_origin);
            let launch_force = Vec2::new(350.0, -450.0);
            self.ball.shoot(launch_force);
        }

        // Apply constant downward gravity force every frame
        self.ball.apply_force(self.gravity);

        // Update linear and angular kinematics
        self.ball.update(dt);

        // Ground collision check
        let ground_y = world_height() - 30.0;
        if self.ball.position.y >= ground_y - self.ball.radius {
            self.ball.position.y = ground_y - self.ball.radius;
            // Stop motion on impact
            self.ball.velocity = Vec2::ZERO;
            self.ball.angular_velocity *= 0.9;
        }
    }

    fn draw(&self) {
        draw_world_border();

        let ground_y = world_height() - 30.0;

        // Draw Ground Line
        draw_line(0.0, ground_y, 600.0, ground_y, 3.0, DARKGRAY);

        // Draw Cannon Barrel
        let barrel_end = self.cannon_origin + Vec2::new(30.0, -30.0);
        draw_line(
            self.cannon_origin.x,
            self.cannon_origin.y,
            barrel_end.x,
            barrel_end.y,
            10.0,
            GRAY,
        );
        draw_circle(self.cannon_origin.x, self.cannon_origin.y, 16.0, GRAY);

        // Draw Spinning Cannonball
        self.ball.draw();

        // Info Panel
        let speed = self.ball.velocity.mag();
        let spin = self.ball.angular_velocity;

        let speed_str = format!("Linear Speed: {:.1} px/s", speed);
        let spin_str = format!("Angular Spin: {:.2} rad/s", spin);

        let lines = [
            ("EXERCISE 3.3: CANNONBALL", WHITE),
            ("Press SPACE / Click to Fire", GREEN),
            (&speed_str, GRAY),
            (&spin_str, GRAY),
        ];

        draw_info_panel(10.0, world_height() - 85.0, 260.0, &lines);
    }
}
