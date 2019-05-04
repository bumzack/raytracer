use std::fmt;
use std::fmt::Display;

pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub trait VectorOps {
    fn magnitude_squared(&self) -> f32;
    fn magnitude(&self) -> f32;
    fn unit_vector(&self) -> Vector;
}

impl VectorOps for Vector {
    fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    fn unit_vector(&self) -> Vector {
        let m = self.magnitude_squared();
        Vector {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
        }
    }
}


impl Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v = ({}/{}/{})", self.x, self.y, self.z)
    }
}