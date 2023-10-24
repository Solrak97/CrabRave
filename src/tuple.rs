#[derive(PartialEq, Debug)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: u8,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: u8) -> Self {
        Tuple {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn is_point(&self) -> bool {
        self.w == 0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 1
    }
}

#[derive(PartialEq, Debug)]
pub struct Point {
    tuple: Tuple,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point {
            tuple: Tuple {
                x: x,
                y: y,
                z: z,
                w: 1,
            },
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Vector {
    pub tuple: Tuple,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector {
            tuple: Tuple {
                x: x,
                y: y,
                z: z,
                w: 0,
            },
        }
    }
}

/*                                     UNITESTS                                        */
#[cfg(test)]
mod tuple_test {
    use crate::tuple::*;

    #[test]
    fn is_vector() {
        let t = Tuple::new(4.3, -4.2, 3.1, 1);
        assert!(t.x == 4.3 && t.y == -4.2 && t.z == 3.1 && !t.is_point() && t.is_vector())
    }

    #[test]
    fn is_point() {
        let t = Tuple::new(4.3, -4.2, 3.1, 0);
        assert!(t.x == 4.3 && t.y == -4.2 && t.z == 3.1 && t.is_point() && !t.is_vector())
    }

    #[test]
    fn create_vector() {
        let vec = Vector::new(4.0, -4.0, 3.0);
        let tup = Tuple::new(4.0, -4.0, 3.0, 0);
        assert_eq!(vec.tuple, tup)
    }

    #[test]
    fn create_point() {
        let pnt = Point::new(4.0, -4.0, 3.0);
        let tup = Tuple::new(4.0, -4.0, 3.0, 1);
        assert_eq!(pnt.tuple, tup)
    }
}
