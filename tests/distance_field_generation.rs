#[cfg(test)]
mod tests {
    use rs_sdf::data::source::SourceField;
    use rs_sdf::data::DistanceField;

    // helper method to get an empty source field
    fn get_source_0_0() -> SourceField {
        SourceField::from_booleans(&[], 0, 0)
    }

    #[test]
    #[should_panic]
    fn source_is_empty() {
        let source = get_source_0_0();
        DistanceField::new(&source);
    }

    #[test]
    fn distance_field_has_correct_dimension() {
        let source = SourceField::from_booleans(&[true, true, true], 3, 1);
        let df = DistanceField::new(&source);

        assert_eq!(df.width, 3);
        assert_eq!(df.height, 1);

        let source = SourceField::from_booleans(&[true, true, true], 1, 3);
        let df = DistanceField::new(&source);

        assert_eq!(df.width, 1);
        assert_eq!(df.height, 3);
    }
}