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
