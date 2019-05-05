use crate::algebra::color::{Color, ColorOps};
use crate::scene::scene::Scene;

mod algebra;
mod utils;
mod geometry;
mod scene;

fn main() {
    let s = Scene::new(640, 480, Color::new(0.5, 0.5, 0.5));

    println!("scene = {}", s);
}
