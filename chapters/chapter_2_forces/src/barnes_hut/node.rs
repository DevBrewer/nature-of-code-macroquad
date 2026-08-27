#![allow(dead_code)]
use macroquad::{color::GRAY, shapes::draw_rectangle_lines};
use vec_math::{Vec2, quadtree::Rectangle};

#[derive(Debug)]
pub struct BarnesHutNode {
    pub boundary: Rectangle,
    // Total mass contained in this node.
    pub mass: f32,

    // Center of mass of everything contained in this node.
    pub center_of_mass: Vec2,

    // Bodies stored directly in this node.
    // Internal nodes normally have no direct bodies;
    // their mass is represented by the aggregate fields.
    pub body_indices: Vec<usize>,

    pub children: Option<Box<[BarnesHutNode; 4]>>, // NW,NE, SW, SE
    pub capacity: usize,
}

impl BarnesHutNode {
    pub fn new(boundary: Rectangle, capacity: usize) -> Self {
        assert!(
            capacity > 0,
            "BarnesHutNode capacity must be greater than zero"
        );

        Self {
            boundary,
            mass: 0.0,
            center_of_mass: Vec2::ZERO,
            body_indices: Vec::with_capacity(capacity),
            children: None,
            capacity,
        }
    }
    // old mass       M
    // old center     C
    // new body       m
    // new position   x
    // Barnes-hut: Core operation, it recalulates the center of mass
    // using the weighted average formula
    // The new center is:
    // C' = (MC + mx)/(M + m)
    pub fn add_mass(&mut self, mass: f32, position: Vec2) {
        let new_mass = self.mass + mass;

        if new_mass <= f32::EPSILON {
            return;
        }
        self.center_of_mass = (self.center_of_mass * self.mass + position * mass) / new_mass;

        self.mass = new_mass;
    }

    pub fn size(&self) -> f32 {
        self.boundary.width * 2.0
    }

    pub fn is_full(&self) -> bool {
        self.body_indices.len() >= self.capacity
    }

    pub fn add_body(&mut self, index: usize) {
        self.body_indices.push(index);
    }

    pub fn subdivide(&mut self) {
        let half_width = self.boundary.width * 0.5;
        let half_height = self.boundary.height * 0.5;

        let x = self.boundary.pos.x;
        let y = self.boundary.pos.y;

        let nw = BarnesHutNode::new(
            Rectangle::new(x - half_width, y - half_height, half_width, half_height),
            self.capacity,
        );

        let ne = BarnesHutNode::new(
            Rectangle::new(x + half_width, y - half_height, half_width, half_height),
            self.capacity,
        );

        let sw = BarnesHutNode::new(
            Rectangle::new(x - half_width, y + half_height, half_width, half_height),
            self.capacity,
        );

        let se = BarnesHutNode::new(
            Rectangle::new(x + half_width, y + half_height, half_width, half_height),
            self.capacity,
        );

        self.children = Some(Box::new([nw, ne, sw, se]));
    }

    pub fn draw(&self) {
        draw_rectangle_lines(
            self.boundary.left(),
            self.boundary.top(),
            self.boundary.width * 2.0,
            self.boundary.height * 2.0,
            1.0,
            GRAY,
        );

        if let Some(children) = self.children.as_ref() {
            for child in children.iter() {
                child.draw();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn center_of_mass_of_equal_masses() {
        let boundary = Rectangle::new(200.0, 200.0, 200.0, 200.0);
        let mut node = BarnesHutNode::new(boundary, 4);

        node.add_mass(2.0, Vec2::new(100.0, 100.0));
        node.add_mass(2.0, Vec2::new(200.0, 100.0));

        assert!((node.mass - 4.0).abs() < f32::EPSILON);
        assert!((node.center_of_mass.x - 150.0).abs() < f32::EPSILON);
        assert!((node.center_of_mass.y - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn center_of_mass_respects_mass() {
        let boundary = Rectangle::new(200.0, 200.0, 200.0, 200.0);
        let mut node = BarnesHutNode::new(boundary, 4);

        node.add_mass(1.0, Vec2::new(0.0, 0.0));
        node.add_mass(3.0, Vec2::new(100.0, 0.0));

        // node.mass = 0.0
        assert!((node.mass - 4.0).abs() < f32::EPSILON);

        // (1 * 0 + 3 * 100) / (1+3) = 75
        assert!((node.center_of_mass.x - 75.0).abs() < f32::EPSILON);

        // (1*0 + 3 * 0)/(1+3) = 0;
        assert!((node.center_of_mass.y - 0.0).abs() < f32::EPSILON);

        assert_eq!(node.mass, 4.0);
    }
}
