pub trait Tuple {
    fn new (x: f64, y: f64, z: f64) -> Self;
    fn zero() -> Self;
}