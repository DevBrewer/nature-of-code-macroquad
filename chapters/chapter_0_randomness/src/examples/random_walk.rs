use runner::{Example, draw_world_border};

use crate::walker::Walker;

pub struct TraditionalRandomWalk {
    walker: Walker,
}

impl TraditionalRandomWalk {
    pub fn new() -> Self {
        Self {
            walker: Walker::new(),
        }
    }
}

impl Example for TraditionalRandomWalk {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        self.walker.step();
    }

    fn draw(&self) {
        draw_world_border();
        self.walker.draw();
    }

    fn clear_background(&self) -> bool {
        false
    }
}
