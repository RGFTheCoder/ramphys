pub mod math;
pub mod shapes;
pub mod simulation;
pub mod util;

use macroquad::{
    input::is_key_down,
    time::get_frame_time,
    window::{clear_background, next_frame},
};
// use macroquad::prelude::*;
use math::vec2::{Pos2, Vec2};
use shapes::{circle::Circle, polygon::Polygon, Collision, Shape, ShapeVariant};
use simulation::Simulation;
use util::{DrawTransform, Drawable, BG, RED};

// Since I intend to separate the physics code from the renderer, I will use my own classes for Vectors, but colors can stay with the Macroquad API.

#[macroquad::main("2d Physics")]
async fn main() {
    let mut sim = Simulation::new();

    sim.items
        .push(ShapeVariant::Polygon(Polygon::from_points(vec![
            Vec2::with(25., 25.),
            Vec2::with(30., 25.),
            Vec2::with(35., 30.),
            Vec2::with(25., 30.),
        ])));
    sim.items
        .push(ShapeVariant::Polygon(Polygon::from_points(vec![
            Vec2::with(15., 15.),
            Vec2::with(20., 15.),
            Vec2::with(25., 20.),
            Vec2::with(15., 20.),
        ])));

    sim.items.push(ShapeVariant::Circle(Circle {
        position: Pos2::at(10., 10.),
        radius: 5.,
    }));

    let t = DrawTransform {
        x: 0.,
        y: 0.,
        zoom: 25.,
    };

    loop {
        clear_background(BG);
        sim.update(get_frame_time());

        sim.draw(&t);

        next_frame().await
    }
}
