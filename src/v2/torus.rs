use std::fmt;
use std::fmt::Display;

use crate::algebra::intersection::{Intersection, IntersectionList};
use crate::algebra::optics::Optics;
use crate::algebra::optics::OpticsOps;
use crate::algebra::vector::Vector;
use crate::algebra::vector::VectorOps;
use crate::v2::solidobject::{SolidObject, SolidObjectOps};

pub struct Torus {
    pub solid_object: SolidObject,
    pub radius1: f32,
    pub radius2: f32,
}

impl Torus {
    pub fn new() -> Torus {
        Torus {
            solid_object: SolidObject::new(),
            radius1: 1.0,
            radius2: 2.0,
        }
    }
}

impl SolidObjectOps for Torus {
    fn rotate_x(&mut self, angle_in_degrees: f32) {
        println!("inside Torus::rotate_x()")
    }
    fn get_center(&mut self) -> &mut Vector {
        &mut self.solid_object.center
    }
}

impl Display for Torus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO  Display Torus")
    }
}

#[test]
fn test_stuff() {
    // TODO??? is there somethint to test?
}
