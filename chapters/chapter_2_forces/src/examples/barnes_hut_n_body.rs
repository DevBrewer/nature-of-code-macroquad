use macroquad::{
    color::{GRAY, WHITE},
    rand::gen_range,
};
use runner::{
    Example, draw_info_panel, draw_world_border, world_center, world_height, world_width,
};
use vec_math::{Vec2, quadtree::Rectangle};

use crate::{barnes_hut::BarnesHutTree, body::Body};

pub struct BarnesHutNBody {
    bodies: Vec<Body>,
    tree: BarnesHutTree,
    galaxy_center: Vec2,
    central_mass: f32,
}

impl BarnesHutNBody {
    pub fn new() -> Self {
        let count = 500;
        let center = world_center();
        let central_mass = 200.0;

        let boundary = Rectangle::new(
            center.x,
            center.y,
            world_width() * 0.5,
            world_height() * 0.5,
        );

        let mut bodies = Vec::with_capacity(count);

        for _ in 0..count {
            // Keep particles in a disk around the center.
            let radius = gen_range(50.0, 180.0);
            let angle = gen_range(0.0, std::f32::consts::TAU);

            let position = Vec2::new(
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin(),
            );

            let mass = 1.0;
            let mut body = Body::new(position, mass);

            // Direction from galaxy center -> particle.
            let radial = position - center;
            let radial_direction = radial.normalized();

            // Tangential direction = radial direction around 90 deg.
            let tangent = radial_direction.rotate(std::f32::consts::FRAC_2_PI);

            // Approximate circular orbital velocity:
            //
            // v = sqrt(GM / r)
            Body::set_gravity(0.06);
            let g = Body::get_gravity();
            let speed = (g * central_mass / radius).sqrt();

            body.set_velocity(tangent * speed);
            bodies.push(body);
        }
        let tree = BarnesHutTree::new(boundary, 1);

        Self {
            bodies,
            tree,
            galaxy_center: center,
            central_mass,
        }
    }

    fn rebuild_tree(&mut self) {
        let boundary = Rectangle::new(
            world_width() * 0.5,
            world_height() * 0.5,
            world_width() * 0.5,
            world_height() * 0.5,
        );
        self.tree = BarnesHutTree::new(boundary, 1);

        for index in 0..self.bodies.len() {
            self.tree.insert(index, &self.bodies);
        }
    }
}

impl Example for BarnesHutNBody {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        // Build tree from current body positions.
        self.rebuild_tree();

        let theta = 0.5;

        let mut forces = vec![Vec2::ZERO; self.bodies.len()];

        for (index, body) in self.bodies.iter().enumerate() {
            forces[index] = self.tree.calculate_force(index, &self.bodies, theta);

            // Central gravitional force.
            let central_force =
                Body::gravitational_force_from_mass(body, self.central_mass, self.galaxy_center);
            forces[index] += central_force;
        }

        // Apply forces only after all forces  have
        // been calculated from the same tree.
        self.bodies
            .iter_mut()
            .zip(forces)
            .for_each(|(body, force)| {
                body.apply_force(force);
                body.update();
            });
    }

    fn draw(&self) {
        draw_world_border();

        // Draw Barnes-hut spatial decomposition.
        self.tree.draw();

        // Draw bodies
        self.bodies.iter().for_each(|body| body.draw());

        let lines = [
            ("BARNES-HUT N-BODY", WHITE),
            ("150 bodies", WHITE),
            ("Far clusters -> one body", GRAY),
            ("Near bodies -> exact", GRAY),
            ("Tree: ON", GRAY),
            ("Theta: 0.5", GRAY),
        ];

        draw_info_panel(10.0, world_height() - 100.0, 230.0, &lines);
    }
}
