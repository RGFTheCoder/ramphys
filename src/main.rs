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
use shapes::{circle::Circle, polygon::Polygon, Collision, Shape};
use simulation::Simulation;
use util::{DrawTransform, Drawable, BG, RED};

// Since I intend to separate the physics code from the renderer, I will use my own classes for Vectors, but colors can stay with the Macroquad API.

#[macroquad::main("2d Physics")]
async fn main() {
    let mut sim = Simulation::new();

    let mut p2 = Circle {
        position: Pos2::at(10., 10.),
        radius: 5.,
    };
    let mut p1 = Polygon::from_points(vec![
        Vec2::with(15., 15.),
        Vec2::with(20., 15.),
        Vec2::with(25., 20.),
        Vec2::with(15., 20.),
    ]);

    let t = DrawTransform {
        x: 0.,
        y: 0.,
        zoom: 5.,
    };

    loop {
        sim.update(get_frame_time());
        if is_key_down(macroquad::input::KeyCode::W) {
            p1.position.y -= 5. * get_frame_time();
        }
        if is_key_down(macroquad::input::KeyCode::S) {
            p1.position.y += 5. * get_frame_time();
        }
        if is_key_down(macroquad::input::KeyCode::A) {
            p1.position.x -= 5. * get_frame_time();
        }
        if is_key_down(macroquad::input::KeyCode::D) {
            p1.position.x += 5. * get_frame_time();
        }
        if is_key_down(macroquad::input::KeyCode::Q) {
            p1.rotate(-get_frame_time());
        }
        if is_key_down(macroquad::input::KeyCode::E) {
            p1.rotate(get_frame_time());
        }

        clear_background(BG);

        // let m = mouse_position();

        sim.draw(&t);

        p1.draw(&t);
        p2.draw(&t);

        // if let Some(col) = p1.collides(&p2) {
        //     p2.position += col.penetration.direction;
        //     col.penetration.draw(&t);
        //     col.penetration.origin.draw(&t);
        //     println!("{:?}", col.penetration.origin)
        // }

        next_frame().await
    }
}
