pub trait Drawable {
    fn draw(&self, transform: &DrawTransform);
}

pub struct DrawTransform {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

impl DrawTransform {
    pub fn transform_x(&self, x: f32) -> f32 {
        (x - self.x) * self.zoom
    }
    pub fn transform_y(&self, y: f32) -> f32 {
        (y - self.y) * self.zoom
    }
}
