use crate::Vec2;

const MIN_CELL_SIZE: f32 = 0.0001;

// ------------------------------------------------------------
// Point
// ------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub pos: Vec2,
}

impl Point {
    pub fn new(pos: Vec2) -> Self {
        Self { pos }
    }
}

// ------------------------------------------------------------
// Rectangle
//
// pos    = center
// width  = half-width
// height = half-height
// ------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub pos: Vec2,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        assert!(
            width >= 0.0 && height >= 0.0,
            "Rectangle dimensions must be non-negative"
        );

        Self {
            pos: Vec2::new(x, y),
            width,
            height,
        }
    }

    pub fn left(&self) -> f32 {
        self.pos.x - self.width
    }

    pub fn right(&self) -> f32 {
        self.pos.x + self.width
    }

    pub fn top(&self) -> f32 {
        self.pos.y - self.height
    }

    pub fn bottom(&self) -> f32 {
        self.pos.y + self.height
    }

    pub fn contains(&self, pos: Vec2) -> bool {
        pos.x >= self.left()
            && pos.x <= self.right()
            && pos.y >= self.top()
            && pos.y <= self.bottom()
    }

    pub fn intersects(&self, other: &Rectangle) -> bool {
        let separated_x = other.left() > self.right() || other.right() < self.left();
        let separated_y = other.top() > self.bottom() || other.bottom() < self.top();

        !(separated_x || separated_y)
    }

    pub fn can_subdivide(&self) -> bool {
        self.width > MIN_CELL_SIZE && self.height > MIN_CELL_SIZE
    }
}

// ------------------------------------------------------------
// Circle
// ------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub pos: Vec2,
    pub radius: f32,
}

impl Circle {
    pub fn new(x: f32, y: f32, radius: f32) -> Self {
        assert!(radius >= 0.0, "Circle radius must be non-negative");

        Self {
            pos: Vec2::new(x, y),
            radius,
        }
    }

    pub fn contains(&self, point: &Point) -> bool {
        point.pos.distance_squared(self.pos) <= self.radius * self.radius
    }

    pub fn intersects(&self, rectangle: &Rectangle) -> bool {
        let closest_x = self.pos.x.clamp(rectangle.left(), rectangle.right());

        let closest_y = self.pos.y.clamp(rectangle.top(), rectangle.bottom());

        let dx = self.pos.x - closest_x;
        let dy = self.pos.y - closest_y;

        let distance_squared = dx * dx + dy * dy;

        distance_squared <= self.radius * self.radius
    }
}

// ------------------------------------------------------------
// QuadTree
// ------------------------------------------------------------

pub struct QuadTree {
    boundary: Rectangle,
    capacity: usize,

    // Points are stored only in leaf nodes.
    points: Vec<Point>,

    // None  -> leaf
    // Some -> internal node
    children: Option<Box<[QuadTree; 4]>>,
}

impl QuadTree {
    pub fn new(boundary: Rectangle, capacity: usize) -> Self {
        assert!(capacity > 0, "QuadTree capacity must be greater than zero");

        Self {
            boundary,
            capacity,
            points: Vec::with_capacity(capacity),
            children: None,
        }
    }

    // --------------------------------------------------------
    // Insert
    // --------------------------------------------------------

    pub fn insert(&mut self, point: Point) -> bool {
        // Point doesn't belong to this node.
        if !self.boundary.contains(point.pos) {
            return false;
        }

        // ----------------------------------------------------
        // Leaf with available capacity.
        // ----------------------------------------------------

        if self.children.is_none() && self.points.len() < self.capacity {
            self.points.push(point);
            return true;
        }

        // ----------------------------------------------------
        // We cannot subdivide any further.
        //
        // This protects us from infinitely subdividing when
        // multiple points occupy the exact same position.
        // ----------------------------------------------------

        if !self.boundary.can_subdivide() {
            self.points.push(point);
            return true;
        }

        // ----------------------------------------------------
        // Subdivide if necessary.
        // ----------------------------------------------------

        if self.children.is_none() {
            self.subdivide();
        }

        // ----------------------------------------------------
        // Insert into one of the four children.
        // ----------------------------------------------------

        if let Some(children) = &mut self.children {
            for child in children.iter_mut() {
                if child.insert(point) {
                    return true;
                }
            }
        }

        false
    }

