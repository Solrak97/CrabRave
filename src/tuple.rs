pub mod t {
    struct Tuple {
        x: f64,
        y: f64,
        z: f64,
        w: u8,
    }

    impl Tuple {
        fn new(x: f64, y: f64, z: f64, w: u8) -> Self {
            Tuple {
                x: x,
                y: y,
                z: z,
                w: w,
            }
        }
    }

    struct Point {
        tuple: Tuple,
    }

    impl Point {
        fn new(x: f64, y: f64, z: f64) -> Self {
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

    struct Vector {
        tuple: Tuple,
    }

    impl Vector {
        fn new(x: f64, y: f64, z: f64) -> Self {
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
}
