use crate::primitive::tuple::Tuple;

pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Tuple for Point {
    fn new (x: f64, y: f64, z: f64) -> Self {
        Point {x: x, y: y, z: z}
    }

    fn zero() -> Self {
        Point {x: 0.0, y: 0.0, z: 0.0}
    }
}