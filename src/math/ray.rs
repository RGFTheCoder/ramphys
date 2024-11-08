use std::ops::Neg;

use macroquad::shapes::draw_line;

use crate::util::{DrawTransform, Drawable, Transform, BLUE, DEVLINE_THICKNESS, FG};

use super::vec2::{Pos2, Vec2};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Pos2,
    pub direction: Vec2,
}

impl Ray {
    pub fn new(origin: Pos2, direction: Vec2) -> Ray {
        Ray { origin, direction }
    }
    pub fn new_target(origin: Pos2, target: Pos2) -> Ray {
        Ray {
            origin,
            direction: target - origin,
        }
    }

    pub fn draw_line(&self, transform: &crate::util::DrawTransform) {
        let origin_transform = transform.transform(self.origin);
        let direction_transform = transform.transform(self.direction);

        let arrow_end = origin_transform + direction_transform;

        draw_line(
            origin_transform.x,
            origin_transform.y,
            arrow_end.x,
            arrow_end.y,
            DEVLINE_THICKNESS,
            FG,
        );
    }
}

impl Neg for Ray {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            origin: self.origin,
            direction: -self.direction,
        }
    }
}

const ARROWHEAD_LENGTH: f32 = 30.;
const ARROWHEAD_HALFWIDTH: f32 = 12.;
impl Drawable for Ray {
    fn draw(&self, transform: &crate::util::DrawTransform) {
        let origin_transform = transform.transform(self.origin);
        let direction_transform = transform.transform(self.direction);

        let normal = direction_transform.normalized();
        let ortho = normal.orthogonal();
        let arrow_end = origin_transform + direction_transform;
        let arrow_tip_0 = arrow_end - normal * ARROWHEAD_LENGTH - ortho * ARROWHEAD_HALFWIDTH;
        let arrow_tip_1 = arrow_end - normal * ARROWHEAD_LENGTH + ortho * ARROWHEAD_HALFWIDTH;

        draw_line(
            origin_transform.x,
            origin_transform.y,
            arrow_end.x,
            arrow_end.y,
            DEVLINE_THICKNESS,
            FG,
        );

        draw_line(
            arrow_end.x,
            arrow_end.y,
            arrow_tip_0.x,
            arrow_tip_0.y,
            DEVLINE_THICKNESS,
            FG,
        );
        draw_line(
            arrow_end.x,
            arrow_end.y,
            arrow_tip_1.x,
            arrow_tip_1.y,
            DEVLINE_THICKNESS,
            FG,
        );
    }
}

impl Transform<Ray> for DrawTransform {
    type Output = Ray;

    fn transform(&self, item: Ray) -> Ray {
        Ray {
            origin: self.transform(item.origin),
            direction: self.transform(item.direction),
        }
    }
}
