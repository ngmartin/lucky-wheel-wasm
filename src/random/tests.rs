#[cfg(test)]
mod random_tests {
    use super::super::rand;

    #[test]
    fn weight_0_test() {
        let random_items = vec![(1, 1), (2, 0)];
        let random_item = rand(&random_items);

        assert_eq!(random_item.0, 1)
    }

    #[test]
    fn random_test() {
        let random_items = vec![(1, 1), (2, 1)];
        let random_item = rand(&random_items);

        assert!(random_items.iter().position(|el| el.0 == random_item.0) != None)
    }
}
