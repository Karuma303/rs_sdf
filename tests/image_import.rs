#[cfg(test)]
mod tests {
    use rs_sdf::import::image::{FileInputError, get_source_from_png_file_input};
    use rs_sdf::import::image;

    const TEST_ASSET_BASE_PATH: &str = r"tests/test_assets/";

    #[test]
    fn input_does_not_exist() {
        let res = get_source_from_png_file_input("some_random_path_that_does_not_exist.dat");
        assert_eq!(res.err().unwrap(), FileInputError::InvalidFile);
    }

    #[test]
    fn input_is_not_png() {
        let res = get_source_from_png_file_input(r"tests/test_assets/invalid_file.dat");
        assert_eq!(res.err().unwrap(), FileInputError::InvalidFileType);
    }

    #[test]
    fn input_file_is_no_valid_rgba() {
        let res = get_source_from_png_file_input(r"tests/test_assets/test_rgb_1x1_black.png");
        assert_eq!(res.err().unwrap(), FileInputError::InvalidImageFormat);
    }

    #[test]
    fn input_file_is_valid() {
        let res = get_source_from_png_file_input(r"tests/test_assets/test_rgba_1x1_fully_transparent.png");
        assert!(res.is_ok());
    }

    #[test]
    fn generated_source_field_contains_valid_data() {

        // check fully transparent 1x1 image
        let s1 = get_source_from_png_file_input(r"tests/test_assets/test_rgba_1x1_fully_transparent.png").unwrap();
        assert_eq!(s1.data[0], false);

        // check fully opaque 1x1 image
        let s2 = get_source_from_png_file_input(r"tests/test_assets/test_rgba_1x1_90_percent_opaque.png").unwrap();
        assert_eq!(s2.data[0], true);

        // check checkered 2x2 image
        let s3 = get_source_from_png_file_input(r"tests/test_assets/test_rgba_2x2_checkerboard.png").unwrap();
        assert_eq!(s3.data[0], true); // tl
        assert_eq!(s3.data[1], false); // tr
        assert_eq!(s3.data[2], false); // bl
        assert_eq!(s3.data[3], true); // br
    }
}
