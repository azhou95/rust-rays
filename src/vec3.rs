use core::fmt;

use std::ops::{Add, Sub, Neg, Mul, Div};

#[derive(Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}


impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn dot(&self, v: Vec3) -> f64 {
        self.x * v.x
        + self.y * v.y
        + self.z * v.z
    }

    pub fn cross(&self, v: Vec3) -> Vec3 {
        Vec3{
            x: self.y * v.z + self.z * v.y,
            y: self.x * v.z + self.z * v.x,
            z: self.x * v.y + self.y * v.x
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x
        + self.y * self.y
        +self.z * self.z
    }

    pub fn to_rgb(&self) -> String {
        let r = (255.999 * self.x) as i32;
        let g = (255.999 * self.y) as i32;
        let b = (255.999 * self.z) as i32;
        format!("{} {} {}", r, g, b)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z
        }
    }
}


impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3{
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, vector: Vec3) -> Vec3 {
        Vec3{
            x: self * vector.x,
            y: self * vector.y,
            z: self * vector.z
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, scalar: f64) -> Vec3 {
        Vec3{
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }
}

// Hadamard product
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3{
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, scalar: f64) -> Vec3 {
        Vec3{
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar
        }
    }
}
