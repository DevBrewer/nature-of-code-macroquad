#![allow(dead_code)]
use vec_math::{Vec2, quadtree::Rectangle};

use crate::{barnes_hut::BarnesHutNode, body::Body};

#[derive(Debug)]
pub struct BarnesHutTree {
    pub root: BarnesHutNode,
}

impl BarnesHutTree {
    pub fn new(boundary: Rectangle, capacity: usize) -> Self {
        Self {
            root: BarnesHutNode::new(boundary, capacity),
        }
    }

    pub fn insert(&mut self, index: usize, bodies: &[Body]) -> bool {
        Self::insert_into_node(&mut self.root, index, bodies)
    }

    fn insert_into_node(node: &mut BarnesHutNode, index: usize, bodies: &[Body]) -> bool {
        let body = &bodies[index];

        // Body doesn't belong to this region.
        if !node.boundary.contains(body.position) {
            return false;
        }

        // Update aggregate information for this node.
        node.add_mass(body.mass, body.position);

        //--------------------------------
        // Internal node:
        // this node already has children, so the body Must go
        // into children.
        // ----------------------------
        if let Some(children) = node.children.as_mut() {
            for child in children.as_mut() {
                if Self::insert_into_node(child, index, bodies) {
                    return true;
                }
            }
            // The body belonged to this node's boundary, so this
            // should never happen if subdivision is correct.
            return false;
        }

        //-----------------------------------------
        // Leaf node with available capacity
        //-----------------------------------------
        if !node.is_full() {
            node.add_body(index);
            return true;
        }

        // The node is full, so create its four quardants.
        if node.children.is_none() {
            node.subdivide();
        }

        // Move existing bodies out of the parent.
        let old_indices = std::mem::take(&mut node.body_indices);

        // Redistribute existing bodies
        if let Some(children) = node.children.as_mut() {
            for old_index in old_indices {
                for child in children.iter_mut() {
                    if Self::insert_into_node(child, old_index, bodies) {
                        break;
                    }
                }
            }
        }

        // Insert the newly arriving body.
        if let Some(children) = node.children.as_mut() {
            for child in children.iter_mut() {
                if Self::insert_into_node(child, index, bodies) {
                    return true;
                }
            }
        }
        false
    }

    pub fn calculate_force(&self, body_index: usize, bodies: &[Body], theta: f32) -> Vec2 {
        Self::calculate_force_from_node(&self.root, body_index, bodies, theta)
    }

    /*
    Algorithm
          Node
            │
      Is node empty?
       /          \
     YES           NO
      │             │
    zero       Is leaf?
                 /      \
               YES       NO
                │         │
          exact forces   target inside?
                          /       \
                        YES        NO
                         │          │
                      recurse     s/d < θ?
                                  /       \
                                YES        NO
                                 │          │
                             aggregate    recurse
    */
    fn calculate_force_from_node(
        node: &BarnesHutNode,
        body_index: usize,
        bodies: &[Body],
        theta: f32,
    ) -> Vec2 {
        let body = &bodies[body_index];

        // Case A
        // Nothing in this node.
        if node.mass <= f32::EPSILON {
            return Vec2::ZERO;
        }

        // Case B
        // Leaf node: calculate exact interactions
        // with the bodies stored here.
        if node.children.is_none() {
            let mut force = Vec2::ZERO;

            for &other_index in &node.body_indices {
                // A body must not attract itself.
                if other_index == body_index {
                    continue;
                }

                force += Body::gravitational_force(body, &bodies[other_index]);
            }

            return force;
        }

        // Case C - Internal node
        let direction = node.center_of_mass - body.position;
        let distance = direction.mag();

        if distance <= f32::EPSILON {
            return Vec2::ZERO;
        }

        let size = node.size();

        // If the entire region looks sufficiently small
        // from the target body's point of view, treat all
        // bodies inside it as one aggregate body.
        let contains_target = node.boundary.contains(body.position);

        if !contains_target && size / distance < theta {
            return Body::gravitational_force_from_mass(body, node.mass, node.center_of_mass);
        }

        // The region is too close/large to approximate.
        // Open the node and inspect its children[NW, NE, SW, SE].
        let mut force = Vec2::ZERO;

        if let Some(children) = node.children.as_ref() {
            for child in children.iter() {
                force += Self::calculate_force_from_node(child, body_index, bodies, theta);
            }
        }

        force
    }

    pub fn draw(&self) {
        self.root.draw();
    }
}

