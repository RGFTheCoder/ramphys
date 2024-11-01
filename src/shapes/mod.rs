use circle::Circle;
use polygon::Polygon;

use crate::math::{
    collision_manifold::CollisionManifold,
    vec2::{Pos2, Vec2},
};

pub mod circle;
pub mod polygon;
// pub mod rectangle;

pub trait Shape {
    fn displace(&mut self, delta: Vec2);

    fn rotate(&mut self, theta: f32);

    fn center(&self) -> Pos2;
}

pub trait Collision<T = Self> {
    fn collides(&self, other: &T) -> Option<CollisionManifold>;
}

pub enum ShapeVariant {
    Circle(Circle),
    Polygon(Polygon),
}

impl Shape for ShapeVariant {
    fn displace(&mut self, delta: Vec2) {
        match self {
            ShapeVariant::Circle(circle) => circle.displace(delta),
            ShapeVariant::Polygon(polygon) => polygon.displace(delta),
        }
    }

    fn rotate(&mut self, theta: f32) {
        match self {
            ShapeVariant::Circle(circle) => circle.rotate(theta),
            ShapeVariant::Polygon(polygon) => polygon.rotate(theta),
        }
    }

    fn center(&self) -> Pos2 {
        match self {
            ShapeVariant::Circle(circle) => circle.center(),
            ShapeVariant::Polygon(polygon) => polygon.center(),
        }
    }
}