    // --------------------------------------------------------
    // Subdivide
    // --------------------------------------------------------

    fn subdivide(&mut self) {
        // Don't subdivide twice.
        if self.children.is_some() {
            return;
        }

        let half_width = self.boundary.width * 0.5;
        let half_height = self.boundary.height * 0.5;

        let x = self.boundary.pos.x;
        let y = self.boundary.pos.y;

        let nw = QuadTree::new(
            Rectangle::new(x - half_width, y - half_height, half_width, half_height),
            self.capacity,
        );

        let ne = QuadTree::new(
            Rectangle::new(x + half_width, y - half_height, half_width, half_height),
            self.capacity,
        );

        let sw = QuadTree::new(
            Rectangle::new(x - half_width, y + half_height, half_width, half_height),
            self.capacity,
        );

        let se = QuadTree::new(
            Rectangle::new(x + half_width, y + half_height, half_width, half_height),
            self.capacity,
        );

        self.children = Some(Box::new([nw, ne, sw, se]));

        // ----------------------------------------------------
        // Redistribute existing points.
        //
        // After subdivision, the parent becomes an internal
        // node and no longer owns individual points.
        // ----------------------------------------------------

        let old_points = std::mem::take(&mut self.points);

        if let Some(children) = &mut self.children {
            for point in old_points {
                for child in children.iter_mut() {
                    if child.insert(point) {
                        break;
                    }
                }
            }
        }
    }

    // --------------------------------------------------------
    // Query
    // --------------------------------------------------------

    pub fn query(&self, range: &Rectangle, found: &mut Vec<Point>) {
        // No intersection means nothing below this node
        // can possibly be inside the requested range.
        if !self.boundary.intersects(range) {
            return;
        }

        // Check points stored by this leaf.
        for point in &self.points {
            if range.contains(point.pos) {
                found.push(*point);
            }
        }

        // Search children if this node has been subdivided.
        if let Some(children) = &self.children {
            for child in children.iter() {
                child.query(range, found);
            }
        }
    }

    // --------------------------------------------------------
    // Circle query
    // --------------------------------------------------------

    pub fn query_circle(&self, range: &Circle, found: &mut Vec<Point>) {
        // If the circle doesn't intersect this node's
        // rectangular boundary, there is nothing to search.
        if !range.intersects(&self.boundary) {
            return;
        }

        for point in &self.points {
            if range.contains(point) {
                found.push(*point);
            }
        }

        if let Some(children) = &self.children {
            for child in children.iter() {
                child.query_circle(range, found);
            }
        }
    }

    // --------------------------------------------------------
    // Accessors
    // --------------------------------------------------------

    pub fn boundary(&self) -> Rectangle {
        self.boundary
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn is_divided(&self) -> bool {
        self.children.is_some()
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_none()
    }

    pub fn points(&self) -> &[Point] {
        &self.points
    }

    pub fn children(&self) -> Option<&[QuadTree; 4]> {
        self.children.as_deref()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn world() -> Rectangle {
        Rectangle::new(0.0, 0.0, 100.0, 100.0)
    }

    fn point(x: f32, y: f32) -> Point {
        Point::new(Vec2::new(x, y))
    }

    #[test]
    fn point_inside_boundary_is_inserted() {
        let mut tree = QuadTree::new(world(), 4);

        assert!(tree.insert(point(10.0, 20.0)));

        let range = Rectangle::new(10.0, 20.0, 0.1, 0.1);

        let mut found = Vec::new();
        tree.query(&range, &mut found);

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].pos, Vec2::new(10.0, 20.0));
    }

    #[test]
    fn point_outside_boundary_is_rejected() {
        let mut tree = QuadTree::new(world(), 4);

        assert!(!tree.insert(point(101.0, 0.0)));

        let mut found = Vec::new();
        tree.query(&world(), &mut found);

        assert!(found.is_empty());
    }

