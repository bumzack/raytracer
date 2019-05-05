use std::fmt;
use std::fmt::Display;

use crate::algebra::color::Color;
use crate::algebra::intersection::{Intersection, IntersectionList};
use crate::algebra::optics::Optics;
use crate::algebra::optics::OpticsOps;
use crate::algebra::vector::Vector;
use crate::algebra::vector::VectorOps;

pub const EPSILON: f32 = 1.0e-6;


pub trait SolidObjectOps {
    fn append_all_intersections(&self, vantage: &Vector, direction: &Vector, mut intersection_list: &IntersectionList);
    fn find_closest_intersection(&mut self, vantage: &Vector, direction: &Vector, mut intersection: &Intersection);
    fn contains(self, point: Vector) -> bool;
    fn surface_optics(&self, surface_point: &Vector) -> &Optics;
    fn get_refractive_index(self) -> f32;
    fn rotate_x(&mut self, angle_in_degrees: f32);
    fn rotate_y(&mut self, angle_in_degrees: f32);
    fn rotate_z(&mut self, angle_in_degrees: f32);
    fn translate(&mut self, dx: f32, dy: f32, dz: f32);

    fn move_by_delta(&mut self, cx: f32, cy: f32, cz: f32);
    fn move_to_center(&mut self, center: Vector);

    fn get_center(self) -> Vector;
    fn set_uniform_optics(&mut self, o: Optics);
    fn set_matte_gloss_balance(&mut self, gloss_factor: f32, raw_matte_color: Color, rwa_gloss_color: Color);
    fn set_full_mate(&mut self, c: Color);
    fn set_opacity(&mut self, o: f32);
    fn set_refraction(&mut self, r: f32);
}

pub fn pick_closest_intersection(list: &IntersectionList, intersection: &Intersection) -> i32 {
    unimplemented!();
}

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

//
//impl SolidObjectOps for AbstractSolidObject {
//    fn append_all_intersections(&self, vantage: &Vector, direction: &Vector, mut intersection_list: &_) {
//        unimplemented!()
//    }
//
//    fn contains(&self, point: Vector) -> bool {
//        unimplemented!()
//    }
//
//    fn rotate_x(&mut self, angle_in_degrees: f32) {
//        unimplemented!()
//    }
//
//    fn rotate_y(&mut self, angle_in_degrees: f32) {
//        unimplemented!()
//    }
//
//    fn rotate_z(&mut self, angle_in_degrees: f32) {
//        unimplemented!()
//    }
//
//    fn get_refractive_index(self) -> f32 {
//        self.refractive_index
//    }
//
//    fn surface_optics(&self, surface_point: &Vector) -> &Optics {
//        &self.uniform_optics
//    }
//
//    fn find_closest_intersection(&self, vantage: &Vector, direction: &Vector, mut intersection: &Intersection) {
//        cached_intersection_list.clear();
//        append_all_intersections(vantage, direction, cachedIntersectionList);
//        pick_closest_intersection(cachedIntersectionList, intersection);
//    }
//
//    fn translate(&mut self, dx: f32, dy: f32, dz: f32) {
//        *self.center.x += dx;
//        *self.center.y += dy;
//        *self.center.z += dz;
//    }
//
//    fn move_by_delta(&mut self, cx: f32, cy: f32, cz: f32) {
//        self.translate(cx - center.x, cy - center.y, cz - center.z);
//    }
//
//    fn move_to_center(&mut self, center: Vector) {
//        self.center = center;
//    }
//
//    fn get_center(self) -> &Vector {
//        *self.center
//    }
//
//    fn set_uniform_optics(&mut self, o: Optics) {
//        self.uniform_optics = o;
//    }
//
//    fn set_matte_gloss_balance(&mut self, gloss_factor: f32, raw_matte_color: Color, raw_gloss_color: Color) {
//        self.uniform_optics.set_matte_gloss_balance(gloss_factor, raw_matte_color, raw_gloss_color);
//    }
//
//    fn set_full_mate(&mut self, c: Color) {
//        self.uniform_optics.set_matte_gloss_balance(
//            0.0,        // glossFactor=0 indicates full matte reflection
//            matteColor,
//            Color::new(0.0, 0.0, 0.0));
//    }
//
//    fn set_opacity(&mut self, o: f32) {
//        self.uniform_optics.set_opactiy(o);
//    }
//
//    fn set_refraction(&mut self, r: f32) {
//        // TODO check result and do stuff ...
//        validate_refraction(r);
//        self.refractive_index = r;
//    }
//}

impl Display for SolidObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TODO  Display SolidObject")
    }
}

#[test]
fn test_stuff() {
    // TODO??? is there somethint to test?
}
