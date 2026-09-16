use macroquad::window::{Conf, next_frame};
use runner::{App, ExampleEntry, window_conf};

use crate::{
    examples::{
        angle_motion::AngleRotation, angular_motion_forces::AngularMotionForces,
        direction_motion::DirectionMotion, oscillation::OscillationExample,
        oscillator_objects::OscillatorObjectExample, polar_to_cartesian::PolarToCartesian,
        shm::ShmExample, shm2::Shm2Example, simple_pendulum::SimplePendulumExample,
        spring_forces::SpringForceExample, static_wave::StaticWaveExample,
    },
    exercises::{
        accelerating_oscillator::AcceleratingOscillatorExample,
        additive_waves::AdditiveWavesExample, angular_motion::AngularMotion,
        cannon::CannonSimulation, multiple_bobs::MultipleBobExercise,
        radial_petals::RadialPetalExample, spring_bob::SpringForces,
        vehicle_simulation::VehicleSimulation, double_pendulum::DoublePendulumExercise,
    },
};

mod angular_mover;
mod attractor;
mod body;
mod cannon_ball;
mod examples;
mod exercises;
mod oscillator;
mod spring;
const WIDTH: i32 = 600;
const HEIGHT: i32 = 600;

fn conf() -> Conf {
    window_conf("Nature of code - Chapter 3", WIDTH, HEIGHT)
}

#[macroquad::main(conf)]
async fn main() {
    let examples = vec![
        ExampleEntry {
            chapter: 3,
            number: "Example 3.1",
            title: "Angular Motion Using rotate()",
            example: Box::new(AngleRotation::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "Exercise 3.2",
            title: "Interactive Baton Drag & Damping",
            example: Box::new(AngularMotion::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "Example 3.2",
            title: "Forces with (Arbitrary) Angular Motion",
            example: Box::new(AngularMotionForces::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "Exercise 3.3",
            title: "Cannonball Simulation with Spin",
            example: Box::new(CannonSimulation::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "Example 3.3",
            title: "Pointing in the Direction of Motion",
            example: Box::new(DirectionMotion::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "Exercise 3.4",
            title: "Vehicle Steering Simulation",
            example: Box::new(VehicleSimulation::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "Example 3.4",
            title: "Polar to Cartesian Coordinates",
            example: Box::new(PolarToCartesian::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "Example 3.4b",
            title: "Polar Oscillation",
            example: Box::new(OscillationExample::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "Example 3.5",
            title: "Simple Harmonic Motion I",
            example: Box::new(ShmExample::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "Example 3.6",
            title: "Simple Harmonic Motion II",
            example: Box::new(Shm2Example::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "Exercise 3.7",
            title: "Spring Bob Simulation using sin() and map()",
            example: Box::new(SpringForces::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "Example 3.7",
            title: "Oscillator Objects",
            example: Box::new(OscillatorObjectExample::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "Exercise 3.8",
            title: "Radial Petals / Oscillator Pattern",
            example: Box::new(RadialPetalExample::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "Example 3.8",
            title: "Static Wave",
            example: Box::new(StaticWaveExample::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "Exercise 3.9",
            title: "Accelerating Oscillator (Insect Legs)",
            example: Box::new(AcceleratingOscillatorExample::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "Exercise 3.12",
            title: "Additive Waves",
            example: Box::new(AdditiveWavesExample::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "Example 3.10",
            title: "A Spring Connection (Hooke's Law)",
            example: Box::new(SpringForceExample::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "Exercise 3.14",
            title: "Multiple Bobs & Spring Connections",
            example: Box::new(MultipleBobExercise::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "Example 3.11",
            title: "Swinging Pendulum",
            example: Box::new(SimplePendulumExample::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "Exercise 3.16",
            title: "Double Pendulum Simulation",
            example: Box::new(DoublePendulumExercise::new()),
        },
    ];

    let mut app = App::from_screen(examples);

    loop {
        app.update();
        next_frame().await;
    }
}
