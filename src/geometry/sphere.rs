use std::fmt;
use std::fmt::Display;

use crate::algebra::color::Color;
use crate::algebra::color::ColorOps;
use crate::algebra::intersection::Intersection;
use crate::algebra::intersection::IntersectionList;
use crate::algebra::optics::Optics;
use crate::algebra::optics::OpticsOps;
use crate::algebra::refraction::validate_refraction;
use crate::algebra::solidobject::*;
use crate::algebra::solidobject::pick_closest_intersection;
use crate::algebra::vector::Vector;
use crate::algebra::vector::VectorOps;

pub struct Sphere {
    pub solid_object: SolidObject,
    pub radius: f32,
}

impl SolidObjectOps for Sphere {
    fn append_all_intersections(&self, vantage: &Vector, direction: &Vector, mut intersection_list: &IntersectionList) {
        unimplemented!()
    }

    fn contains(self, point: Vector) -> bool {
        let r = self.radius + EPSILON;
        let t = point - self.get_center();
        let m = t.magnitude_squared();
        m <= (r * r)
    }

    fn rotate_x(&mut self, angle_in_degrees: f32) {
        // do nothing for a sphere
    }

    fn rotate_y(&mut self, angle_in_degrees: f32) {
        // do nothing for a sphere
    }

    fn rotate_z(&mut self, angle_in_degrees: f32) {
        // do nothing for a sphere
    }

    fn get_refractive_index(self) -> f32 {
        self.solid_object.refractive_index
    }

    fn surface_optics(&self, surface_point: &Vector) -> &Optics {
        &self.solid_object.uniform_optics
    }

    fn find_closest_intersection(&mut self, vantage: &Vector, direction: &Vector, mut intersection: &Intersection) {
        &self.solid_object.cached_intersection_list.clear();
        &self.append_all_intersections(vantage, direction, &self.solid_object.cached_intersection_list);
        pick_closest_intersection(&self.solid_object.cached_intersection_list, intersection);
    }

    fn translate(&mut self, dx: f32, dy: f32, dz: f32) {
        self.solid_object.center.x += dx;
        self.solid_object.center.y += dy;
        self.solid_object.center.z += dz;
    }

    fn move_by_delta(&mut self, cx: f32, cy: f32, cz: f32) {
        self.translate(cx - self.solid_object.center.x, cy - self.solid_object.center.y, cz - self.solid_object.center.z);
    }

    fn move_to_center(&mut self, center: Vector) {
        self.solid_object.center = center;
    }

    fn get_center(self) -> Vector {
        // TODO: can this be done without clone?
        self.solid_object.center
    }

    fn set_uniform_optics(&mut self, o: Optics) {
        self.solid_object.uniform_optics = o;
    }

    fn set_matte_gloss_balance(&mut self, gloss_factor: f32, raw_matte_color: Color, raw_gloss_color: Color) {
        self.solid_object.uniform_optics.set_matte_gloss_balance(gloss_factor, raw_matte_color, raw_gloss_color);
    }

    fn set_full_mate(&mut self, c: Color) {
        self.solid_object.uniform_optics.set_matte_gloss_balance(
            0.0,        // glossFactor=0 indicates full matte reflection
            c,
            Color::new(0.0, 0.0, 0.0));
    }

    fn set_opacity(&mut self, o: f32) {
        self.solid_object.uniform_optics.set_opactiy(o);
    }

    fn set_refraction(&mut self, r: f32) {
        // TODO check result and do stuff ...
        validate_refraction(r).expect("crap - refraction invalid");
        self.solid_object.refractive_index = r;
    }
}

impl Sphere {
    fn new() -> Sphere {
        Sphere {
            radius: 1.0,
            solid_object: SolidObject::new(),
        }
    }
}

impl Display for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "radius = {}, solid_object = {}", self.radius, self.solid_object)
    }
}

#[test]
fn test_stuff() {
    // TODO  use assertions :-)
}
