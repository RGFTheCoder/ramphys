use macroquad::{input::is_key_down, time::get_frame_time};

use crate::{
    math::vec2::Vec2,
    shapes::{Collision, Shape, ShapeVariant},
    util::{DrawTransform, Drawable},
};

pub struct Simulation {
    pub items: Vec<ShapeVariant>,
}

impl Simulation {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn update(&mut self, dt: f32) {
        if is_key_down(macroquad::input::KeyCode::W) {
            self.items[0].displace(Vec2 {
                x: 0.,
                y: -5. * get_frame_time(),
            });
        }
        if is_key_down(macroquad::input::KeyCode::S) {
            self.items[0].displace(Vec2 {
                x: 0.,
                y: 5. * get_frame_time(),
            });
        }
        if is_key_down(macroquad::input::KeyCode::A) {
            self.items[0].displace(Vec2 {
                y: 0.,
                x: -5. * get_frame_time(),
            });
        }
        if is_key_down(macroquad::input::KeyCode::D) {
            self.items[0].displace(Vec2 {
                y: 0.,
                x: 5. * get_frame_time(),
            });
        }
        if is_key_down(macroquad::input::KeyCode::Q) {
            self.items[0].rotate(-get_frame_time());
        }
        if is_key_down(macroquad::input::KeyCode::E) {
            self.items[0].rotate(get_frame_time());
        }

        let t = DrawTransform {
            x: 0.,
            y: 0.,
            zoom: 25.,
        };

        if is_key_down(macroquad::input::KeyCode::Space) {
            for i in 0..self.items.len() {
                for j in 0..self.items.len() {
                    if i == j {
                        continue;
                    }

                    if let Some(col) = self.items[i].collides(&self.items[j]) {
                        let half_dir = col.penetration.direction * 0.5;
                        self.items[i].displace(-half_dir);
                        self.items[j].displace(half_dir);
                    }
                }
            }
        }
    }
}

impl Drawable for Simulation {
    fn draw(&self, transform: &crate::util::DrawTransform) {
        for i in &self.items {
            i.draw(transform)
        }
    }
}
