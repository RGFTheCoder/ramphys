pub mod math;
pub mod simulation;
pub mod util;

use macroquad::{
    time::get_frame_time,
    window::{clear_background, next_frame},
};
// use macroquad::prelude::*;
use math::{ray::Ray, vec2::Pos2};
use simulation::Simulation;
use util::{DrawTransform, Drawable, BG};

// Since I intend to separate the physics code from the renderer, I will use my own classes for Vectors, but colors can stay with the Macroquad API.

#[macroquad::main("2d Physics")]
async fn main() {
    let mut sim = Simulation::new();

    let p = Ray::new_target(Pos2::at(10., 10.), Pos2::at(20., 15.));
    let t = DrawTransform {
        x: 0.,
        y: 0.,
        zoom: 20.,
    };

    loop {
        sim.update(get_frame_time());

        clear_background(BG);

        // let m = mouse_position();

        sim.draw(&t);

        p.draw(&t);

        next_frame().await
    }
}
