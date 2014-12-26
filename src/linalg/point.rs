use std::num::Float;

use linalg::vector;

/// Point is a standard 3 component point but transforms as a point
/// point when transformations are applied
#[deriving(Show, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    /// Initialize the point and set values for x, y, z
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x: x, y: y, z: z }
    }
    /// Initialize the point with the same value of x, y, z
    pub fn broadcast(x: f32) -> Point {
        Point { x: x, y: x, z: x }
    }
    /// Compute the squared length of the point
    pub fn length_sqr(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.x
    }
    /// Compute the length of the point
    pub fn length(&self) -> f32 {
        Float::sqrt(self.length_sqr())
    }
    /// Get a pointized copy of this point
    pub fn pointized(&self) -> Point {
        let len = self.length();
        Point { x: self.x / len, y: self.y / len, z: self.z / len }
    }
    /// Compute the squared distance between this point and another
    pub fn distance_sqr(&self, a: &Point) -> f32 {
        (*self - *a).length_sqr()
    }
    /// Compute the distance between this point and another
    pub fn distance(&self, a: &Point) -> f32 {
        (*self - *a).length()
    }
}

impl Add<Point, Point> for Point {
    /// Add two points together
    fn add(self, rhs: Point) -> Point {
        Point { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Add<vector::Vector, Point> for Point {
    /// Add two points together
    fn add(self, rhs: vector::Vector) -> Point {
        Point { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Sub<Point, vector::Vector> for Point {
    /// Subtract two points to get the vector between them
    fn sub(self, rhs: Point) -> vector::Vector {
        vector::Vector { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl Sub<vector::Vector, Point> for Point {
    /// Subtract a vector from a point, translating the point by -vector
    fn sub(self, rhs: vector::Vector) -> Point {
        Point { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl Mul<f32, Point> for Point {
    /// Scale the point by some value
    fn mul(self, rhs: f32) -> Point {
        Point { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl Mul<Point, Point> for f32 {
    /// Scale the point by some value
    fn mul(self, rhs: Point) -> Point {
        rhs * self
    }
}

impl Div<Point, Point> for Point {
    /// Divide the points components by the right hand side's components
    fn div(self, rhs: Point) -> Point {
        Point { x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z }
    }
}

impl Div<f32, Point> for Point {
    /// Divide the points components by scalar
    fn div(self, rhs: f32) -> Point {
        Point { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl Neg<Point> for Point {
    /// Negate the point
    fn neg(self) -> Point {
        Point { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Index<uint, f32> for Point {
    /// Access the point by index
    ///
    /// - 0 = x
    /// - 1 = y
    /// - 2+ = z
    fn index(&self, i: &uint) -> &f32 {
        match *i {
            0 => &self.x,
            1 => &self.y,
            _ => &self.z,
        }
    }
}

impl IndexMut<uint, f32> for Point {
    /// Access the point by index
    ///
    /// - 0 = x
    /// - 1 = y
    /// - 2+ = z
    fn index_mut(&mut self, i: &uint) -> &mut f32 {
        match *i {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => &mut self.z,
        }
    }
}

