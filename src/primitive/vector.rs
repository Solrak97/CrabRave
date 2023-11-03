use crate::primitive::tuple::Tuple;

#[derive(Clone, Copy, Debug)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn magnitude(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }
}

impl Tuple for Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
    }

    fn zero() -> Self {
        Vector { x: 0.0, y: 0.0, z: 0.0 }
    }
}

// vec == vec
impl std::cmp::PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y  == other.y && self.z == other.z
    }
}

// -vec
impl std::ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector { x: -self.x, y: -self.y, z: -self.z }
    }
}

// vec + vec
impl std::ops::Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self {
        Vector { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

// vec - vec
impl std::ops::Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}

// vec * scalar
impl std::ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self {
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

    fn mul(self, rhs: Self) -> Self {
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

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}
