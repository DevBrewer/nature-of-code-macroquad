use macroquad::window::{Conf, next_frame};
use runner::{App, ExampleEntry};

use crate::examples::{
    attract_many::AttractionMany, attraction::AttractionExample, barnes_hut_n_body::BarnesHutNBody,
    fluid_resistance::FluidResistance, force::ForceExample, friction::FrictionExample,
    mass::MassExample, n_body_attraction::NBodyAttraction, two_body_attraction::TwoBodyAttraction,
};

mod attractor;
mod barnes_hut;
mod body;
mod examples;
mod liquid;
mod mover;

const HEIGHT: f32 = 600.0;
const WIDTH: f32 = 600.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Nature of Code - Chapter 2".into(),
        window_width: 600,
        window_height: 600,
        high_dpi: false,
        sample_count: 4,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let examples = vec![
        ExampleEntry {
            chapter: 2,
            number: "2.1",
            title: "Applying a Force",
            example: Box::new(ForceExample::new()),
        },
        ExampleEntry {
            chapter: 2,
            number: "2.2",
            title: "Mass and Acceleration",
            example: Box::new(MassExample::new()),
        },
        ExampleEntry {
            chapter: 2,
            number: "2.3",
            title: "Including Friction",
            example: Box::new(FrictionExample::new()),
        },
        ExampleEntry {
            chapter: 2,
            number: "2.4",
            title: "Fluid Resistance",
            example: Box::new(FluidResistance::new()),
        },
        ExampleEntry {
            chapter: 2,
            number: "2.5",
            title: "Attraction (Gravitional Force)",
            example: Box::new(AttractionExample::new()),
        },
        ExampleEntry {
            chapter: 2,
            number: "2.6",
            title: "Attraction with Many Movers(Gravitional Force)",
            example: Box::new(AttractionMany::new()),
        },
        ExampleEntry {
            chapter: 2,
            number: "2.7",
            title: "Two-Body Attraction",
            example: Box::new(TwoBodyAttraction::new()),
        },
        ExampleEntry {
            chapter: 2,
            number: "2.9",
            title: "N-Body Attraction",
            example: Box::new(NBodyAttraction::new()),
        },
        ExampleEntry {
            chapter: 2,
            number: "2.10",
            title: "N-Body Attraction Barnes-hut",
            example: Box::new(BarnesHutNBody::new()),
        },
    ];

    let mut app = App::new(examples, WIDTH, HEIGHT);

    loop {
        app.update();
        next_frame().await;
    }
}
