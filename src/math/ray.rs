use macroquad::{color::BLUE, shapes::draw_line};

use crate::util::Drawable;

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
}

const ARROWHEAD_LENGTH: f32 = 10.;
const ARROWHEAD_HALFWIDTH: f32 = 4.;
const ARROWHEAD_THICKNESS: f32 = 4.;
impl Drawable for Ray {
    fn draw(&self, transform: &crate::util::DrawTransform) {
        let normal = self.direction.normalized();
        let ortho = normal.orthogonal();
        let arrow_end = self.origin + self.direction;
        let arrow_tip_0 = arrow_end - normal * ARROWHEAD_LENGTH - ortho * ARROWHEAD_HALFWIDTH;
        let arrow_tip_1 = arrow_end - normal * ARROWHEAD_LENGTH + ortho * ARROWHEAD_HALFWIDTH;

        draw_line(
            self.origin.x,
            self.origin.y,
            arrow_end.x,
            arrow_end.y,
            ARROWHEAD_THICKNESS,
            BLUE,
        );

        draw_line(
            arrow_end.x,
            arrow_end.y,
            arrow_tip_0.x,
            arrow_tip_0.y,
            ARROWHEAD_THICKNESS,
            BLUE,
        );
        draw_line(
            arrow_end.x,
            arrow_end.y,
            arrow_tip_1.x,
            arrow_tip_1.y,
            ARROWHEAD_THICKNESS,
            BLUE,
        );
    }
}
