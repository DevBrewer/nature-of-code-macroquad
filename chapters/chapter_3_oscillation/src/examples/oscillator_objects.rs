use runner::{Example, draw_world_border};

use crate::oscillator::Oscillator;

pub struct OscillatorObjectExample {
    oscillators: Vec<Oscillator>,
}

impl OscillatorObjectExample {
    pub fn new() -> Self {
        let oscillators = (0..10).map(|_| Oscillator::new()).collect();

        Self { oscillators }
    }
}

impl Example for OscillatorObjectExample {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        self.oscillators
            .iter_mut()
            .for_each(|oscillator| oscillator.update());
    }

    fn draw(&self) {
        draw_world_border();

        // Draw bod objects
        self.oscillators
            .iter()
            .for_each(|oscillator| oscillator.show());
    }
}
