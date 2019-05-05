use std::fmt;
use std::fmt::Display;

use crate::algebra::intersection::IntersectionList;
use crate::algebra::solidobject::SolidObjectOps;

const MAX_OPTICAL_RECURSION_DEPTH: i32 = 20;
const MIN_OPTICAL_INTENSITY: f32 = 0.001;


pub struct LightSource {}

pub trait LightSourceOps {}

impl LightSourceOps for LightSource {}

impl Display for LightSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: add all fields
        write!(f, "LightSource")
    }
}

#[test]
fn test_stuff() {
    // TODO  use assertions :-)
}