use macroquad::{
    color::{Color, GRAY, GREEN, RED, WHITE, YELLOW},
    input::{KeyCode, is_key_down, is_key_pressed},
    math::vec2,
    shapes::{draw_circle, draw_line, draw_triangle},
    time::get_frame_time,
};
use runner::{Example, draw_info_panel, draw_vector, draw_world_border};
use vec_math::Vec2;

const DEFAULT_GRAVITY: f32 = 350.0;
const RAMP_LENGTH: f32 = 420.0;
const BOX_WIDTH: f32 = 50.0;
const BOX_HEIGHT: f32 = 34.0;
const VECTOR_VISUAL_SCALE: f32 = 0.45;

/// Draws a rotated rectangle with a solid fill and crisp border lines
fn draw_rotated_box(center: Vec2, width: f32, height: f32, angle: f32, fill: Color, border: Color) {
    let (sin, cos) = angle.sin_cos();
    let hw = width * 0.5;
    let hh = height * 0.5;

    let local_corners = [
        Vec2::new(-hw, -hh),
        Vec2::new(hw, -hh),
        Vec2::new(hw, hh),
        Vec2::new(-hw, hh),
    ];

    let world_corners: Vec<Vec2> = local_corners
        .iter()
        .map(|c| center + Vec2::new(c.x * cos - c.y * sin, c.x * sin + c.y * cos))
        .collect();

    // Two triangles forming the box interior
    draw_triangle(
        vec2(world_corners[0].x, world_corners[0].y),
        vec2(world_corners[1].x, world_corners[1].y),
        vec2(world_corners[2].x, world_corners[2].y),
        fill,
    );
    draw_triangle(
        vec2(world_corners[0].x, world_corners[0].y),
        vec2(world_corners[2].x, world_corners[2].y),
        vec2(world_corners[3].x, world_corners[3].y),
        fill,
    );

    // Outline perimeter
    for i in 0..4 {
        let p1 = world_corners[i];
        let p2 = world_corners[(i + 1) % 4];
        draw_line(p1.x, p1.y, p2.x, p2.y, 1.8, border);
    }
}

/// Exercise 3.16: Inclined Plane & Normal Force
///
/// ```text
/// ============================================================================
/// MATHEMATICAL FORMULATION & FREE BODY DIAGRAM (FBD)
/// ============================================================================
///
/// 1. The System:
///    A box of mass `m` resting on an incline wedge tilted at angle `θ` with the horizontal.
///
/// 2. Coordinate Orientation (Screen space where y points downward):
///    - Downhill slope tangent vector:  t̂ = ( cos(θ),  sin(θ) )
///    - Perpendicular normal vector:    n̂ = ( sin(θ), -cos(θ) )  (points upward/out of surface)
///    - Incline normal into surface:   -n̂ = (-sin(θ),  cos(θ) )
///
/// 3. Gravity Vector:
///    F_g = (0, m * g)  (points straight down in screen space)
///
/// 4. Force Decomposition:
///    - Parallel downhill force:
///        F_parallel = (F_g · t̂) * t̂ = (m * g * sin(θ)) * t̂
///    - Perpendicular normal compression into the surface:
///        F_perp = (F_g · -n̂) * (-n̂) = (m * g * cos(θ)) * (-n̂)
///
///    Identity:
///        F_parallel + F_perp = (0, m * g) = F_g
///
/// 5. Normal Force (F_N):
///    The solid incline resists penetration, exerting a contact force perpendicular
///    to the surface pointing outward into the air:
///        F_N = (m * g * cos(θ)) * n̂
///
/// 6. Friction Force (f):
///    Opposes downhill motion with coefficient μ:
///        f = μ * |F_N| = μ * m * g * cos(θ)
///
/// 7. Net Downhill Acceleration:
///    - If tan(θ) <= μ: Static friction holds the box in place (a = 0)
///    - If tan(θ) > μ:  a = g * (sin(θ) - μ * cos(θ))
/// ============================================================================
/// ```
pub struct InclinedPlaneExercise {
    /// Incline angle θ in radians (typically 0.15 rad to 1.25 rad)
    pub angle: f32,
    /// Box mass in arbitrary units
    pub mass: f32,
    /// Distance along the hypotenuse from ramp top [0.0, RAMP_LENGTH]
    pub position_s: f32,
    /// Velocity down the slope
    pub velocity_s: f32,
    /// Acceleration down the slope
    pub acceleration_s: f32,
    /// Coefficient of kinetic/static friction
    pub friction_coefficient: f32,
    /// Whether the box is actively sliding or paused in static analysis
    pub is_sliding: bool,
    /// Whether friction is enabled
    pub friction_enabled: bool,
}

