#[cfg(test)]
mod ship_speed_tests {
    #[test]
    fn one_box_thruster_1000kg_test() {
        let test_weight: f64 = 1_000_000.0;

        let result = starbase_tools::calculate_potential_ship_speed(&1, &0, &test_weight);

        assert_eq!(result, 22.19)
    }

    #[test]
    fn one_triangle_thruster_1000kg_test() {
        let test_weight: f64 = 1_000_000.0;

        let result = starbase_tools::calculate_potential_ship_speed(&0, &1, &test_weight);

        assert_eq!(result, 13.53)
    }

    #[test]
    fn one_of_each_thruster_1000kg_test() {
        let test_weight: f64 = 1_000_000.0;

        let result = starbase_tools::calculate_potential_ship_speed(&1, &1, &test_weight);

        assert_eq!(result, 34.65)
    }
}
