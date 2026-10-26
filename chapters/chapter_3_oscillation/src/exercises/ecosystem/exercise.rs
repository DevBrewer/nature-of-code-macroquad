use macroquad::{
    color::Color,
    input::{KeyCode, MouseButton, is_key_pressed, is_mouse_button_pressed},
    time::get_frame_time,
};
use runner::{Example, draw_info_panel, draw_world_border, mouse_world_position, world_height};

use crate::exercises::ecosystem::{Fish, Jellyfish, Plankton};

pub struct AquaticEcosystem {
    pub foods: Vec<Plankton>,
    pub jellyfish: Vec<Jellyfish>,
    pub fishes: Vec<Fish>,
}

impl AquaticEcosystem {
    pub fn new() -> Self {
        let food_count = 50;
        let mut foods = Vec::with_capacity(food_count);
        for _ in 0..food_count {
            foods.push(Plankton::random_in_world(25.0));
        }

        let jelly_count = 5;
        let mut jellyfish = Vec::with_capacity(jelly_count);
        for _ in 0..jelly_count {
            jellyfish.push(Jellyfish::random_in_world(40.0));
        }

        let fish_count = 4;
        let mut fishes = Vec::with_capacity(fish_count);
        for _ in 0..fish_count {
            fishes.push(Fish::random_in_world(30.0));
        }

        Self {
            foods,
            jellyfish,
            fishes,
        }
    }
}

impl Default for AquaticEcosystem {
    fn default() -> Self {
        Self::new()
    }
}

impl Example for AquaticEcosystem {
    fn reset(&mut self) {
        *self = Self::new();
    }

    fn update(&mut self) {
        let dt = get_frame_time();

        if is_key_pressed(KeyCode::R) {
            self.reset();
            return;
        }

        // [F] key spawns a fish at mouse position
        if is_key_pressed(KeyCode::F) {
            let mouse = mouse_world_position();
            self.fishes.push(Fish::new(mouse));
        }

        // [J] key spawns a jellyfish at mouse position
        if is_key_pressed(KeyCode::J) {
            let mouse = mouse_world_position();
            self.jellyfish.push(Jellyfish::new(mouse));
        }

        // Left Click to spawn food particle at mouse position
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse = mouse_world_position();
            self.foods.push(Plankton::new(mouse));
        }

        // 1. Update plankton
        for food in &mut self.foods {
            food.update(dt);
        }

        // 2. Update fish (foraging, hunting, tail oscillation, feeding)
        for fish in &mut self.fishes {
            fish.update(dt, &mut self.foods);
        }

        // Remove consumed plankton and replenish if population falls low
        self.foods.retain(|f| f.is_alive);
        while self.foods.len() < 35 {
            self.foods.push(Plankton::random_in_world(25.0));
        }

        // 3. Update jellyfish
        for jelly in &mut self.jellyfish {
            jelly.update(dt);
        }
    }

    fn draw(&self) {
        draw_world_border();

        // 1. Draw glowing food particles
        for food in &self.foods {
            food.draw();
        }

        // 2. Draw pulsating jellyfish
        for jelly in &self.jellyfish {
            jelly.draw();
        }

        // 3. Draw swimming fish with oscillating caudal fins
        for fish in &self.fishes {
            fish.draw();
        }

        let total_eaten: usize = self.fishes.iter().map(|f| f.eaten_count).sum();
        let stats_str = format!(
            "Plankton: {} | Jellyfish: {} | Fish: {} | Consumed: {}",
            self.foods.len(),
            self.jellyfish.len(),
            self.fishes.len(),
            total_eaten
        );
        let lines: [(&str, Color); 5] = [
            (
                "Aquatic Ecosystem: Oscillation Mechanics",
                Color::new(0.4, 0.9, 1.0, 1.0),
            ),
            (
                "Fish: Heading rotation + harmonic tail fin oscillation",
                Color::new(0.8, 0.8, 0.8, 1.0),
            ),
            (
                "[LMB] Spawn Food  |  [F] Spawn Fish  |  [J] Jellyfish",
                Color::new(0.5, 0.9, 0.7, 1.0),
            ),
            ("[R] Reset simulation", Color::new(0.7, 0.7, 0.7, 1.0)),
            (&stats_str, Color::new(0.9, 0.9, 0.4, 1.0)),
        ];
        draw_info_panel(10.0, world_height() - 130.0, 430.0, &lines);
    }
}
