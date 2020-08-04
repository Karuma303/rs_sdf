#[cfg(test)]
mod tests {
    use rs_sdf::data::DistanceField;
    use rs_sdf::data::input::{BoolInputData, InputField};

    // helper method to get an empty source field
    fn get_source_0_0() -> InputField {
        InputField::from(BoolInputData::new(vec![], 0, 0))
    }

    #[test]
    #[should_panic]
    fn source_is_empty() {
        let source = get_source_0_0();
        DistanceField::new(&source);
    }

    #[test]
    fn distance_field_has_correct_dimension() {
        let source = InputField::from(BoolInputData::new(vec![true, true, true], 3, 1));
        let df = DistanceField::new(&source);

        assert_eq!(df.width, 3);
        assert_eq!(df.height, 1);

        let source = InputField::from(BoolInputData::new(vec![true, true, true], 1, 3));
        let df = DistanceField::new(&source);

        assert_eq!(df.width, 1);
        assert_eq!(df.height, 3);
    }
}