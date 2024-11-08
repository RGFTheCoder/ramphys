use macroquad::shapes::{draw_circle_lines, draw_line};

use crate::{
    math::{collision_manifold::CollisionManifold, ray::Ray, vec2::Pos2},
    util::{Drawable, Transform, DEVLINE_THICKNESS, FG, GREEN},
};

use super::{Collision, Shape};

#[derive(Clone, Debug)]
pub struct Circle {
    pub position: Pos2,
    pub radius: f32,
}

impl Shape for Circle {
    fn displace(&mut self, delta: crate::math::vec2::Vec2) {
        self.position += delta;
    }

    fn rotate(&mut self, _theta: f32) {}

    fn center(&self) -> Pos2 {
        self.position
    }
}

impl Drawable for Circle {
    fn draw(&self, transform: &crate::util::DrawTransform) {
        let t_center = transform.transform(self.position);
        let t_radius = transform.transform(self.radius);

        draw_circle_lines(t_center.x, t_center.y, t_radius, DEVLINE_THICKNESS, FG);
        draw_line(
            t_center.x,
            t_center.y,
            t_center.x + t_radius,
            t_center.y,
            DEVLINE_THICKNESS,
            FG,
        );
    }
}

impl Collision for Circle {
    fn collides(&self, other: &Self) -> Option<CollisionManifold> {
        let dir = other.position - self.position;
        let dist2 = dir.length_squared();
        let sum_radius = self.radius + other.radius;

        if dist2 < sum_radius.powi(2) {
            let dist = dist2.sqrt();
            let normal = dir / dist;
            let depth = sum_radius - dist;
            Some(CollisionManifold {
                depth,
                normal,
                penetration: Ray {
                    origin: self.position + normal * self.radius,
                    direction: normal * depth,
                },
            })
        } else {
            None
        }
    }
}
