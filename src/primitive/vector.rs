use crate::primitive::tuple::Tuple;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn magnitude(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Vector {
        self / self.magnitude()
    }
}

impl Tuple for Vector {
    fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    fn zero() -> Vector {
        Vector { x: 0.0, y: 0.0, z: 0.0 }
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
        0.0
    }
}

// -vec
impl std::ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector { x: -self.x, y: -self.y, z: -self.z }
    }
}

// vec + vec
impl std::ops::Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Vector {
        Vector { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

// vec - vec
impl std::ops::Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Vector {
        self + -rhs
    }
}

// vec * scalar
impl std::ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Vector {
        Vector { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

// scalar * vec
impl std::ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        rhs * self
    }
}

// vec * vec
impl std::ops::Mul for Vector {
    type Output = Vector;

    fn mul(self, rhs: Self) -> Vector {
        Vector {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

// vec ^ vec
impl std::ops::BitXor for Vector {
    type Output = f64;

    fn bitxor(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

// vec / scalar
impl std::ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Vector {
        self * (1.0 / rhs)
    }
}





#[cfg(test)]
mod tests {
    use crate::primitive::{ vector::Vector, tuple::Tuple };
    #[test]
    fn is_vector() {
        let v = Vector::new(-0.5, 0.4, 1.7);
        assert!(v.x == -0.5 && v.y == 0.4 && v.z == 1.7);
    }

    #[test]
    fn _neg_vector() {
        let v = Vector::new(3.0, -2.0, 5.0);
        assert_eq!(Vector::new(-3.0, 2.0, -5.0), -v)
    }

    #[test]
    fn _add_vectors() {
        let v1 = Vector::new(3.0, -2.0, 5.0);
        let v2 = Vector::new(-2.0, 3.0, 1.0);
        assert_eq!(Vector::new(1.0, 1.0, 6.0), v1 + v2)
    }

    #[test]
    fn _sub_vectors() {
        let v1 = Vector::new(3.0, -2.0, 5.0);
        let v2 = Vector::new(-2.0, 3.0, 1.0);
        assert_eq!(Vector::new(5.0, -5.0, 4.0), v1 - v2)
    }

    #[test]
    fn _vec_mul_scalar() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let s = 3.0;

        assert_eq!(Vector::new(3.0, 6.0, 9.0), v * s)
    }

    #[test]
    fn _scalar_mul_vec() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let s = 3.0;

        assert_eq!(Vector::new(3.0, 6.0, 9.0), s * v)
    }

    #[test]
    fn _cross_product() {
        let v1 = Vector::new(-1.0, -2.0, 3.0);
        let v2 = Vector::new(4.0, 0.0, -8.0);

        assert_eq!(Vector::new(16.0, 4.0, 8.0), v1 * v2)
    }

    #[test]
    fn _dot_product() {
        let v1 = Vector::new(-1.0, -2.0, 3.0);
        let v2 = Vector::new(4.0, 0.0, -8.0);

        assert_eq!(-28.0, v1 ^ v2)
    }

    #[test]
    fn _vec_div_scalar() {
        let v = Vector::new(3.0, 6.0, -9.0);
        let s = 3.0;

        assert_eq!(Vector::new(1.0, 2.0, -3.0), v / s)
    }
}
