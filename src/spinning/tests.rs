#[cfg(test)]
mod spinning_tests {
    use super::super::Spinning;

    #[test]
    #[should_panic]
    fn min_velocity_l_test() {
        Spinning::new(0.0, 20.0, 360);
    }

    #[test]
    #[should_panic]
    fn min_velocity_h_test() {
        Spinning::new(1.1, 20.0, 360);
    }

    #[test]
    #[should_panic]
    fn max_velocity_l_test() {
        Spinning::new(1.0, 0.9, 360);
    }

    #[test]
    #[should_panic]
    fn max_velocity_h_test() {
        Spinning::new(1.0, 31.0, 360);
    }

    #[test]
    #[should_panic]
    fn circles_test() {
        Spinning::new(1.0, 30.0, 359);
    }

    #[test]
    fn stoped_degree_test() {
        let mut spinning = Spinning::new(0.5, 20.0, 365);
        let stoped_degree = 3;

        spinning.start(stoped_degree);

        while !spinning.is_stop() {
            spinning.tick();
        }

        assert_eq!(stoped_degree as f64, spinning.degree().floor());
    }
}
