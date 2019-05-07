use std::fmt;
use std::fmt::Display;

use crate::algebra::color::Color;
use crate::algebra::intersection::{Intersection, IntersectionList};
use crate::algebra::optics::Optics;
use crate::algebra::optics::OpticsOps;
use crate::algebra::vector::Vector;
use crate::algebra::vector::VectorOps;

pub const EPSILON: f32 = 1.0e-6;

pub struct SolidObject {
    pub center: Vector,
    pub uniform_optics: Optics,
    pub refractive_index: f32,
    pub is_fully_enclose: bool,
    pub cached_intersection_list: IntersectionList,
    pub enclosure_list: IntersectionList,
}

impl SolidObject {
    pub fn new() -> SolidObject {
        SolidObject {
            center: Vector::default(),
            uniform_optics: Optics::default(),
            refractive_index: 1.0,
            is_fully_enclose: true,
            cached_intersection_list: Vec::new(),
            enclosure_list: Vec::new(),
        }
    }
}

pub trait SolidObjectOps {
    fn rotate_x(&mut self, angle_in_degrees: f32);
}


impl SolidObjectOps for Sphere {
    fn rotate_x(&mut self, angle_in_degrees: f32) {
        println!("inside Sphere::rotate_x()")
    }
}

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


#[derive(Debug)]
pub enum AvailableShapes {
    Sphere(Sphere),
    Torus(Torus),
}

impl SolidObjectOps for AvailableShapes {
    fn rotate_x(&mut self, angle_in_degrees: f32) {
        match *self {
            AvailableShapes::Sphere(ref sphere) => sphere.rotate_x(angle_in_degrees),
            AvailableShapes::Torus(ref torus) => torus.rotate_x(angle_in_degrees),
        }
    }
}


impl Display for SolidObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO  Display SolidObject")
    }
}

#[test]
fn test_stuff() {
    // TODO??? is there somethint to test?
}
