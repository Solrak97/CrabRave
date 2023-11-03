#[derive(PartialEq, Debug)]
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

/*     Testfunc     */

pub fn floored (t: &Tuple) -> bool {
    t.y <= 0.0
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    tuple(x, y, z, 0.0)
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    tuple(x, y, z, 1.0)
}

pub fn is_vector(t: &Tuple) -> bool {
    t.w == 0.0
}

pub fn is_point(t: &Tuple) -> bool {
    t.w == 1.0
}

pub fn add_tuple(t1: &Tuple, t2: &Tuple) -> Tuple {
    tuple(t1.x + t2.x, t1.y + t2.y, t1.z + t2.z, t1.w + t2.w)
}

pub fn sub_tuple(t1: &Tuple, t2: &Tuple) -> Tuple {
    tuple(t1.x - t2.x, t1.y - t2.y, t1.z - t2.z, t1.w - t2.w)
}

pub fn neg_tuple(t: &Tuple) -> Tuple {
    let _t = tuple(0.0, 0.0, 0.0, 0.0);
    sub_tuple(&_t, t)
}

pub fn scale_tuple(t: &Tuple, s: f64) -> Tuple {
    tuple(t.x * s, t.y * s, t.z * s, t.w * s)
}

pub fn scale_tuple_div(t: &Tuple, s: f64) -> Tuple {
    scale_tuple(t, 1.0 / s)
}

pub fn vector_magnitude(t: &Tuple) -> f64 {
    (t.x.powi(2) + t.y.powi(2) + t.z.powi(2) + t.w.powi(2)).sqrt()
}

pub fn vector_normalize(t: &Tuple) -> Tuple {
    scale_tuple_div(t, vector_magnitude(t))
}

pub fn vector_dot_product(t1: &Tuple, t2: &Tuple) -> f64 {
    t1.x * t2.x + t1.y * t2.y + t1.z * t2.z + t1.w * t2.w
}

pub fn vector_cross_product(v1: &Tuple, v2: &Tuple) -> Tuple {
    vector(v1.y * v2.z - v1.z * v2.y, v1.z * v2.x - v1.x * v2.z, v1.x * v2.y - v1.y * v2.x)
}

/*                                     UNITESTS                                        */
#[cfg(test)]
mod tuple_test {
    use crate::tuple::*;

    #[test]
    fn _is_vector() {
        assert!(is_vector(&tuple(1.0, 2.0, -3.0, 0.0)));
    }

    #[test]
    fn is_not_vector() {
        assert!(!is_vector(&tuple(1.0, 2.0, -3.0, 1.0)));
    }

    #[test]
    fn vector_is_point() {
        assert!(is_vector(&vector(1.0, 2.0, -3.0)));
    }

    #[test]
    fn _is_point() {
        assert!(is_point(&tuple(1.0, 2.0, -3.0, 1.0)));
    }

    #[test]
    fn is_not_point() {
        assert!(!is_point(&tuple(1.0, 2.0, -3.0, 0.0)));
    }

    #[test]
    fn point_is_point() {
        assert!(is_point(&point(1.0, 2.0, -3.0)));
    }

    #[test]
    fn tuple_addition() {
        let t1 = tuple(3.0, -2.0, 5.0, 1.0);
        let t2 = tuple(-2.0, 3.0, 1.0, 0.0);
        assert_eq!(add_tuple(&t1, &t2), tuple(1.0, 1.0, 6.0, 1.0))
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        assert_eq!(sub_tuple(&p1, &p2), vector(-2.0, -4.0, -6.0))
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = vector(5.0, 6.0, 7.0);
        assert_eq!(sub_tuple(&p1, &p2), point(-2.0, -4.0, -6.0))
    }

    #[test]
    fn subtracting_two_vectors() {
        let p1 = vector(3.0, 2.0, 1.0);
        let p2 = vector(5.0, 6.0, 7.0);
        assert_eq!(sub_tuple(&p1, &p2), vector(-2.0, -4.0, -6.0))
    }

    #[test]
    fn subtracting_a_vector_from_vector_zero() {
        let p1 = vector(0.0, 0.0, 0.0);
        let p2 = vector(1.0, -2.0, 3.0);
        assert_eq!(sub_tuple(&p1, &p2), vector(-1.0, 2.0, -3.0))
    }

    #[test]
    fn negating_a_tuple() {
        let p2 = vector(1.0, -2.0, 3.0);
        assert_eq!(neg_tuple(&p2), tuple(-1.0, 2.0, -3.0, 0.0))
    }

    #[test]
    fn scaling_tuple() {
        let t = tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(scale_tuple(&t, 3.5), tuple(3.5, -7.0, 10.5, -14.0))
    }

    #[test]
    fn scaling_tuple_by_fraction() {
        let t = tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(scale_tuple(&t, 0.5), tuple(0.5, -1.0, 1.5, -2.0))
    }

    #[test]
    fn scaling_tuple_division() {
        let t = tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(scale_tuple_div(&t, 2.0), tuple(0.5, -1.0, 1.5, -2.0))
    }

    #[test]
    fn magnitude_of_vector() {
        let t = vector(-1.0, -2.0, -3.0);
        let t1: f64 = 14.0; //cast issue?
        assert_eq!(vector_magnitude(&t), t1.sqrt())
    }

    #[test]
    fn normalize_vector() {
        let t = vector(1.0, 2.0, 3.0);
        let t1: f64 = 14.0; //cast issue?
        let tmp = t1.sqrt();
        assert_eq!(vector_normalize(&t), tuple(1.0 / tmp, 2.0 / tmp, 3.0 / tmp, 0.0))
    }

    #[test]
    fn dot_product_of_vectors() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);
        assert_eq!(vector_dot_product(&v1, &v2), 20.0)
    }

    #[test]
    fn cross_product_of_vectors() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);
        assert_eq!(vector_cross_product(&v1, &v2), vector(-1.0, 2.0, -1.0));
        assert_eq!(vector_cross_product(&v2, &v1), vector(1.0, -2.0, 1.0))
    }


    /*

    #[test]
    fn reflecting_vector_45_deg() {
        let v = vector(1.0, -1.0, 0.0);
        let n = vector(0.0, 1.0, 0.0);
        let r = vector_reflect(&v, &n);
        assert_eq!(r, vector(1.0, 1.0, 0.0))
    }

    #[test]
    fn reflecting_vector_off_slanted_surface() {
        let v = vector(0.0, -1.0, 0.0);
        let value = (2.0_f64).sqrt() / 2.0;
        let n = vector(value, value, 0.0);
        let r = vector_reflect(&v, &n);
        assert_eq!(r, vector(1.0000000000000002, 0.0000000000000002220446049250313, 0.0))
    }
     */
}
