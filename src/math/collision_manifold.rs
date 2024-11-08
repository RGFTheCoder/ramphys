use std::ops::Neg;

use super::{ray::Ray, vec2::Vec2};

pub struct CollisionManifold {
    pub normal: Vec2,
    // pub penetration_point: Pos2,
    pub penetration: Ray,
    pub depth: f32,
}

impl Neg for CollisionManifold {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            normal: -self.normal,
            penetration: -self.penetration,
            depth: self.depth,
        }
    }
}
