//! Module to read png image files as an input for generation
use std::fs::File;

use png::{Decoder, ColorType};
use png::Transformations;

use crate::source::SourceField;
use std::fmt;

/// Type for a valid input for the distance field generator.
/// Implementors of this trait provide a SourceField that can be further processed by the generator.
pub trait FieldInput {
    fn get_source_field(&self) -> Option<SourceField>;
}

pub struct PngInput {
    file_path: String,
}

impl PngInput {
    pub fn new(file_path: &String) -> Self {
        Self {
            file_path : String::from(file_path),
        }
    }
}

impl FieldInput for PngInput {
    fn get_source_field(&self) -> Option<SourceField> {
        let res = get_source_from_png_file_input(&self.file_path);
        match res {
            Ok(sourcefield) => Some(sourcefield),
            Err(_) => None
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FileInputError {
    InvalidFile,
    // e.g. path points to no file
    InvalidFileType,
    // no PNG file
    InvalidImageFormat, // not RGBA or something
}

impl fmt::Display for FileInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error: {:?}", self)
    }
}

/// Opens a png file from the given path and converts it into a SourceField
fn get_source_from_png_file_input(file_path: &str) -> Result<SourceField, FileInputError> {
    let input_file = File::open(&file_path).map_err(|_| { FileInputError::InvalidFile })?;

    // The decoder is a build for reader and can be used to set various decoding options
    // via `Transformations`. The default output transformation is `Transformations::EXPAND
    // | Transformations::STRIP_ALPHA`.

    // NOTE: There's also a new_with_limits() constructor !
    let mut d = Decoder::new(input_file);
    d.set_transformations(Transformations::IDENTITY);

    let read_info = d.read_info().map_err(|_| { FileInputError::InvalidFileType })?;

    let mut reader = read_info.1;
    let info = read_info.0;

    if info.color_type != ColorType::RGBA {
        return Err(FileInputError::InvalidImageFormat);
    }

    println!("source png: {}", file_path);
    println!("size (w/h): {} * {}", info.width, info.height);
    println!("line_size (bytes): {}", info.line_size);
    println!("size (bytes): {}", info.buffer_size());
    println!("color type: {:?}", info.color_type); // Grayscale, RGB, Indexed, GrayscaleAlpha, RGBA
    println!("bit depth: {:?}", info.bit_depth); // One, Two, Four, Eight, Sixteen

    // Allocate the output buffer.
    let mut image_buffer = vec![0; info.buffer_size()];
    // Read the next frame. Currently this function should only called once.
    // The default options
    reader.next_frame(&mut image_buffer).unwrap();

    // the source is RGBA (8 bit per channel)
    // in this case, we just take a look at the 8-bit alpha channel
    let mut output_buffer = vec![0u8; info.buffer_size() / 4];
    for (index, element) in output_buffer.iter_mut().enumerate() {
        *element = image_buffer[index * 4 + 3];
    }

    let source = SourceField::new(&output_buffer, info.width, info.height);
    Ok(source)
}

#[cfg(test)]
mod tests {
    use crate::input::{get_source_from_png_file_input, FileInputError};

    const TEST_ASSET_BASE_PATH: &str = r"test/";

    #[test]
    fn input_does_not_exist() {
        let res = get_source_from_png_file_input("some_random_path_that_does_not_exist.dat");
        assert_eq!(res.err().unwrap(), FileInputError::InvalidFile);
    }

    #[test]
    fn input_is_not_png() {
        let res = get_source_from_png_file_input(r"test/invalid_file.dat");
        assert_eq!(res.err().unwrap(), FileInputError::InvalidFileType);
    }

    #[test]
    fn input_file_is_no_valid_rgba() {
        let res = get_source_from_png_file_input(r"test/test_rgb_1x1_black.png");
        assert_eq!(res.err().unwrap(), FileInputError::InvalidImageFormat);
    }

    #[test]
    fn input_file_is_valid() {
        let res = get_source_from_png_file_input(r"test/test_rgba_1x1_fully_transparent.png");
        assert!(res.is_ok());
    }

    #[test]
    fn generated_source_field_contains_valid_data() {

        // check fully transparent 1x1 image
        let s1 = get_source_from_png_file_input(r"test/test_rgba_1x1_fully_transparent.png").unwrap();
        assert_eq!(s1.data[0], false);

        // check fully opaque 1x1 image
        let s2 = get_source_from_png_file_input(r"test/test_rgba_1x1_90_percent_opaque.png").unwrap();
        assert_eq!(s2.data[0], true);

        // check checkered 2x2 image
        let s3 = get_source_from_png_file_input(r"test/test_rgba_2x2_checkerboard.png").unwrap();
        assert_eq!(s3.data[0], true); // tl
        assert_eq!(s3.data[1], false); // tr
        assert_eq!(s3.data[2], false); // bl
        assert_eq!(s3.data[3], true); // br
    }
}
