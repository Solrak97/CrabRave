#[derive(Debug)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

pub fn tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
    Tuple {
        x: x,
        y: y,
        z: z,
        w: w,
    }
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    tuple(x, y, z, 0.0)
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    tuple(x, y, z, 1.0)
}

pub fn is_point(t: &Tuple) -> bool {
    t.w == 0.0
}

pub fn is_vector(t: &Tuple) -> bool {
    t.w == 1.0
}

pub fn add_tuple(t1: &Tuple, t2: &Tuple) -> Tuple {
    tuple(t1.x + t2.x, t1.y + t2.y, t1.z + t2.z, t1.w + t2.w)
}

pub fn sub_tuple(t1: &Tuple, t2: &Tuple) -> Tuple {
    tuple(t1.x - t2.x, t1.y - t2.y, t1.z - t2.z, t1.w - t2.w)
}

pub fn neg_tuple(t: &Tuple) -> Tuple {
    let _t = tuple(0.0,0.0,0.0,0.0);
    sub_tuple(&_t, t)
}

pub fn mul_tuple(t: &Tuple, s:f64) -> Tuple{
    tuple(t.x * s, t.y * s, t.z * s, t.w * s)
}

pub fn div_tuple(t: &Tuple, s: f64) -> Tuple {
    mul_tuple(t, 1.0/s)
}

pub fn magnitude(t: Tuple) -> f64{
    (t.x.powi(2) + t.y.powi(2) + t.z.powi(2) + t.w.powi(2)).sqrt()
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
