#[cfg(test)]
mod source_tests {
    use rs_sdf::data::source::SourceField;

    #[test]
    #[should_panic]
    fn source_must_have_width_greater_than_zero() {
        let b = [true];
        SourceField::from_booleans(&b, 0, 1);
    }

    #[test]
    #[should_panic]
    fn source_must_have_height_greater_than_zero() {
        let b = [true];
        SourceField::from_booleans(&b, 1, 0);
    }

    #[test]
    #[should_panic]
    fn buffer_size_does_not_match_given_dimensions() {
        let b = [true];
        SourceField::from_booleans(&b, 10, 10);
    }

    #[test]
    fn source_field_is_correct() {
        let b = [0, 128, 255, 0];
        let f = SourceField::from_bytes(&b, 127, 4, 1);
        assert_eq!(f.data, [false, true, true, false]);
    }

    #[test]
    fn invert() {
        let b = [0, 255, 0];
        let mut f = SourceField::from_bytes(&b, 127, 3, 1);
        f.invert();
        assert_eq!(f.data, [true, false, true]);
    }
}