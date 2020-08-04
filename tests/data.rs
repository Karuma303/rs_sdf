#[cfg(test)]
mod source_tests {
    use rs_sdf::data::input::{InputField, BoolInputData, ByteInputData};
    use bitvec::prelude::*;

    #[test]
    #[should_panic]
    fn source_must_have_width_greater_than_zero() {
        let b = vec![true];
        InputField::from(BoolInputData::new(b, 0, 1));
    }

    #[test]
    #[should_panic]
    fn source_must_have_height_greater_than_zero() {
        let b = vec![true];
        InputField::from(BoolInputData::new(b, 1, 0));
    }

    #[test]
    #[should_panic]
    fn buffer_size_does_not_match_given_dimensions() {
        let b = vec![true];
        InputField::from(BoolInputData::new(b, 10, 10));
    }

    #[test]
    fn source_field_is_correct() {
        let b = vec![0, 128, 255, 0];
        let f = InputField::from(ByteInputData::new(b, 127, 4, 1));
        assert_eq!(f.data, bitvec![0, 1, 1, 0]);
    }

    #[test]
    fn invert() {
        let b = vec![0, 255, 0];
        let mut f = InputField::from(ByteInputData::new(b, 127, 3, 1));
        f.invert();
        assert_eq!(f.data, bitvec![1, 0, 1]);
    }
}