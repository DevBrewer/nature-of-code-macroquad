use macroquad::{
    color::{Color, WHITE},
    shapes::draw_rectangle,
};

use rand::RngExt;
use runner::{
    Example,
    camera::{world_height, world_width},
    render::draw_info_panel,
};

const BIN_COUNT: usize = 30;
const MIN_VALUE: f32 = -3.0;
const MAX_VALUE: f32 = 3.0;

pub struct GaussianDistribution {
    counts: Vec<u64>,
    total_samples: u64,
}

impl GaussianDistribution {
    pub fn new() -> Self {
        Self {
            counts: vec![0; BIN_COUNT],
            total_samples: 0,
        }
    }

    fn gaussian_sample() -> f32 {
        let mut rng = rand::rng();

        // Box-Muller transform:
        // convert two uniform values into a Gaussian value.
        let u1: f32 = rng.random_range(f32::EPSILON..1.0);
        let u2: f32 = rng.random_range(0.0..1.0);

        (-2.0 * u1.ln()).sqrt() * (std::f32::consts::TAU * u2).cos()
    }

    fn bin_index(value: f32) -> Option<usize> {
        // Ignore samples outside our visible range.
        if !(MIN_VALUE..MAX_VALUE).contains(&value) {
            return None;
        }

        let normalized = (value - MIN_VALUE) / (MAX_VALUE - MIN_VALUE);

        let index = (normalized * BIN_COUNT as f32) as usize;

        Some(index.min(BIN_COUNT - 1))
    }

    fn max_count(&self) -> u64 {
        self.counts.iter().copied().max().unwrap_or(1)
    }
}

impl Example for GaussianDistribution {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        let value = Self::gaussian_sample();

        // Only count values inside our visible range.
        if let Some(index) = Self::bin_index(value) {
            self.counts[index] += 1;
        }

        self.total_samples += 1;
    }

    fn draw(&self) {
        let width = world_width();
        let height = world_height();

        let margin = 20.0;
        let chart_top = 80.0;
        let chart_height = height - chart_top - margin;
        let bin_width = (width - margin * 2.0) / BIN_COUNT as f32;

        let max_count = self.max_count() as f32;

        for (index, &count) in self.counts.iter().enumerate() {
            let bar_height = if max_count > 0.0 {
                (count as f32 / max_count) * chart_height
            } else {
                0.0
            };

            let x = margin + index as f32 * bin_width;
            let y = height - margin - bar_height;

            draw_rectangle(x + 1.0, y, bin_width - 2.0, bar_height, WHITE);
        }

        // The center of the Gaussian distribution.
        let mean_x = margin + (BIN_COUNT as f32 * 0.5) * bin_width;

        // Mean marker.
        macroquad::shapes::draw_line(
            mean_x,
            chart_top,
            mean_x,
            height - margin,
            1.0,
            Color::new(1.0, 0.2, 0.2, 0.8),
        );

        let samples = format!("Samples: {}", self.total_samples);

        draw_info_panel(
            10.0,
            50.0,
            190.0,
            &[
                ("GAUSSIAN DISTRIBUTION", WHITE),
                ("Mean: 0", Color::new(0.7, 0.7, 0.7, 1.0)),
                ("Std. deviation: 1", Color::new(0.7, 0.7, 0.7, 1.0)),
                (&samples, Color::new(0.7, 0.7, 0.7, 1.0)),
            ],
        );
    }
}
