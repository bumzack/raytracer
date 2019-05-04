use std::fmt;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub trait VectorOps {
    fn magnitude_squared(&self) -> f32;
    fn magnitude(&self) -> f32;
    fn unit_vector(&self) -> Vector;
    fn new(x: f32, y: f32, z: f32) -> Vector;
    fn dot_product(a: &Vector, b: &Vector) -> f32;
    fn cross_product(a: &Vector, b: &Vector) -> Vector;
}

impl VectorOps for Vector {
    fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector {
            x: x,
            y: y,
            z: z,
        }
    }

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

    fn dot_product(a: &Vector, b: &Vector) -> f32 {
        a.x * a.x + a.y * b.y + a.z * b.z
    }

    fn cross_product(a: &Vector, b: &Vector) -> Vector {
        Vector {
            x: (a.y * b.z) - (a.z * b.y),
            y: (a.z * b.x) - (a.x * b.z),
            z: (a.x * b.y) - (a.y * b.x),
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// TODO make generic ?!
impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, other: f32) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<i32> for Vector {
    type Output = Vector;

    fn mul(self, other: i32) -> Vector {
        Vector {
            x: self.x * other as f32,
            y: self.y * other as f32,
            z: self.z * other as f32,
        }
    }
}

impl Mul<Vector> for i32 {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: other.x * self as f32,
            y: other.y * self as f32,
            z: other.z * self as f32,
        }
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self,
        }
    }
}

impl Div<i32> for Vector {
    type Output = Vector;

    fn div(self, other: i32) -> Vector {
        Vector {
            x: self.x / other as f32,
            y: self.y / other as f32,
            z: self.z / other as f32,
        }
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, other: f32) -> Vector {
        Vector {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}/{}/{})", self.x, self.y, self.z)
    }
}

#[test]
fn test_stuff() {

    // TODO  use assertions :-)
    let v = Vector::new(1.0, 2.0, 2.0);
    let v1 = Vector::new(1.0, 2.0, 3.0);

    println!("{}", v);
    println!("magnitude = {}", v.magnitude());
    println!("unit vector = {}", v.unit_vector());

    println!("dot of a . b = {}", Vector::dot_product(&v, &v1));
    println!("cross of a x b = {}", Vector::cross_product(&v, &v1));

    println!("sum of a + b = {}", v + v1);

    let v = Vector::new(1.0, 2.0, 2.0);
    let v1 = Vector::new(1.0, 2.0, 3.0);
    println!("diff of a + b = {}", v - v1);

    let v = Vector::new(1.0, 2.0, 2.0);
    println!("2*v  = {}", 2 * v);

    let v = Vector::new(1.0, 2.0, 2.0);
    println!("v*2.5 = {}", v * 2.5);

    let v = Vector::new(1.0, 2.0, 2.0);
    println!("2.5 * v = {}", 2.5 * v);

    let v = Vector::new(1.0, 2.0, 2.0);
    println!(" v / 2 = {}", v / 2);

    let v = Vector::new(1.0, 2.0, 2.0);
    println!(" v / 2 = {}", v / 2.0);

    let v = Vector::new(1.0, 2.0, 2.0);
    println!(" -v  = {}", -v);
}