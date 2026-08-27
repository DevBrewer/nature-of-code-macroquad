use macroquad::window::{Conf, next_frame};
use runner::{App, ExampleEntry};

use crate::examples::{
    accept_reject::AcceptRejectDistribution, gaussian_distribution::GaussianDistribution,
    perlin_noise::PerlinNoiseExample, perlin_noise_2d::PerlinNoise2D,
    perlin_noise_walker::PerlinNoiseWalker, random_distribution::RandomDistribution,
    random_walk::TraditionalRandomWalk, rightward_walk::RightwardWalk,
};

mod examples;
mod walker;
mod walker_right;

const WIDTH: f32 = 400.0;
const HEIGHT: f32 = 400.0;
fn window_conf() -> Conf {
    Conf {
        window_title: "Nature of Code - Chapter 0".to_string(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        high_dpi: false,
        sample_count: 4,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let examples: Vec<ExampleEntry> = vec![
        ExampleEntry {
            chapter: 0,
            number: "0.1",
            title: "Traditional Random Walk",
            example: Box::new(TraditionalRandomWalk::new()),
        },
        ExampleEntry {
            chapter: 0,
            number: "0.2",
            title: "Random-Number Distribution",
            example: Box::new(RandomDistribution::new()),
        },
        ExampleEntry {
            chapter: 0,
            number: "0.3",
            title: "Walker That Tends to Move Right",
            example: Box::new(RightwardWalk::new()),
        },
        ExampleEntry {
            chapter: 0,
            number: "0.4",
            title: "Gaussian Distribution",
            example: Box::new(GaussianDistribution::new()),
        },
        ExampleEntry {
            chapter: 0,
            number: "0.5",
            title: " Accept-Reject Distribution",
            example: Box::new(AcceptRejectDistribution::new()),
        },
        ExampleEntry {
            chapter: 0,
            number: "0.6",
            title: "1D Perlin Noise",
            example: Box::new(PerlinNoiseExample::new()),
        },
        ExampleEntry {
            chapter: 0,
            number: "0.7",
            title: "Perlin Noise Walker",
            example: Box::new(PerlinNoiseWalker::new()),
        },
        ExampleEntry {
            chapter: 0,
            number: "0.8",
            title: "2D Perlin Noise",
            example: Box::new(PerlinNoise2D::new()),
        },
    ];

    let mut app = App::new(examples, WIDTH, HEIGHT);
    loop {
        app.update();

        next_frame().await
    }
}
