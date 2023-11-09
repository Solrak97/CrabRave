const EPSILON:f64 = 0.000001;

pub fn floats_equal(f1: f64, f2: f64) -> bool{
    (f1 - f2).abs() < EPSILON
}