use std::collections::VecDeque;

use macroquad::{
    color::{Color, GREEN, RED, WHITE},
    shapes::{draw_circle, draw_line},
};

use rand::RngExt;
use runner::{
    Example,
    camera::{world_height, world_width},
    render::draw_info_panel,
};

const MAX_SAMPLES: usize = 500;

/// Demonstrates accept-reject sampling.
///
/// Use case:
/// Sometimes we know the probability distribution we want,
/// but we don't have a random function that can generate it
/// directly.
///
/// Accept-reject sampling lets us build that distribution:
///
/// 1. Generate a random x.
/// 2. Generate a random y.
/// 3. Evaluate the desired probability at x.
/// 4. Accept the point when y is below the curve.
/// 5. Reject it when y is above the curve.
///
/// After many samples, the accepted x values follow the
/// shape of the probability curve.
pub struct AcceptRejectDistribution {
    accepted: VecDeque<(f32, f32)>,
    rejected: VecDeque<(f32, f32)>,
    rng: rand::rngs::ThreadRng,
}

impl AcceptRejectDistribution {
    pub fn new() -> Self {
        Self {
            accepted: VecDeque::new(),
            rejected: VecDeque::new(),
            rng: rand::rng(),
        }
    }

    /// The probability curve we want our samples to follow.
    ///
    /// This is a simple bell-like curve:
    /// - high probability near the center
    /// - low probability near the edges
    fn probability(x: f32) -> f32 {
        1.0 - x * x
    }

    /// Generate one candidate point and decide whether to keep it.
    fn sample(&mut self) {
        // Generate a random point inside the sampling rectangle.
        let x = self.rng.random_range(-1.0..1.0);
        let y = self.rng.random_range(0.0..1.0);

        // The curve tells us how likely this x should be.
        let probability = Self::probability(x);

        if y <= probability {
            // Below the curve → accept.
            self.accepted.push_back((x, y));
        } else {
            // Above the curve → reject.
            self.rejected.push_back((x, y));
        }

        self.trim_samples();
    }

    // Keep the visualization from growing forever.
    fn trim_samples(&mut self) {
        while self.accepted.len() + self.rejected.len() > MAX_SAMPLES {
            // Remove the oldest sample from whichever queue
            // currently contains the oldest entry.
            // We do not track global ordering here, so we simply
            // remove from the larger queue.
            if self.accepted.len() >= self.rejected.len() {
                self.accepted.pop_front();
            } else {
                self.rejected.pop_front();
            }
        }
    }

    /// Convert mathematical coordinates into screen coordinates.
    fn screen_position(x: f32, y: f32) -> (f32, f32) {
        let width = world_width();
        let height = world_height();
        let margin = 40.0;

        let screen_x = margin + ((x + 1.0) * 0.5) * (width - margin * 2.0);
        let screen_y = height - margin - y * (height - margin * 2.0);

        (screen_x, screen_y)
    }
}

impl Example for AcceptRejectDistribution {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        // Add one candidate point per frame.
        self.sample();
    }

    fn draw(&self) {
        let width = world_width();
        let height = world_height();

        let margin = 40.0;

        // -------------------------
        // Draw probability curve
        // -------------------------

        let mut previous: Option<(f32, f32)> = None;

        for i in 0..=100 {
            let x = -1.0 + (i as f32 / 100.0) * 2.0;
            let y = Self::probability(x);

            let current = Self::screen_position(x, y);

            if let Some(previous) = previous {
                draw_line(previous.0, previous.1, current.0, current.1, 2.0, WHITE);
            }

            previous = Some(current);
        }

        // -------------------------
        // Draw accepted samples
        // -------------------------

        for &(x, y) in &self.accepted {
            let (screen_x, screen_y) = Self::screen_position(x, y);

            draw_circle(screen_x, screen_y, 2.0, GREEN);
        }

        // -------------------------
        // Draw rejected samples
        // -------------------------

        for &(x, y) in &self.rejected {
            let (screen_x, screen_y) = Self::screen_position(x, y);

            draw_circle(screen_x, screen_y, 2.0, RED);
        }

        // -------------------------
        // Draw sampling boundary
        // -------------------------

        draw_line(
            margin,
            height - margin,
            width - margin,
            height - margin,
            1.0,
            Color::new(0.4, 0.4, 0.4, 1.0),
        );

        // -------------------------
        // Information
        // -------------------------

        let accepted = self.accepted.len();
        let rejected = self.rejected.len();

        let accepted_text = format!("Accepted: {}", accepted);

        let rejected_text = format!("Rejected: {}", rejected);

        draw_info_panel(
            10.0,
            50.0,
            210.0,
            &[
                ("ACCEPT-REJECT SAMPLING", WHITE),
                ("Green = accepted", GREEN),
                ("Red = rejected", RED),
                (&accepted_text, Color::new(0.7, 0.7, 0.7, 1.0)),
                (&rejected_text, Color::new(0.7, 0.7, 0.7, 1.0)),
            ],
        );
    }
}
