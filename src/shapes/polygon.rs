use std::iter::once;

use macroquad::shapes::draw_line;

use crate::{
    math::{
        ray::Ray,
        vec2::{Pos2, Vec2},
    },
    util::{Drawable, Transform, DEVLINE_THICKNESS, GREEN},
};

use super::Shape;

#[derive(Clone, Debug)]
pub struct Polygon {
    pub position: Pos2,
    pub points: Vec<Vec2>,
    pub normals: Vec<Vec2>,
    pub theta: f32,
}

impl Polygon {
    pub fn from_points(mut points: Vec<Vec2>) -> Polygon {
        let center = centroid(&points);
        let center_offset = center.from_origin();

        points.iter_mut().for_each(|x| {
            *x -= center_offset;
        });

        Polygon {
            position: center,
            normals: normals(&points),
            points,
            theta: 0.,
        }
    }

    pub fn rectangle(center: Pos2, width: f32, height: f32) -> Polygon {
        Polygon {
            position: center,
            points: vec![
                Vec2::with(-width * 0.5, -height * 0.5),
                Vec2::with(width * 0.5, -height * 0.5),
                Vec2::with(width * 0.5, height * 0.5),
                Vec2::with(-width * 0.5, height * 0.5),
            ],
            normals: vec![
                Vec2::with(0., -1.),
                Vec2::with(1., 0.),
                Vec2::with(0., 1.),
                Vec2::with(-1., 0.),
            ],
            theta: 0.,
        }
    }

    pub fn get_world_points<'a>(&'a self) -> impl Iterator<Item = Pos2> + 'a {
        self.points
            .iter()
            .map(|x| self.center() + x.rotate(self.theta))
    }
    pub fn get_world_normals<'a>(&'a self) -> impl Iterator<Item = Vec2> + 'a {
        self.normals.iter().map(|x| x.rotate(self.theta))
    }
    pub fn get_world_points_cycled<'a>(&'a self) -> impl Iterator<Item = Pos2> + 'a {
        self.points
            .iter()
            .map(|x| self.position + x.rotate(self.theta))
            .chain(once(self.position + self.points[0].rotate(self.theta)))
    }
    pub fn get_world_normals_cycled<'a>(&'a self) -> impl Iterator<Item = Vec2> + 'a {
        self.normals
            .iter()
            .map(|x| x.rotate(self.theta))
            .chain(once(self.normals[0].rotate(self.theta)))
    }
}

impl Shape for Polygon {
    fn displace(&mut self, delta: Vec2) {
        self.position += delta;
    }

    fn rotate(&mut self, theta: f32) {
        self.theta += theta;
    }

    fn center(&self) -> Pos2 {
        self.position
    }
}

impl Drawable for Polygon {
    fn draw(&self, transform: &crate::util::DrawTransform) {
        let points: Vec<Pos2> = self
            .get_world_points_cycled()
            .map(|x| transform.transform(x))
            .collect();

        for p in points.windows(2) {
            draw_line(p[0].x, p[0].y, p[1].x, p[1].y, DEVLINE_THICKNESS, GREEN)
        }

        let real_points = self.get_world_points_cycled().collect::<Vec<Pos2>>();
        let normals = self.get_world_normals().zip(real_points.windows(2));

        for (normal, p) in normals {
            let origin = p[0].midpoint(p[1]);
            (Ray {
                origin,
                direction: normal * 5.,
            })
            .draw(transform);
        }

        // Origin
        // self.position.draw(transform);
    }
}

pub fn area(points: &Vec<Vec2>) -> f32 {
    let mut area = 0.;

    for i in 0..points.len() {
        let next = (i + 1).rem_euclid(points.len());
        area += points[i].cross(points[next]);
    }

    area * 0.5
}

pub fn centroid(points: &Vec<Vec2>) -> Pos2 {
    let area = area(points);
    let cent_mult = 1. / (6. * area);

    let mut centroid = Pos2 { x: 0., y: 0. };

    for i in 0..points.len() {
        let next = (i + 1).rem_euclid(points.len());

        centroid += (points[i] + points[next]) * points[i].cross(points[next]);
    }

    centroid.x *= cent_mult;
    centroid.y *= cent_mult;

    centroid
}

fn normals(points: &Vec<Vec2>) -> Vec<Vec2> {
    let mut normals = Vec::with_capacity(points.len());

    for i in 0..points.len() {
        let next = (i + 1).rem_euclid(points.len());
        let current = points[i];
        let next = points[next];

        normals.push((next - current).normalized().orthogonal());
    }

    normals
}
