use std::fmt;
use std::fmt::Display;

use crate::algebra::color::Color;

pub struct PixelData {
    pub color: Color,
    pub is_ambigious: bool,
}

impl Display for PixelData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "color = {}, is ambigious = {}   ", self.color, self.is_ambigious)
    }
}

#[test]
fn test_stuff() {
    // TODO  use assertions :-)
}