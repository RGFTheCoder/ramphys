use circle::Circle;
use polygon::Polygon;

use crate::{
    math::{
        collision_manifold::CollisionManifold,
        ray::Ray,
        vec2::{Pos2, Vec2},
    },
    simulation::Simulation,
    util::Drawable,
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

impl Drawable for ShapeVariant {
    fn draw(&self, transform: &crate::util::DrawTransform) {
        match self {
            ShapeVariant::Circle(circle) => circle.draw(transform),
            ShapeVariant::Polygon(polygon) => polygon.draw(transform),
        }
    }
}

impl Collision for ShapeVariant {
    fn collides(&self, other: &Self) -> Option<CollisionManifold> {
        match (self, other) {
            (ShapeVariant::Circle(c1), ShapeVariant::Circle(c2)) => c1.collides(c2),
            (ShapeVariant::Polygon(p1), ShapeVariant::Polygon(p2)) => p1.collides(p2),
            (ShapeVariant::Circle(c1), ShapeVariant::Polygon(p2)) => c1.collides(p2),
            (ShapeVariant::Polygon(p1), ShapeVariant::Circle(c2)) => p1.collides(c2),
        }
    }
}

impl Collision<Circle> for Polygon {
    fn collides(&self, other: &Circle) -> Option<CollisionManifold> {
        circle_vs_edges(other, &self).or_else(|| circle_vs_points(other, self.get_world_points()))
    }
}

impl Collision<Polygon> for Circle {
    fn collides(&self, other: &Polygon) -> Option<CollisionManifold> {
        let col = other.collides(self)?;

        Some(-col)
    }
}

fn circle_vs_edges(circle: &Circle, poly: &Polygon) -> Option<CollisionManifold> {
    let real_points = poly.get_world_points_cycled().collect::<Vec<Pos2>>();
    let normals = poly.get_world_normals().zip(real_points.windows(2));

    let mut nearest = None;

    for (normal, p) in normals {
        let vert_to_circ = circle.position - p[0];
        let vert_to_next = p[1] - p[0];
        let vert_to_next_len = vert_to_next.length();
        let vert_to_next = vert_to_next.normalized();

        let circ_next_proj = vert_to_circ.dot(vert_to_next);
        let circ_norm_proj = vert_to_circ.dot(normal);
        if circ_next_proj > 0. && circ_next_proj < vert_to_next_len && circ_norm_proj >= 0. {
            nearest = Some(Ray {
                origin: p[0],
                direction: normal,
            })
        }
    }

    let nearest = nearest?;

    let vert_to_circ = circle.position - nearest.origin;
    let proj_to_norm = nearest.direction.dot(vert_to_circ);
    if proj_to_norm - circle.radius < 0. {
        let depth = circle.radius - proj_to_norm;
        Some(CollisionManifold {
            normal: nearest.direction,
            penetration: Ray {
                origin: circle.position + nearest.direction * circle.radius * -1.,
                direction: nearest.direction * depth,
            },
            depth,
        })
    } else {
        None
    }
}
fn circle_vs_points(
    circle: &Circle,
    poly: impl Iterator<Item = Pos2>,
) -> Option<CollisionManifold> {
    for v in poly {
        let dir = v - circle.position;

        if dir.length_squared() < circle.radius * circle.radius {
            let pen_depth = circle.radius - dir.length();
            let normal = -dir.normalized();
            return Some(CollisionManifold {
                normal,
                penetration: Ray {
                    origin: v,
                    direction: normal * pen_depth,
                },
                depth: pen_depth,
            });
        }
    }

    None
}
