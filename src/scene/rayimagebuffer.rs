use std::fmt;
use std::fmt::Display;

use crate::algebra::color::ColorOps;
use crate::geometry::pixeldata::PixelData;

pub struct RayImageBuffer {
    width: usize,
    height: usize,
    pixels: Vec<PixelData>,
}

impl RayImageBuffer {
    pub fn new(width: usize, height: usize) -> RayImageBuffer {
        RayImageBuffer {
            width: width,
            height: height,
            pixels: Vec::new(),
        }
    }

    pub fn max_color_value(self) -> f32
    {
        let mut max = 0.0;
        for p in self.pixels.iter() {
            p.color.validate().expect("crap - color not valid");
            if p.color.r > max {
                max = p.color.r;
            }
            if p.color.g > max {
                max = p.color.g;
            }
            if p.color.b > max {
                max = p.color.b;
            }
        }
        if max == 0.0 {
            max = 1.0;
        }
        max
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> &mut PixelData {
        &mut self.pixels[y * self.width + x]
    }

    pub fn get_width(self) -> usize {
        self.width
    }

    pub fn get_height(self) -> usize {
        self.height
    }
}

impl Display for RayImageBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: add all fields
        write!(f, "width = {}, height = {}", self.width, self.height)
    }
}

#[test]
fn test_stuff() {
    // TODO  use assertions :-)
}
