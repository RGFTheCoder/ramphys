use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use macroquad::shapes::draw_circle;

use crate::util::{DrawTransform, Drawable, Transform, RED};

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Pos2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn length_squared(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2)
    }
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn normalized(&self) -> Self {
        let inv_length = self.length().recip();
        Self {
            x: self.x * inv_length,
            y: self.y * inv_length,
        }
    }
    pub fn normalize(&mut self) {
        let inv_length = self.length().recip();
        self.x *= inv_length;
        self.y *= inv_length;
    }
    pub fn orthogonal(&self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, rhs: &Self) -> f32 {
        self.x * rhs.y - self.y * rhs.x
    }

    pub fn with(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Pos2 {
    pub fn at(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Vec2> for Pos2 {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub for Pos2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<Vec2> for Pos2 {
    type Output = Self;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Self::Output {
        let inv_rhs = rhs.recip();
        Self {
            x: self.x * inv_rhs,
            y: self.y * inv_rhs,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
    }
}
impl AddAssign<Vec2> for Pos2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
    }
}
impl SubAssign<Vec2> for Pos2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x.mul_assign(rhs);
        self.y.mul_assign(rhs);
    }
}
impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        let inv_rhs = rhs.recip();
        self.x.mul_assign(inv_rhs);
        self.y.mul_assign(inv_rhs);
    }
}

impl Drawable for Pos2 {
    fn draw(&self, transform: &crate::util::DrawTransform) {
        let transformed = transform.transform(*self);

        draw_circle(transformed.x, transformed.y, 5., RED);
    }
}

impl Transform<Vec2> for DrawTransform {
    type Output = Vec2;

    fn transform(&self, item: Vec2) -> Vec2 {
        Vec2 {
            x: item.x * self.zoom,
            y: item.y * self.zoom,
        }
    }
}

impl Transform<Pos2> for DrawTransform {
    type Output = Pos2;

    fn transform(&self, item: Pos2) -> Self::Output {
        Pos2 {
            x: item.x * self.zoom,
            y: item.y * self.zoom,
        }
    }
}
