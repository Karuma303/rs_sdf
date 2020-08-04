#[cfg(test)]
mod tests {
    use rs_sdf::data::input::InputField;
    use bitvec::prelude::*;

    #[test]
    fn distance_field_initializes_correctly() {
        let sf: InputField = InputField {
            width: 2,
            height: 2,
            data: bitvec![1, 0, 0, 1],
        };
        assert_eq!(sf.width, 2);
        assert_eq!(sf.height, 2);
        assert_eq!(sf.data[0], true);
        assert_eq!(sf.data[1], false);
        assert_eq!(sf.data[2], false);
        assert_eq!(sf.data[3], true);
        // TODO: add more checks here!
    }
}