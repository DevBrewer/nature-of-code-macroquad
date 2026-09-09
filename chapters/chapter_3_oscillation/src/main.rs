use macroquad::window::{Conf, next_frame};
use runner::{App, ExampleEntry, window_conf};

use crate::examples::{
    accelerating_oscillator::AcceleratingOscillatorExample, additive_waves::AdditiveWavesExample,
    angle_motion::AngleRotation, angular_motion::AngularMotion,
    angular_motion_forces::AngularMotionForces, cannon::CannonSimulation,
    direction_motion::DirectionMotion, oscillation::OscillationExample,
    oscillator_objects::OscillatorObjectExample, polar_to_cartesian::PolarToCartesian,
    radial_petals::RadialPetalExample, shm::ShmExample, shm2::Shm2Example,
    spring_bob::SpringForces, static_wave::StaticWaveExample,
    vehicle_simulation::VehicleSimulation,
};

mod angular_mover;
mod attractor;
mod body;
mod cannon_ball;
mod examples;
mod oscillator;
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
            number: "3.1",
            title: "Angle Motion",
            example: Box::new(AngleRotation::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "3.2.1",
            title: "Angular Motion",
            example: Box::new(AngularMotion::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "3.2.2",
            title: "Angular Motion Force(Arbitary)",
            example: Box::new(AngularMotionForces::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "3.2.3",
            title: "Cannonball Simulation with Spin",
            example: Box::new(CannonSimulation::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "3.3.1",
            title: "Point in the direction of Motion",
            example: Box::new(DirectionMotion::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "3.3.2",
            title: "Exercise 3.4: Vehicle Steering Simulation",
            example: Box::new(VehicleSimulation::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "3.4",
            title: "Polar to Cartesian Coordinates",
            example: Box::new(PolarToCartesian::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "3.4.1",
            title: "Polar Oscillation",
            example: Box::new(OscillationExample::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "3.5",
            title: "Simple Harmonic Motion",
            example: Box::new(ShmExample::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "3.6",
            title: "Simple Harmonic Motion II",
            example: Box::new(Shm2Example::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "3.7.0",
            title: "Exercise: Spring Forces using Map",
            example: Box::new(SpringForces::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "3.7.1",
            title: "Oscillator Ojbects",
            example: Box::new(OscillatorObjectExample::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "3.8",
            title: "Exercise Radial Petals",
            example: Box::new(RadialPetalExample::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "3.9",
            title: "Exercise 3.9: Accelerating Oscillator (Insect Legs)",
            example: Box::new(AcceleratingOscillatorExample::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "3.8",
            title: "Static Wave",
            example: Box::new(StaticWaveExample::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "3.12",
            title: "Exercise 3.12: Additive Waves",
            example: Box::new(AdditiveWavesExample::new()),
        },
    ];

    let mut app = App::from_screen(examples);

    loop {
        app.update();
        next_frame().await;
    }
}
