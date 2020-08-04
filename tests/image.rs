#[cfg(test)]
mod tests {
    use rs_sdf::input::image::{PngInput};
    use rs_sdf::input::{Input, InputError};
    use rs_sdf::data::input::InputField;

    fn get_png_source(file_path: &str) -> Result<InputField, InputError> {
        PngInput::new(&file_path.to_string()).source_field()
    }

    #[test]
    fn input_does_not_exist() {
        let source
            = get_png_source("non_existing_path.dat");

        assert!(source.is_err());
        assert!(matches!(source.unwrap_err(), InputError::InvalidInput{ message:_}));
    }

    #[test]
    fn input_is_not_png() {
        let source
            = get_png_source(r"tests/test_assets/invalid_file.dat");

        assert!(source.is_err());
        // assert_eq!(source.unwrap_err(), InputError::InvalidInput);
        assert!(matches!(source.unwrap_err(), InputError::InvalidInput{message:_}));
    }

    #[test]
    fn input_file_is_no_valid_rgba() {
        let source
            = get_png_source(r"tests/test_assets/test_rgb_1x1_black.png");

        assert!(source.is_err());
        assert!(matches!(source.unwrap_err(), InputError::InvalidInput{message : _}));
    }

    #[test]
    fn input_file_is_valid() {
        let res
            = get_png_source(r"tests/test_assets/test_rgba_1x1_fully_transparent.png");
        assert!(res.is_ok());
    }

    #[test]
    fn generated_source_field_contains_valid_data() {

        // check fully transparent 1x1 image
        let s1 = get_png_source(r"tests/test_assets/test_rgba_1x1_fully_transparent.png").unwrap();
        assert_eq!(s1.data[0], false);

        // check fully opaque 1x1 image
        let s2 = get_png_source(r"tests/test_assets/test_rgba_1x1_90_percent_opaque.png").unwrap();
        assert_eq!(s2.data[0], true);

        // check checkered 2x2 image
        let s3 = get_png_source(r"tests/test_assets/test_rgba_2x2_checkerboard.png").unwrap();
        assert_eq!(s3.data[0], true); // tl
        assert_eq!(s3.data[1], false); // tr
        assert_eq!(s3.data[2], false); // bl
        assert_eq!(s3.data[3], true); // br
    }
}
