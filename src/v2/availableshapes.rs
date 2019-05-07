use std::fmt;
use std::fmt::Display;

use crate::algebra::intersection::{Intersection, IntersectionList};
use crate::algebra::optics::Optics;
use crate::algebra::optics::OpticsOps;
use crate::algebra::vector::Vector;
use crate::algebra::vector::VectorOps;
use crate::v2::raytracemath;
use crate::v2::raytracemath::RayTraceMath;
use crate::v2::solidobject::SolidObjectOps;
use crate::v2::sphere::Sphere;
use crate::v2::torus::Torus;

pub const EPSILON: f32 = 1.0e-6;

pub enum AvailableShapes {
    Sphere(Sphere),
    Torus(Torus),
}

impl SolidObjectOps for AvailableShapes {
    fn rotate_x(&mut self, angle_in_degrees: f32) {
        match *self {
            AvailableShapes::Sphere(ref mut sphere) => sphere.rotate_x(angle_in_degrees),
            AvailableShapes::Torus(ref mut torus) => torus.rotate_x(angle_in_degrees),
        }
    }

    fn get_center(&mut self) -> &mut Vector {
        match *self {
            AvailableShapes::Sphere(ref mut sphere) => &mut sphere.solid_object.center,
            AvailableShapes::Torus(ref mut torus) => &mut torus.solid_object.center,
        }
    }
}

impl AvailableShapes {
    pub fn translate(&mut self, dx: f32, dy: f32, dz: f32) {
        RayTraceMath::translate(&mut self.get_center(), dx, dy, dz);
    }
}


#[test]
fn test_stuff() {
    // TODO??? is there somethint to test?
}
