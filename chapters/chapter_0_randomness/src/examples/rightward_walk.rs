use runner::{Example, render::draw_world_border};

use crate::walker_right::RightWalker;

pub struct RightwardWalk {
    walker: RightWalker,
}

impl RightwardWalk {
    pub fn new() -> Self {
        Self {
            walker: RightWalker::new(),
        }
    }
}

impl Example for RightwardWalk {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        self.walker.step();
    }

    fn draw(&self) {
        self.walker.draw();
        draw_world_border();
    }

    // Keep the previous path visible.
    fn clear_background(&self) -> bool {
        false
    }
}
