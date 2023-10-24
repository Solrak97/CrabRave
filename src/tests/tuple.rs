#[cfg(test)]
mod tuple_test {

    use crate::tuple::*;


    #[test]
    fn is_vector() {

        let tup = Tuple::new(4.3, -4.2, 3.1, 0);
        let vec = Vector::new(4.3, -4.2, 3.1);

        assert_eq!(vec.tuple, tup);
    }

    #[test]
    fn is_point() {
        assert!(true)
    }

    #[test]
    fn create_vector() {
        assert!(true)
    }

    #[test]
    fn create_point() {
        assert!(true)
    }
}
