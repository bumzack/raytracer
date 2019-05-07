use std::fmt;
use std::fmt::Display;

use crate::algebra::intersection::{Intersection, IntersectionList};
use crate::algebra::optics::Optics;
use crate::algebra::optics::OpticsOps;
use crate::algebra::vector::Vector;
use crate::algebra::vector::VectorOps;
use crate::v2::solidobject::{SolidObject, SolidObjectOps};

pub struct Sphere {
    pub solid_object: SolidObject,
    pub radius: f32,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            solid_object: SolidObject::new(),
            radius: 1.0,
        }
    }
}

impl SolidObjectOps for Sphere {
    fn rotate_x(&mut self, angle_in_degrees: f32) {
        println!("inside Sphere::rotate_x()")
    }
    fn get_center(&mut self) -> &mut Vector {
        &mut self.solid_object.center
    }
}

impl Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO  Display sphere")
    }
}

#[test]
fn test_stuff() {
    // TODO??? is there somethint to test?
}
