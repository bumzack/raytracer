use crate::algebra::color::{Color, ColorOps};
use crate::algebra::vector::{Vector, VectorOps};

mod algebra;
mod utils;

fn main() {
    let c = Color::new(1.0, 2.0, 2.0);
    let c1 = Color::new(1.0, 2.0, 3.0);

    println!("{}", c);
    println!("sum of a + b = {}", c + c1);

    let c = Color::new(1.0, 2.0, 2.0);
    println!("v*2.5 = {}", c * 2.5);

    let c = Color::new(1.0, 2.0, 2.0);
    println!("2.5 * v = {}", 2.5 * c);

    let c = Color::new(0.0, -2.0, 2.0);

    match c.validate() {
        Err(e) => {
            println!("Error: {}", e.to_string());
        }
        Ok(()) => print!("valid color")
    }
}