    #[test]
    fn points_are_found_in_range() {
        let mut tree = QuadTree::new(world(), 4);

        tree.insert(point(-50.0, -50.0));
        tree.insert(point(0.0, 0.0));
        tree.insert(point(50.0, 50.0));

        let range = Rectangle::new(0.0, 0.0, 10.0, 10.0);

        let mut found = Vec::new();
        tree.query(&range, &mut found);

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].pos, Vec2::new(0.0, 0.0));
    }

    #[test]
    fn subdivision_happens_after_capacity_is_reached() {
        let mut tree = QuadTree::new(world(), 1);

        tree.insert(point(-25.0, -25.0));
        tree.insert(point(25.0, 25.0));

        assert!(tree.is_divided());
        assert!(tree.points().is_empty());
        assert!(tree.children().is_some());
    }

    #[test]
    fn points_are_redistributed_after_subdivision() {
        let mut tree = QuadTree::new(world(), 1);

        let p1 = point(-25.0, -25.0);
        let p2 = point(25.0, 25.0);

        tree.insert(p1);
        tree.insert(p2);

        let mut found = Vec::new();
        tree.query(&world(), &mut found);

        assert_eq!(found.len(), 2);

        assert!(found.iter().any(|p| p.pos == p1.pos));
        assert!(found.iter().any(|p| p.pos == p2.pos));
    }

    #[test]
    fn points_can_be_inserted_into_all_four_quadrants() {
        let mut tree = QuadTree::new(world(), 1);

        let points = [
            point(-25.0, -25.0), // NW
            point(25.0, -25.0),  // NE
            point(-25.0, 25.0),  // SW
            point(25.0, 25.0),   // SE
        ];

        for p in points {
            assert!(tree.insert(p));
        }

        let mut found = Vec::new();
        tree.query(&world(), &mut found);

        assert_eq!(found.len(), 4);

        for expected in points {
            assert!(
                found.iter().any(|p| p.pos == expected.pos),
                "Point {:?} was not found",
                expected
            );
        }
    }

    #[test]
    fn query_does_not_return_points_outside_range() {
        let mut tree = QuadTree::new(world(), 1);

        tree.insert(point(-40.0, -40.0));
        tree.insert(point(40.0, 40.0));
        tree.insert(point(0.0, 0.0));

        let range = Rectangle::new(0.0, 0.0, 5.0, 5.0);

        let mut found = Vec::new();
        tree.query(&range, &mut found);

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].pos, Vec2::new(0.0, 0.0));
    }

    #[test]
    fn circle_query_finds_points_inside_circle() {
        let mut tree = QuadTree::new(world(), 1);

        tree.insert(point(0.0, 0.0));
        tree.insert(point(5.0, 0.0));
        tree.insert(point(20.0, 0.0));

        let circle = Circle::new(0.0, 0.0, 10.0);

        let mut found = Vec::new();
        tree.query_circle(&circle, &mut found);

        assert_eq!(found.len(), 2);

        assert!(found.iter().any(|p| p.pos == Vec2::new(0.0, 0.0)));

        assert!(found.iter().any(|p| p.pos == Vec2::new(5.0, 0.0)));
    }

    #[test]
    fn point_on_rectangle_boundary_is_contained() {
        let rectangle = Rectangle::new(0.0, 0.0, 10.0, 10.0);

        assert!(rectangle.contains(Vec2::new(-10.0, 0.0)));
        assert!(rectangle.contains(Vec2::new(10.0, 0.0)));
        assert!(rectangle.contains(Vec2::new(0.0, -10.0)));
        assert!(rectangle.contains(Vec2::new(0.0, 10.0)));
    }

    #[test]
    fn point_on_circle_boundary_is_contained() {
        let circle = Circle::new(0.0, 0.0, 10.0);

        assert!(circle.contains(&point(10.0, 0.0)));
        assert!(circle.contains(&point(0.0, 10.0)));
    }

    #[test]
    fn coincident_points_do_not_cause_infinite_subdivision() {
        let mut tree = QuadTree::new(world(), 1);

        let p = point(0.0, 0.0);

        for _ in 0..100 {
            assert!(tree.insert(p));
        }

        let mut found = Vec::new();
        tree.query(&world(), &mut found);

        assert_eq!(found.len(), 100);
    }

    #[test]
    fn empty_tree_returns_no_points() {
        let tree = QuadTree::new(world(), 4);

        let mut found = Vec::new();
        tree.query(&world(), &mut found);

        assert!(found.is_empty());
    }

    #[test]
    #[should_panic(expected = "QuadTree capacity must be greater than zero")]
    fn zero_capacity_is_rejected() {
        QuadTree::new(world(), 0);
    }
}
