mod examples;

use macroquad::window::{Conf, next_frame};
use runner::{App, ExampleEntry, window_conf};

use crate::examples::{
    bouncing_ball::BouncingBall, bouncing_ball_vectors::BouncingBallVectors,
    motion_101_accleration::Motion101Acceleraton, motion_101_velocity::Motion101Velocity,
    motion101_mouse_acceleration::Motion101MouseAcceleration,
    motion101_random_acceleration::Motion101RandomAcceleration, random_vector::RandomVector,
    vector_magnitude::VectorMagnitude, vector_multiplication::VectorMultiplication,
    vector_normalization::VectorNormalization, vector_subtraction::VectorSubtraction,
};

const WIDTH: i32 = 600;
const HEIGHT: i32 = 600;

fn config() -> Conf {
    window_conf("Nature of Code - Chapter 1", WIDTH, HEIGHT)
}

#[macroquad::main(config)]
async fn main() {
    let examples = vec![
        ExampleEntry {
            chapter: 1,
            number: "1.1",
            title: "Bouncing Ball",
            example: Box::new(BouncingBall::new()),
        },
        ExampleEntry {
            chapter: 1,
            number: "1.2",
            title: "Bouncing Ball with Vectors",
            example: Box::new(BouncingBallVectors::new()),
        },
        ExampleEntry {
            chapter: 1,
            number: "1.3",
            title: "Vector Subtraction",
            example: Box::new(VectorSubtraction::new()),
        },
        ExampleEntry {
            chapter: 1,
            number: "1.4",
            title: "Vector Multiplication",
            example: Box::new(VectorMultiplication::new()),
        },
        ExampleEntry {
            chapter: 1,
            number: "1.5",
            title: "Vector Magnitude",
            example: Box::new(VectorMagnitude::new()),
        },
        ExampleEntry {
            chapter: 1,
            number: "1.6",
            title: "Vector Normalization",
            example: Box::new(VectorNormalization::new()),
        },
        ExampleEntry {
            chapter: 1,
            number: "1.7",
            title: "Motion 101 (Velocity)",
            example: Box::new(Motion101Velocity::new()),
        },
        ExampleEntry {
            chapter: 1,
            number: "1.8",
            title: "Motion 101 (Velocity and Acceleration)",
            example: Box::new(Motion101Acceleraton::new()),
        },
        ExampleEntry {
            chapter: 1,
            number: "1.9",
            title: "Motion 101 (Velocity and Random Acceleration)",
            example: Box::new(Motion101RandomAcceleration::new()),
        },
        ExampleEntry {
            chapter: 1,
            number: "1.10",
            title: "Motion 101 (Velocity and Acceleration Towards the Mouse)",
            example: Box::new(Motion101MouseAcceleration::new()),
        },
        ExampleEntry {
            chapter: 1,
            number: "1.11",
            title: "Random Vector",
            example: Box::new(RandomVector::new()),
        },
    ];

    let mut app = App::from_screen(examples);

    loop {
        app.update();

        next_frame().await;
    }
}
