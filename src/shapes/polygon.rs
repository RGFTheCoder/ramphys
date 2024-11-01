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
        }
    }
}

impl Shape for Polygon {
    fn displace(&mut self, delta: Vec2) {
        self.position += delta;
    }

    fn rotate(&mut self, theta: f32) {
        self.points.iter_mut().for_each(|x| {
            *x = x.rotate(theta);
        });
        self.normals.iter_mut().for_each(|x| {
            *x = x.rotate(theta);
        });
    }

    fn center(&self) -> Pos2 {
        self.position
    }
}

impl Drawable for Polygon {
    fn draw(&self, transform: &crate::util::DrawTransform) {
        let t_center = transform.transform(self.position);

        for i in 0..self.points.len() {
            let next = (i + 1).rem_euclid(self.points.len());
            // self.points.windows(2).for_each(|x| {
            let p1_trans = t_center + transform.transform(self.points[i]);
            let p2_trans = t_center + transform.transform(self.points[next]);
            draw_line(
                p1_trans.x,
                p1_trans.y,
                p2_trans.x,
                p2_trans.y,
                DEVLINE_THICKNESS,
                GREEN,
            );

            // Normals
            let origin = self.position + (self.points[i] + self.points[next]) * 0.5;
            (Ray {
                origin,
                direction: self.normals[i] * 5.,
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
