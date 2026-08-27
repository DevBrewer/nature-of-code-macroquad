use std::sync::atomic::{AtomicU32, Ordering};

use macroquad::{
    camera::{Camera2D, set_camera},
    color::WHITE,
    math::{Rect, Vec2 as MacroquadVec2, vec2},
    texture::{DrawTextureParams, FilterMode, RenderTarget, draw_texture_ex, render_target},
    window::{screen_height, screen_width},
};

use vec_math::Vec2;

static WORLD_WIDTH_BITS: AtomicU32 = AtomicU32::new(0);
static WORLD_HEIGHT_BITS: AtomicU32 = AtomicU32::new(0);

/// Default world canvas width.
pub const WORLD_WIDTH: f32 = 400.0;

/// Default world canvas height.
pub const WORLD_HEIGHT: f32 = 400.0;

/// Sets the dimensions used by the global world-coordinate helpers.
///
/// The Camera remains the authoritative owner of its own dimensions.
/// These globals exist as a compatibility layer for rendering helpers
/// and existing examples.
pub fn set_world_size(width: f32, height: f32) {
    WORLD_WIDTH_BITS.store(width.to_bits(), Ordering::Relaxed);
    WORLD_HEIGHT_BITS.store(height.to_bits(), Ordering::Relaxed);
}

/// Returns the current world canvas width (defaults to WORLD_WIDTH = 400).
pub fn world_width() -> f32 {
    let bits = WORLD_WIDTH_BITS.load(Ordering::Relaxed);
    if bits == 0 {
        WORLD_WIDTH
    } else {
        f32::from_bits(bits)
    }
}

/// Returns the current world canvas height (defaults to WORLD_HEIGHT = 400).
pub fn world_height() -> f32 {
    let bits = WORLD_HEIGHT_BITS.load(Ordering::Relaxed);
    if bits == 0 {
        WORLD_HEIGHT
    } else {
        f32::from_bits(bits)
    }
}

/// Returns the current world canvas dimensions as a Vec2.
pub fn world_size() -> Vec2 {
    Vec2::new(world_width(), world_height())
}

/// Returns the center point of the world canvas as a Vec2.
pub fn world_center() -> Vec2 {
    Vec2::new(world_width() * 0.5, world_height() * 0.5)
}

/// Converts the current mouse position on screen to world canvas coordinates.
pub fn mouse_world_position() -> Vec2 {
    let (mx, my) = macroquad::input::mouse_position();
    let screen_w = screen_width();
    let screen_h = screen_height();
    let ww = world_width();
    let wh = world_height();

    let scale = (screen_w / ww).min(screen_h / wh);
    let viewport_width = ww * scale;
    let viewport_height = wh * scale;

    let viewport_x = (screen_w - viewport_width) * 0.5;
    let viewport_y = (screen_h - viewport_height) * 0.5;

    let world_x = (mx - viewport_x) / scale;
    let world_y = (my - viewport_y) / scale;

    Vec2::new(world_x, world_y)
}

pub struct Camera {
    render_target: RenderTarget,
    camera: Camera2D,
    world_width: f32,
    world_height: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(WORLD_WIDTH, WORLD_HEIGHT)
    }
}

impl Camera {
    /// Creates a Camera with explicit world width and height.
    pub fn new(world_width: f32, world_height: f32) -> Self {
        set_world_size(world_width, world_height);

        let render_target = render_target(world_width as u32, world_height as u32);
        render_target.texture.set_filter(FilterMode::Nearest);

        let mut camera =
            Camera2D::from_display_rect(Rect::new(0.0, 0.0, world_width, world_height));
        camera.render_target = Some(render_target.clone());

        Self {
            render_target,
            camera,
            world_width,
            world_height,
        }
    }

    /// Automatically creates a Camera using the current window / screen dimensions.
    pub fn from_screen() -> Self {
        let sw = screen_width();
        let sh = screen_height();
        let w = if sw > 0.0 { sw } else { WORLD_WIDTH };
        let h = if sh > 0.0 { sh } else { WORLD_HEIGHT };
        Self::new(w, h)
    }

    pub fn world_width(&self) -> f32 {
        self.world_width
    }

    pub fn world_height(&self) -> f32 {
        self.world_height
    }

    pub fn world_size(&self) -> Vec2 {
        Vec2::new(self.world_width, self.world_height)
    }

    pub fn update(&mut self) {}

    pub fn apply(&self) {
        set_camera(&self.camera);
    }

    pub fn screen_to_world(&self, position: (f32, f32)) -> Vec2 {
        let screen = MacroquadVec2::new(position.0, position.1);

        let screen_w = screen_width();
        let screen_h = screen_height();

        let scale = (screen_w / self.world_width).min(screen_h / self.world_height);

        let viewport_width = self.world_width * scale;
        let viewport_height = self.world_height * scale;

        let viewport_x = (screen_w - viewport_width) * 0.5;
        let viewport_y = (screen_h - viewport_height) * 0.5;

        let world_x = (screen.x - viewport_x) / scale;
        let world_y = (screen.y - viewport_y) / scale;

        Vec2::new(world_x, world_y)
    }

    pub fn mouse_world_position(&self) -> Vec2 {
        let (mx, my) = macroquad::input::mouse_position();
        self.screen_to_world((mx, my))
    }

    pub fn present(&self) {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // One uniform scale factor.
        //
        // This is the critical part:
        //
        //     scale_x != scale_y
        //
        // is NOT allowed.
        let scale = (screen_w / self.world_width).min(screen_h / self.world_height);

        let width = self.world_width * scale;
        let height = self.world_height * scale;

        let x = (screen_w - width) * 0.5;
        let y = (screen_h - height) * 0.5;

        draw_texture_ex(
            &self.render_target.texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(width, height)),
                flip_y: true,
                ..Default::default()
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_world_sizing() {
        set_world_size(800.0, 600.0);
        assert_eq!(world_width(), 800.0);
        assert_eq!(world_height(), 600.0);
        assert_eq!(world_size(), Vec2::new(800.0, 600.0));
        assert_eq!(world_center(), Vec2::new(400.0, 300.0));

        set_world_size(400.0, 400.0);
        assert_eq!(world_width(), 400.0);
        assert_eq!(world_height(), 400.0);
        assert_eq!(world_center(), Vec2::new(200.0, 200.0));
    }
}