impl InclinedPlaneExercise {
    pub fn new() -> Self {
        Self {
            angle: 30.0_f32.to_radians(), // Default classic 30-degree incline
            mass: 1.0,
            position_s: 70.0,
            velocity_s: 0.0,
            acceleration_s: 0.0,
            friction_coefficient: 0.20,
            is_sliding: true,
            friction_enabled: true,
        }
    }

    /// Calculates downhill tangent vector t̂
    pub fn tangent_vector(&self) -> Vec2 {
        Vec2::new(self.angle.cos(), self.angle.sin())
    }

    /// Calculates upward normal vector n̂ (perpendicular to ramp into air)
    pub fn normal_vector(&self) -> Vec2 {
        Vec2::new(self.angle.sin(), -self.angle.cos())
    }

    /// Normal Force magnitude: F_N = m * g * cos(θ)
    pub fn normal_force_magnitude(&self) -> f32 {
        self.mass * DEFAULT_GRAVITY * self.angle.cos()
    }

    /// Parallel downhill force magnitude: F_parallel = m * g * sin(θ)
    pub fn parallel_force_magnitude(&self) -> f32 {
        self.mass * DEFAULT_GRAVITY * self.angle.sin()
    }

    /// Friction force magnitude: f = μ * F_N (if active)
    pub fn friction_force_magnitude(&self) -> f32 {
        if self.friction_enabled {
            self.friction_coefficient * self.normal_force_magnitude()
        } else {
            0.0
        }
    }

    /// Computes net acceleration down the incline
    pub fn net_acceleration(&self) -> f32 {
        let f_parallel = self.parallel_force_magnitude();
        let f_friction = self.friction_force_magnitude();

        if f_parallel <= f_friction {
            // Static friction holds the box
            0.0
        } else {
            (f_parallel - f_friction) / self.mass
        }
    }

    /// Returns the 3 wedge vertices (top, bottom-left wall, bottom-right base)
    pub fn wedge_vertices(&self) -> (Vec2, Vec2, Vec2) {
        let base_corner = Vec2::new(510.0, 500.0);
        let top_corner = Vec2::new(
            base_corner.x - RAMP_LENGTH * self.angle.cos(),
            base_corner.y - RAMP_LENGTH * self.angle.sin(),
        );
        let wall_corner = Vec2::new(top_corner.x, base_corner.y);

        (top_corner, wall_corner, base_corner)
    }

    /// Returns the world position of the box center
    pub fn box_center(&self) -> Vec2 {
        let (top, _, _) = self.wedge_vertices();
        let tangent = self.tangent_vector();
        let normal = self.normal_vector();

        let surface_point = top + tangent * self.position_s;
        surface_point + normal * (BOX_HEIGHT * 0.5)
    }
}

impl Example for InclinedPlaneExercise {
    fn reset(&mut self) {
        self.position_s = 60.0;
        self.velocity_s = 0.0;
        self.acceleration_s = 0.0;
    }

    fn update(&mut self) {
        let dt = get_frame_time().min(0.033);

        // Adjust incline angle θ with Up/Down or W/S
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            self.angle = (self.angle + 0.5 * dt).min(75.0_f32.to_radians());
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            self.angle = (self.angle - 0.5 * dt).max(5.0_f32.to_radians());
        }

