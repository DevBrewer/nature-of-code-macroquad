use macroquad::window::{Conf, next_frame};
use runner::{App, ExampleEntry, window_conf};

use crate::examples::{
    angle_motion::AngleRotation, angular_motion::AngularMotion,
    angular_motion_forces::AngularMotionForces, cannon::CannonSimulation,
    direction_motion::DirectionMotion, vehicle_simulation::VehicleSimulation,
};

mod angular_mover;
mod attractor;
mod body;
mod cannon_ball;
mod examples;

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
            number: "3.3",
            title: "Point in the direction of Motion",
            example: Box::new(DirectionMotion::new()),
        },
        ExampleEntry {
            chapter: 3,
            number: "3.4",
            title: "Exercise 3.4: Vehicle Steering Simulation",
            example: Box::new(VehicleSimulation::new()),
        },
    ];

    let mut app = App::from_screen(examples);

    loop {
        app.update();
        next_frame().await;
    }
}