#[cfg(test)]
mod tests {
    use vec_math::{Vec2, quadtree::Rectangle};

    use super::*;

    fn make_body(x: f32, y: f32, mass: f32) -> Body {
        Body::new(Vec2::new(x, y), mass)
    }

    #[test]
    fn subdivision_redistribution_existing_bodies() {
        let boundary = Rectangle::new(200.0, 200.0, 200.0, 200.0);

        // Capacity = 1 means the second intersection
        // forces the root to subdivide.
        let mut tree = BarnesHutTree::new(boundary, 1);

        let bodies = vec![
            make_body(150.0, 150.0, 1.0), // NW
            make_body(250.0, 150.0, 1.0), // NE
        ];

        assert!(tree.insert(0, &bodies));
        assert!(tree.insert(1, &bodies));

        // The root is now an internal node.
        assert!(tree.root.children.is_some());

        // The bodies must no longer be stored directly
        // in the parent.
        assert!(tree.root.body_indices.is_empty());

        let children = tree.root.children.as_ref().unwrap();

        // NW child contains body 0
        assert_eq!(
            children[0].body_indices,
            vec![0],
            "Nort West index should be 0"
        );

        // NE child contains body 0
        assert_eq!(
            children[1].body_indices,
            vec![1],
            "Nort East index should be 1"
        );

        // SW and SE should be empty
        assert!(
            children[2].body_indices.is_empty(),
            "South West should have 0 index"
        );
        assert!(
            children[3].body_indices.is_empty(),
            "South East should have 0 index"
        );
    }

    #[test]
    fn subdivision_preserve_total_mass() {
        let boundary = Rectangle::new(200.0, 200.0, 100.0, 100.0);

        let mut tree = BarnesHutTree::new(boundary, 1);

        let bodies = vec![
            make_body(150.0, 150.0, 2.0), // NW
            make_body(250.0, 150.0, 3.0), // NE
            make_body(150.0, 250.0, 4.0), // SW
            make_body(250.0, 250.0, 5.0), // SE
        ];

        for index in 0..bodies.len() {
            assert!(tree.insert(index, &bodies))
        }

        // The root represents all four bodies.
        assert_eq!(tree.root.mass, 14.0);

        // The root itself should not directly contain them.
        assert!(tree.root.body_indices.is_empty());

        let children = tree.root.children.as_ref().unwrap();

        assert_eq!(children[0].mass, 2.0);
        assert_eq!(children[1].mass, 3.0);
        assert_eq!(children[2].mass, 4.0);
        assert_eq!(children[3].mass, 5.0);
    }

    #[test]
    fn subdivision_preserves_center_of_mass() {
        let boundary = Rectangle::new(200.0, 200.0, 100.0, 100.0);

        let mut tree = BarnesHutTree::new(boundary, 1);

        let bodies = vec![
            make_body(150.0, 150.0, 2.0), // NW
            make_body(250.0, 150.0, 3.0), // NE
            make_body(150.0, 250.0, 4.0), // SW
            make_body(250.0, 250.0, 5.0), // SE
        ];

        for index in 0..bodies.len() {
            assert!(tree.insert(index, &bodies));
        }

        let expected_x = (2.0 * 150.0 + 3.0 * 250.0 + 4.0 * 150.0 + 5.0 * 250.0) / 14.0;
        let expected_y = (2.0 * 150.0 + 3.0 * 150.0 + 4.0 * 250.0 + 5.0 * 250.0) / 14.0;

        let center = tree.root.center_of_mass;

        assert!((center.x - expected_x).abs() < f32::EPSILON);
        assert!((center.y - expected_y).abs() < f32::EPSILON);
    }

    #[test]
    fn redistribution_can_continue_recursivley() {
        let boundary = Rectangle::new(200.0, 200.0, 100.0, 100.0);

        let mut tree = BarnesHutTree::new(boundary, 1);

        let bodies = vec![
            make_body(150.0, 150.0, 1.0),
            make_body(120.0, 125.0, 1.0),
            make_body(170.0, 125.0, 1.0),
        ];

        for index in 0..bodies.len() {
            assert!(tree.insert(index, &bodies));
        }

        // Root must have subdivided.
        assert!(tree.root.children.is_some(), "should have been divided");

        let children = tree.root.children.as_ref().unwrap();

        // Body 0, 1, 2 are all in the NW quadrant of the root.
        let nw = &children[0];
        assert!(nw.children.is_some());

        // NW itself become an internal node.
        assert!(nw.body_indices.is_empty());
    }

