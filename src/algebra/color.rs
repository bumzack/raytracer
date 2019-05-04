use std::fmt;
use std::fmt::Display;
use std::ops::{Add, Div, Mul};

use crate::utils::raytracer_error::RayTracerError;

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub trait ColorOps {
    fn validate(&self) -> Result<(), RayTracerError>;
    fn default() -> Color;
    fn new(r: f32, g: f32, b: f32) -> Color;
    fn new_from_rgb_luminosity(r: f32, g: f32, b: f32, luminositg: f32) -> Color;
}

impl ColorOps for Color {
    fn default() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    fn new(r: f32, g: f32, b: f32) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
        }
    }

    fn new_from_rgb_luminosity(r: f32, g: f32, b: f32, luminosity: f32) -> Color {
        Color {
            r: r * luminosity,
            g: g * luminosity,
            b: b * luminosity,
        }
    }

    fn validate(&self) -> Result<(), RayTracerError> {
//        if (self.r < 0.0) {
//            Err(ColorInvalid("red is < 0".to_string()));
//        }
//        if (self.g < 0.0) {
//            Err(ColorInvalid("green is < 0".to_string()));
//        }
//        if (self.b < 0.0) {
//            Err(ColorInvalid("blue is < 0".to_string()));
//        }
//        Ok(())
        // Err(RayTracerError::ColorInvalid("df".to_owned()));

        if self.r < 0.0 {
            return Err::<(), RayTracerError>(RayTracerError::ColorInvalid("red is < 0".to_string()));
        }
        if self.g < 0.0 {
            return Err::<(), RayTracerError>(RayTracerError::ColorInvalid("green is < 0".to_string()));
        }
        if self.b < 0.0 {
            return Err::<(), RayTracerError>(RayTracerError::ColorInvalid("blue is < 0".to_string()));
        }
        Ok(())
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

// TODO make generic ?!
impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl Mul<i32> for Color {
    type Output = Color;

    fn mul(self, other: i32) -> Color {
        Color {
            r: self.r * other as f32,
            g: self.g * other as f32,
            b: self.b * other as f32,
        }
    }
}


impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: other.r * self,
            g: other.g * self,
            b: other.b * self,
        }
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, other: f32) -> Color {
        Color {
            r: self.r / other,
            g: self.g / other,
            b: self.b / other,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}/{}/{})", self.r, self.g, self.b)
    }
}

#[test]
fn test_stuff() {

    // TODO  use assertions :-)
    let c = Color::new(1.0, 2.0, 2.0);
    let c1 = Color::new(1.0, 2.0, 3.0);

    println!("{}", c);
    println!("sum of a + b = {}", c + c1);
    let c = Color::new(1.0, 2.0, 2.0);
    let c1 = Color::new(1.0, 2.0, 3.0);
    println!("diff of a + b = {}", c - c1);


    let c = Color::new(1.0, 2.0, 2.0);
    println!("v*2.5 = {}", c * 2.5);

    let c = Color::new(1.0, 2.0, 2.0);
    println!("2.5 * v = {}", 2.5 * c);


    let c = Color::new(1.0, 2.0, 2.0);
    println!(" v / 2 = {}", c / 2.0);
}