use crate::v2::availableshapes::AvailableShapes;
use crate::v2::solidobject::SolidObjectOps;
use crate::v2::sphere::Sphere;
use crate::v2::torus::Torus;

mod v2;
mod algebra;
mod utils;

fn main() {
    let sphere = Sphere::new();
    let torus = Torus::new();

    let mut shapes: Vec<AvailableShapes> = Vec::new();

    shapes.push(AvailableShapes::Sphere(sphere));
    shapes.push(AvailableShapes::Torus(torus));

    shapes.iter_mut().for_each(|s| s.rotate_x(23.1));

    shapes.iter_mut().for_each(|s| println!("center: {}", s.get_center()));

    let mut sphere = &mut shapes[0];
    sphere.get_center().x = 1.;
    sphere.get_center().y = 2.;
    sphere.get_center().z = 3.;

    let mut torus = &mut shapes[1];
    torus.get_center().x = 5.;
    torus.get_center().y = 6.;
    torus.get_center().z = 7.;

    shapes.iter_mut().for_each(|s| println!("center: {}", s.get_center()));

    shapes.iter_mut().for_each(|s| s.translate(2., 3., 4.));

    shapes.iter_mut().for_each(|s| println!("center: {}", s.get_center()));

}
