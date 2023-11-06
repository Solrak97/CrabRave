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
impl std::ops::Sub<Vector> for Point {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
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






#[cfg(test)]
mod tests {
    use crate::primitive::{ point::Point, tuple::Tuple, vector::Vector };
    #[test]
    fn is_point() {
        let p = Point::new(-0.5, 0.4, 1.7);
        assert!(p.x == -0.5 && p.y == 0.4 && p.z == 1.7);
    }

    #[test]
    fn _neg_point() {
        let p = Point::new(3.0, -2.0, 5.0);
        assert_eq!(Point::new(-3.0, 2.0, -5.0), -p)
    }

    #[test]
    fn _add_point_vector() {
        let p = Point::new(3.0, -2.0, 5.0);
        let v = Vector::new(-2.0, 3.0, 1.0);
        assert_eq!(Point::new(1.0, 1.0, 6.0), v + p)
    }

    #[test]
    fn _add_vector_point() {
        let p = Point::new(3.0, -2.0, 5.0);
        let v = Vector::new(-2.0, 3.0, 1.0);
        assert_eq!(Point::new(1.0, 1.0, 6.0), p + v)
    }

    #[test]
    fn _sub_point_vector() {
        let p = Point::new(3.0, -2.0, 5.0);
        let v = Vector::new(-2.0, 3.0, 1.0);
        assert_eq!(Vector::new(5.0, -5.0, 4.0), p - v)
    }

    #[test]
    fn _point_mul_scalar() {
        let p = Point::new(1.0, 2.0, 3.0);
        let s = 3.0;

        assert_eq!(Point::new(3.0, 6.0, 9.0), p * s)
    }

    #[test]
    fn _scalar_mul_point() {
        let p = Point::new(1.0, 2.0, 3.0);
        let s = 3.0;

        assert_eq!(Point::new(3.0, 6.0, 9.0), s * p)
    }

    #[test]
    fn _point_div_scalar() {
        let p = Vector::new(3.0, 6.0, -9.0);
        let s = 3.0;

        assert_eq!(Vector::new(1.0, 2.0, -3.0), p / s)
    }
}