    fn assert_vec2_close(actual: Vec2, expected: Vec2) {
        let epsilon = 0.0001;

        assert!(
            (actual.x - expected.x).abs() < epsilon,
            "x:actual={}, expected={}",
            actual.x,
            expected.x
        );

        assert!(
            (actual.y - expected.y).abs() < epsilon,
            "y:actual={}, expected{}",
            actual.y,
            expected.y
        );
    }
    #[test]
    fn leaf_force_matches_exact_gravity() {
        Body::set_gravity(1.0);

        let boundary = Rectangle::new(200.0, 200.0, 200.0, 200.0);
        let mut tree = BarnesHutTree::new(boundary, 4);

        let bodies = vec![make_body(100.0, 200.0, 2.0), make_body(300.0, 200.0, 3.0)];

        assert!(tree.insert(0, &bodies));
        assert!(tree.insert(1, &bodies));

        let barnes_hut_force = tree.calculate_force(0, &bodies, 0.5);
        let exact_force = Body::gravitational_force(&bodies[0], &bodies[1]);

        assert_vec2_close(barnes_hut_force, exact_force);
    }

    #[test]
    fn distance_cluster_uses_center_of_mass() {
        Body::set_gravity(1.0);

        let boundary = Rectangle::new(500.0, 200.0, 500.0, 200.0);
        let mut tree = BarnesHutTree::new(boundary, 1);

        let bodies = vec![
            make_body(100.0, 200.0, 1.0), // target
            make_body(400.0, 180.0, 2.0),
            make_body(420.0, 220.0, 3.0),
        ];

        for index in 0..bodies.len() {
            assert!(tree.insert(index, &bodies));
        }

        let theta = 0.5;
        let force = tree.calculate_force(0, &bodies, theta);

        // The cluster consists of:
        // body1: mass 2 at (400, 180)
        // body2: mass 3 at (420, 220)
        //
        // Center of mass:
        // x = (2*400 + 3*420)/(2+3) = 412
        // y = (2*180 + 3*220)/5 = 204
        let center_of_mass = Vec2::new(412.0, 204.0);

        let expected = Body::gravitational_force_from_mass(&bodies[0], 5.0, center_of_mass);

        assert_vec2_close(force, expected);
    }

    #[test]
    fn redistribution_keeps_all_bodies() {
        Body::set_gravity(1.0);

        let boundary = Rectangle::new(200.0, 200.0, 200.0, 200.0);

        let mut tree = BarnesHutTree::new(boundary, 1);

        let bodies = vec![
            make_body(100.0, 200.0, 1.0),
            make_body(120.0, 200.0, 2.0),
            make_body(140.0, 200.0, 3.0),
        ];

        for index in 0..bodies.len() {
            assert!(tree.insert(index, &bodies));
        }

        let mut found = Vec::new();

        fn collect_indices(node: &BarnesHutNode, found: &mut Vec<usize>) {
            found.extend(&node.body_indices);

            if let Some(children) = node.children.as_ref() {
                for child in children.iter() {
                    collect_indices(child, found);
                }
            }
        }

        collect_indices(&tree.root, &mut found);

        found.sort_unstable();

        assert_eq!(found, vec![0, 1, 2]);
    }

    #[test]
    fn node_containing_target_is_not_approximated() {
        Body::set_gravity(1.0);

        let boundary = Rectangle::new(200.0, 200.0, 200.0, 200.0);
        let mut tree = BarnesHutTree::new(boundary, 1);

        let bodies = vec![
            make_body(100.0, 100.0, 1.0),
            make_body(120.0, 120.0, 2.0),
            make_body(140.0, 140.0, 3.0),
        ];

        for index in 0..bodies.len() {
            assert!(tree.insert(index, &bodies), "failed to insert body {index}");
        }

        let theta = 10.0;

        let barnes_hut_force = tree.calculate_force(0, &bodies, theta);

        let force_from_body_1 = Body::gravitational_force(&bodies[0], &bodies[1]);

        let force_from_body_2 = Body::gravitational_force(&bodies[0], &bodies[2]);

        let expected = force_from_body_1 + force_from_body_2;

        println!("BH force:     {:?}", barnes_hut_force);
        println!("Exact force:  {:?}", expected);
        println!("Body 1 force: {:?}", force_from_body_1);
        println!("Body 2 force: {:?}", force_from_body_2);

        assert_vec2_close(barnes_hut_force, expected);
    }
}
