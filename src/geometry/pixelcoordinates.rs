use std::fmt;
use std::fmt::Display;

use crate::algebra::color::Color;

pub struct PixelCoordinates {
    pub x: usize,
    pub y: usize,
}

impl Display for PixelCoordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}/{})", self.x, self.y)
    }
}

#[test]
fn test_stuff() {
    // TODO  use assertions :-)
}
