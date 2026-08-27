// vec_math/src/vector2.rs
use rand::RngExt;
use std::f32::consts::TAU;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    /// The zero vector `<0, 0>`
    pub const ZERO: Self = Self::new(0.0, 0.0);

    /// A vector with both components set to `1`.
    pub const ONE: Self = Self::new(1.0, 1.0);

    /// Creates a new 2D Vector
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Create a duplicate vector.
    pub fn copy(&self) -> Self {
        Self::new(self.x, self.y)
    }

    /// Creates a unit vector pointing in the direction of the given angle.
    ///
    /// The angle is measured in radians
    pub fn from_angle(angle: f32) -> Self {
        Self::new(angle.cos(), angle.sin())
    }

    /// Generates a random 2D unit vector (Magnitude = 1.0)
    pub fn random_2d() -> Self {
        let mut rng = rand::rng();
        let theta = rng.random_range(0.0..TAU);

        Self::from_angle(theta)
    }

    /// Calculates the magnitude (length) of the vector: sqrt(x^2 + y^2)
    pub fn mag(&self) -> f32 {
        self.x.hypot(self.y)
    }

    /// Calculates the squared magnitude (faster, avoids square root)
    pub fn mag_sq(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// Normalizes the vector to a unit length of 1.0 (Safe from division by zero)
    pub fn normalize(&mut self) {
        let m = self.mag();
        if m != 0.0 {
            *self /= m;
        }
    }

    /// Returns a new normalized copy of the vector without changing the original
    pub fn normalized(&self) -> Self {
        let m = self.mag();
        if m != 0.0 {
            *self / m
        } else {
            Self::new(0.0, 0.0)
        }
    }

    /// Limits the maximum magnitude of the vector (Crucial for acceleration/force limits)
    pub fn limit(&mut self, max: f32) {
        if self.mag_sq() > max * max {
            self.normalize();
            *self *= max;
        }
    }

    /// Sets the exact magnitude of the vector regardless of its current length
    pub fn set_mag(&mut self, magnitude: f32) {
        self.normalize();
        *self *= magnitude;
    }

    /// Calculates the dot product of two vectors
    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Calculates the Euclidean distance between two vector positions
    pub fn distance(&self, other: Self) -> f32 {
        (*self - other).mag()
    }

    /// Calculates the squared distance between two positions.
    pub const fn distance_squared(&self, other: Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        dx * dx + dy * dy
    }

    /// Returns the direction of this vector in radains.
    pub fn heading(&self) -> f32 {
        self.y.atan2(self.x)
    }

    /// Calculate the unsigned angle between this vector and another vector.
    pub fn angle_between(&self, other: Self) -> f32 {
        let denominator = (self.mag_sq() * other.mag_sq()).sqrt();

        if denominator == 0.0 {
            return 0.0;
        }

        let cosine = (self.dot(other) / denominator).clamp(-1.0, 1.0);

        cosine.acos()
    }

    /// Linear interpolates between this vector and another vector.
    /// `t = 0` return this vector
    /// `t = 1` return another vector
    pub fn lerp(&self, other: Self, t: f32) -> Self {
        *self + (other - *self) * t
    }

    /// Rotates this vector by the given angle in radians.
    pub fn rotate(&self, angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();

        Self::new(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
    }

    /// Returns the reflection of this vector about the given surface normal.
    /// The supplied normal **must** be normalized (unit length).
    /// Formaula : r = v - 2(v .n)n
    pub fn reflected(&self, normal: Self) -> Self {
        *self - normal * (2.0 * self.dot(normal))
    }

    /// Reflects this vector about the given surface normal in place.
    pub fn reflect(&mut self, normal: Self) {
        *self = self.reflected(normal);
    }
}

// ==========================================
// OPERATOR OVERLOADING (Rust Trait Implementations)
// ==========================================

// Vector + Vector (e.g., position + velocity)
impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}
impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

// Vector - Vector (e.g., target - position for steering)
impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}
impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

// Vector * f32 Scalar (Scaling up velocity or forces)
impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self::new(self.x * scalar, self.y * scalar)
    }
}
impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

// Vector / f32 Scalar (Dividing forces by mass: acceleration = force / mass)
impl Div<f32> for Vec2 {
    type Output = Self;
    fn div(self, scalar: f32) -> Self {
        Self::new(self.x / scalar, self.y / scalar)
    }
}
impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

// - vector
impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}
