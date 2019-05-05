use std::fmt;
use std::fmt::Display;

use crate::algebra::vector::Vector;
use crate::algebra::vector::VectorOps;

pub struct Intersection {
    pub distance_squared: f32,
    pub point: Vector,
    pub surface_normal: Vector,
    pub solid: usize,
}

pub type IntersectionList = Vec<Intersection>;

fn pick_closest_intersection(list: &IntersectionList, intersection: &Intersection) -> i32 {
    unimplemented!();
}

impl Intersection {
    fn new() -> Intersection {
        Intersection {
            distance_squared: 1.0e20,
            point: Vector::new(0.0, 0.0, 0.0),
            surface_normal: Vector::new(0.0, 0.0, 0.0),
            solid: 99999,
        }
    }
}

impl Display for Intersection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "distance_squared {}, point: {}, surface_normal: {}, solid idx: {}", self.distance_squared, self.point, self.surface_normal, self.solid)
    }
}

#[test]
fn test_stuff() {
    // TODO  use assertions :-)
}