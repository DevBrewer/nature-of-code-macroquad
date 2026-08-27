use std::f32::consts::TAU;

// Vec_math/src/noise.rs
/// Smooth interperpolation curve use by Perlin noise.
///
/// Unlike simple linear interpolation, this curve has a
/// smooth slope at both ends.
// Fade function as defined by Ken Perlin (6t^5 - 15t^4 + 10t^3)
fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Linear interpolation.
/// Returns a value between a and b according to t
/// t = 0 -> a
/// t = 1 -> b
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

/// Gradient contribution from a lattice point.
/// grad_1dient of either -1 or +1
fn grad_1d(hash: u32, x: f32) -> f32 {
    let grad_1dient = if hash & 1 == 0 { 1.0 } else { -1.0 };

    grad_1dient * x
}

// Return one of eight possible 2D gradient directions.
fn grad_2d(hash: u32) -> (f32, f32) {
    let angle_index = hash & 7;
    let angle = angle_index as f32 * (TAU / 8.0);

    (angle.cos(), angle.sin())
}

/// Dot product between a gradient and the distance
/// from the latice point to the sample point.
fn gradient_dot(hash: u32, dx: f32, dy: f32) -> f32 {
    let (gx, gy) = grad_2d(hash);
    gx * dx + gy * dy
}
/// Generate a smooth 1D PERLIN-NOISE value.
///
/// The input x is continuous, so nearby x values produce
/// smoothly related results.
pub fn perlin_1d(x: f32) -> f32 {
    // Which lattice cell are we inside?
    let x0 = x.floor() as i32;

    // Position inside the cell.
    let t = x - x.floor();

    // Distance from the left lattice point.
    let left = grad_1d(hash_1d(x0), t);

    // Distance from the left lattice point.
    let right = grad_1d(hash_1d(x0 + 1), t - 1.0);

    // Smooth the interpolation amount.
    let u = fade(t);

    // Blend the two grad_1dient contributions.
    lerp(left, right, u)
}

/// Simple deterministic hash.
///
/// The important property is that the same lattice point
/// always receives the same grad_1dient.
fn hash_1d(x: i32) -> u32 {
    let mut value = x as u32;

    value ^= value >> 16;
    value = value.wrapping_mul(0x45d93b);
    value ^= value >> 16;
    value = value.wrapping_mul(0x45d93b);
    value ^= value >> 16;

    value
}

/// Deterministically combine two integer coordinates.
fn hash_2d(x: i32, y: i32) -> u32 {
    let mut value = (x as u32).wrapping_mul(374761393);
    value = value.wrapping_add((y as u32).wrapping_mul(668265263));

    value ^= value >> 13;
    value = value.wrapping_mul(1274126177);
    value ^= value >> 16;

    value
}
/// Generates a smooth 2D Perlin-noise value.
/// ///
/// /// Nearby (x, y) coordinates produce smoothly related values.
pub fn perlin_2d(x: f32, y: f32) -> f32 {
    // Find the lattice cell containing the sample.
    let x0 = x.floor() as i32;
    let y0 = y.floor() as i32;
    // Fractional position inside the cell.
    let tx = x - x.floor();
    let ty = y - y.floor();
    // Four corners of the lattice cell:
    //
    // (x0, y0) -------- (x0 + 1, y0)
    //     |                  |
    //     |       (x,y)      |
    //     |                  |
    // (x0, y0 + 1) ---- (x0 + 1, y0 + 1)
    let n00 = gradient_dot(hash_2d(x0, y0), tx, ty);
    let n10 = gradient_dot(hash_2d(x0 + 1, y0), tx - 1.0, ty);
    let n01 = gradient_dot(hash_2d(x0, y0 + 1), tx, ty - 1.0);
    let n11 = gradient_dot(hash_2d(x0 + 1, y0 + 1), tx - 1.0, ty - 1.0);
    // Smooth interpolation coordinates.
    let u = fade(tx);
    let v = fade(ty);
    // Interpolate along X first.
    let nx0 = lerp(n00, n10, u);
    let nx1 = lerp(n01, n11, u);
    // Then interpolate along Y.
    lerp(nx0, nx1, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nearby_inputs_produce_nearby_values() {
        let a = perlin_1d(2.10);
        let b = perlin_1d(2.11);

        let difference = (a - b).abs();

        assert!(difference < 0.1);
    }

    #[test]
    fn same_input_is_deterministic() {
        let a = perlin_1d(3.25);
        let b = perlin_1d(3.25);

        let c = perlin_2d(2.35, 4.72);
        let d = perlin_2d(2.35, 4.72);

        assert_eq!(a, b);
        assert_eq!(c, d);
    }
}