        // Adjust friction with Left/Right or A/D
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.friction_coefficient = (self.friction_coefficient + 0.2 * dt).min(0.80);
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.friction_coefficient = (self.friction_coefficient - 0.2 * dt).max(0.0);
        }

        // Toggle friction
        if is_key_pressed(KeyCode::F) {
            self.friction_enabled = !self.friction_enabled;
        }

        // Toggle sliding vs static study
        if is_key_pressed(KeyCode::Space) {
            self.is_sliding = !self.is_sliding;
        }

        // Reset box position
        if is_key_pressed(KeyCode::R) {
            self.reset();
        }

        // Physics integration
        self.acceleration_s = self.net_acceleration();

        if self.is_sliding {
            self.velocity_s += self.acceleration_s * dt;
            self.position_s += self.velocity_s * dt;

            // Reset when reaching bottom of ramp
            if self.position_s > RAMP_LENGTH - (BOX_WIDTH * 0.5) {
                self.position_s = 60.0;
                self.velocity_s = 0.0;
            }
        }
    }

    fn draw(&self) {
        draw_world_border();

        let (top, wall, base) = self.wedge_vertices();

        // 1. Draw solid inclined plane wedge
        draw_triangle(
            vec2(top.x, top.y),
            vec2(wall.x, wall.y),
            vec2(base.x, base.y),
            Color::new(0.12, 0.16, 0.24, 0.95),
        );
        // Wedge border lines
        draw_line(wall.x, wall.y, base.x, base.y, 2.0, GRAY);
        draw_line(wall.x, wall.y, top.x, top.y, 2.0, GRAY);
        // Bright highlighted hypotenuse (the incline surface)
        draw_line(
            top.x,
            top.y,
            base.x,
            base.y,
            3.0,
            Color::new(0.35, 0.65, 1.0, 1.0),
        );

        // 2. Draw angle arc at base corner
        let arc_radius = 45.0;
        let arc_steps = 16;
        for i in 0..arc_steps {
            let a1 = self.angle * (i as f32 / arc_steps as f32);
            let a2 = self.angle * ((i + 1) as f32 / arc_steps as f32);
            draw_line(
                base.x - arc_radius * a1.cos(),
                base.y - arc_radius * a1.sin(),
                base.x - arc_radius * a2.cos(),
                base.y - arc_radius * a2.sin(),
                1.5,
                YELLOW,
            );
        }

        // 3. Draw rotated box resting on slope
        let box_pos = self.box_center();
        draw_rotated_box(
            box_pos,
            BOX_WIDTH,
            BOX_HEIGHT,
            self.angle,
            Color::new(0.20, 0.30, 0.45, 0.95),
            Color::new(0.50, 0.75, 1.0, 1.0),
        );

        // 4. Draw Free Body Diagram Force Vectors
        let tangent = self.tangent_vector();
        let normal = self.normal_vector();

        // A. Gravity vector F_g = (0, mg) [Green]
        let f_g_vec = Vec2::new(0.0, self.mass * DEFAULT_GRAVITY * VECTOR_VISUAL_SCALE);
        draw_vector(box_pos, f_g_vec, GREEN, Some("Gravity (Fg)"));

        // B. Normal force vector F_N = (mg * cos(θ)) * n̂ [Cyan]
        let f_n_vec = normal * (self.normal_force_magnitude() * VECTOR_VISUAL_SCALE);
        draw_vector(
            box_pos,
            f_n_vec,
            Color::new(0.2, 0.85, 1.0, 1.0),
            Some("Normal (FN)"),
        );

        // C. Parallel slope force F_parallel = (mg * sin(θ)) * t̂ [Yellow]
        let f_par_vec = tangent * (self.parallel_force_magnitude() * VECTOR_VISUAL_SCALE);
        draw_vector(box_pos, f_par_vec, YELLOW, Some("Parallel (F||)"));

        // D. Friction force opposing motion f = -μ * F_N * t̂ [Red]
        if self.friction_enabled && self.friction_force_magnitude() > 0.0 {
            let f_fric_vec = tangent * (-self.friction_force_magnitude() * VECTOR_VISUAL_SCALE);
            draw_vector(box_pos, f_fric_vec, RED, Some("Friction (mu)"));
        }

        // E. Perpendicular gravity component projection line [Dotted/gray guide]
        let f_perp_vec = normal * (-self.normal_force_magnitude() * VECTOR_VISUAL_SCALE);
        draw_line(
            box_pos.x + f_par_vec.x,
            box_pos.y + f_par_vec.y,
            box_pos.x + f_g_vec.x,
            box_pos.y + f_g_vec.y,
            1.2,
            Color::new(0.6, 0.6, 0.7, 0.5),
        );
        draw_line(
            box_pos.x + f_perp_vec.x,
            box_pos.y + f_perp_vec.y,
            box_pos.x + f_g_vec.x,
            box_pos.y + f_g_vec.y,
            1.2,
            Color::new(0.6, 0.6, 0.7, 0.5),
        );

        // Center pin on box
        draw_circle(box_pos.x, box_pos.y, 3.5, WHITE);

        // 5. HUD Information Panel
        let deg = self.angle.to_degrees();
        let angle_str = format!(
            "Incline Angle: {:.1}° [{:.2} rad]  [W/S or UP/DOWN]",
            deg, self.angle
        );

        let fn_str = format!(
            "Normal Force F_N: {:.1} N  (mg·cos(theta))",
            self.normal_force_magnitude()
        );

        let fpar_str = format!(
            "Parallel Force F_||: {:.1} N  (mg·sin(theta))",
            self.parallel_force_magnitude()
        );

        let fric_str = if self.friction_enabled {
            format!(
                "Friction: mu = {:.2} [f = {:.1} N]  [A/D, F: toggle]",
                self.friction_coefficient,
                self.friction_force_magnitude()
            )
        } else {
            "Friction: Disabled (Frictionless surface)  [F: toggle]".to_string()
        };

        let status_str = if self.acceleration_s == 0.0 && self.is_sliding {
            "Motion: Held by static friction (tan(theta) <= mu)".to_string()
        } else if self.is_sliding {
            format!(
                "Motion: Sliding (a = {:.1} px/s², v = {:.1})",
                self.acceleration_s, self.velocity_s
            )
        } else {
            "Motion: Paused [SPACE: slide]".to_string()
        };

        let lines = [
            ("EXERCISE 3.16: INCLINED PLANE & NORMAL FORCE", WHITE),
            (&angle_str, Color::new(0.35, 0.85, 1.0, 1.0)),
            (&fn_str, Color::new(0.25, 0.85, 1.0, 1.0)),
            (&fpar_str, YELLOW),
            (&fric_str, if self.friction_enabled { RED } else { GRAY }),
            (&status_str, GREEN),
            ("[SPACE] Slide/Pause  |  [R] Reset  |  [H] Toggle HUD", GRAY),
        ];
        draw_info_panel(10.0, 10.0, 480.0, &lines);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_force_flat_and_vertical() {
        let mut sim = InclinedPlaneExercise::new();

        // At θ = 0 (horizontal ground):
        sim.angle = 0.0;
        assert!((sim.normal_force_magnitude() - DEFAULT_GRAVITY).abs() < 1e-4);
        assert!(sim.parallel_force_magnitude().abs() < 1e-4);

        // At θ = 90 deg (vertical wall):
        sim.angle = 90.0_f32.to_radians();
        assert!(sim.normal_force_magnitude().abs() < 1e-3);
        assert!((sim.parallel_force_magnitude() - DEFAULT_GRAVITY).abs() < 1e-3);
    }

    #[test]
    fn test_vectors_orthogonal() {
        let sim = InclinedPlaneExercise::new();
        let t = sim.tangent_vector();
        let n = sim.normal_vector();

        // Dot product of tangent and normal must be zero
        let dot = t.x * n.x + t.y * n.y;
        assert!(dot.abs() < 1e-6);
    }

    #[test]
    fn test_vector_decomposition_sum() {
        let sim = InclinedPlaneExercise::new();
        let t = sim.tangent_vector();
        let n = sim.normal_vector();

        let f_parallel = t * sim.parallel_force_magnitude();
        let f_perp_into_surface = n * (-sim.normal_force_magnitude());

        // Vector sum of parallel and perpendicular components must equal gravity (0, mg)
        let sum = f_parallel + f_perp_into_surface;
        assert!(sum.x.abs() < 1e-4);
        assert!((sum.y - (sim.mass * DEFAULT_GRAVITY)).abs() < 1e-4);
    }

    #[test]
    fn test_critical_angle_friction() {
        let mut sim = InclinedPlaneExercise::new();
        sim.friction_coefficient = 0.50; // tan(θ_c) = 0.5 => θ_c ≈ 26.56°

        // Below critical angle (e.g. 20 deg): static friction holds box (a = 0)
        sim.angle = 20.0_f32.to_radians();
        assert_eq!(sim.net_acceleration(), 0.0);

        // Above critical angle (e.g. 35 deg): box accelerates down (a > 0)
        sim.angle = 35.0_f32.to_radians();
        assert!(sim.net_acceleration() > 0.0);
    }
}
