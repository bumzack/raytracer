use crate::algebra::vector::{Vector, VectorOps};

mod algebra;

fn main() {
    let v = Vector { x: 1.0, y: 2.0, z: 2.0 };

    println!("v = {}", v);
    println!("magnotude = {}", v.magnitude());
    println!("unit vector = {}", v.unit_vector());
}
