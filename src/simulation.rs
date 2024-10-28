use macroquad::{color::BLUE, shapes::draw_rectangle};

use crate::util::Drawable;

pub struct Simulation {}

impl Simulation {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, dt: f32) {}
}

impl Drawable for Simulation {
    fn draw(&self, transform: &crate::util::DrawTransform) {}
}
