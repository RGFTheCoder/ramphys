use macroquad::color::Color;

pub trait Drawable {
    fn draw(&self, transform: &DrawTransform);
}

pub trait Transform<T> {
    type Output;

    fn transform(&self, item: T) -> Self::Output;
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

pub const RED: Color = Color::new(
    0.8235294117647058,
    0.058823529411764705,
    0.2235294117647059,
    1.,
);
pub const GREEN: Color = Color::new(
    0.25098039215686274,
    0.6274509803921569,
    0.16862745098039217,
    1.0,
);
pub const BLUE: Color = Color::new(0.11764705882352941, 0.4, 0.9607843137254902, 1.0);
pub const YELLOW: Color = Color::new(
    0.8745098039215686,
    0.5568627450980392,
    0.11372549019607843,
    1.0,
);
pub const BG: Color = Color::new(
    0.9372549019607843,
    0.9450980392156862,
    0.9607843137254902,
    1.0,
);
pub const FG: Color = Color::new(
    0.2980392156862745,
    0.30980392156862746,
    0.4117647058823529,
    1.0,
);
