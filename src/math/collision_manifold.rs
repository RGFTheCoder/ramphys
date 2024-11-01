use super::{ray::Ray, vec2::Vec2};

pub struct CollisionManifold {
    pub normal: Vec2,
    // pub penetration_point: Pos2,
    pub penetration: Ray,
    pub depth: f32,
}
