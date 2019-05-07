use crate::algebra::vector::Vector;

pub const EPSILON: f32 = 1.0e-6;

pub struct RayTraceMath {}

impl RayTraceMath {
    pub fn translate(center: &mut Vector, dx: f32, dy: f32, dz: f32) {
        center.x += dx;
        center.y += dy;
        center.z += dz;
    }
}


#[test]
fn test_stuff() {
    // TODO??? is there somethint to test?
}
