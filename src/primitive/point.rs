use crate::primitive::{ tuple::Tuple, vector::Vector };

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Tuple for Point {
    fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x: x, y: y, z: z }
    }

    fn zero() -> Point {
        Point { x: 0.0, y: 0.0, z: 0.0 }
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }

    fn w(&self) -> f64 {
        1.0
    }
}

// -Point
impl std::ops::Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point { x: -self.x, y: -self.y, z: -self.z }
    }
}

// Point + Vector
impl std::ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Point {
        Point { x: self.x + rhs.x(), y: self.y + rhs.y(), z: self.z + rhs.z() }
    }
}

// Vector + Point
impl std::ops::Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        rhs + self
    }
}

// Point - Vector
impl std::ops::Sub<Point> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Vector {
        Vector::new(self.x() - rhs.x, self.y() - rhs.y, self.z() - rhs.z)
    }
}


// Point * scalar
impl std::ops::Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Point {
        Point { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

// scalar * Point
impl std::ops::Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Point {
        rhs * self
    }
}

// Point / scalar
impl std::ops::Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Point {
        self * (1.0 / rhs)
    }
}