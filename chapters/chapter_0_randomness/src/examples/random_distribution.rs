use macroquad::{
    color::{Color, WHITE},
    shapes::draw_rectangle,
    window::{screen_height, screen_width},
};
use rand::RngExt;
use runner::{Example, draw_info_panel};

const BIN_COUNT: usize = 20;

pub struct RandomDistribution {
    counts: Vec<u32>,
    total_samples: u64,
}

impl RandomDistribution {
    pub fn new() -> Self {
        Self {
            counts: vec![0; BIN_COUNT],
            total_samples: 0,
        }
    }

    fn max(&self) -> u32 {
        self.counts.iter().copied().max().unwrap_or(1)
    }
}

impl Example for RandomDistribution {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        let mut rng = rand::rng();
        let index = rng.random_range(0..BIN_COUNT);
        self.counts[index] += 1;
        self.total_samples += 1;
    }

    fn draw(&self) {
        let width = screen_width();
        let height = screen_height();

        let margin = 20.0;
        let chart_height = height - 100.0;
        let bin_width = (width - margin * 2.0) / BIN_COUNT as f32;

        let max_count = self.max() as f32;

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

        let samples = format!("Samples: {}", self.total_samples);

        draw_info_panel(
            0.5,
            height - 40.0,
            150.0,
            &[
                ("RANDOM DISTRIBUTION", WHITE),
                (&samples, Color::new(0.7, 0.7, 0.7, 1.0)),
            ],
        );
    }
}
