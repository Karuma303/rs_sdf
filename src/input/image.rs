use std::fs::File;

use png::{Decoder, ColorType};
use png::Transformations;

use std::fmt;
use crate::input::{Input, InputError};
use crate::data::input::{InputField, ByteInputData};

pub struct PngInput {
    file_path: String,
}

impl PngInput {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: String::from(file_path),
        }
    }

    /// Opens a png file from the given path and converts it into a SourceField
    fn get_source_from_png_file_input(&self) -> Result<InputField, FileInputError> {
        let input_file = File::open(&self.file_path).map_err(|_| { FileInputError::InvalidFile })?;

        // The decoder is a build for reader and can be used to set various decoding options
        // via `Transformations`. The default export transformation is `Transformations::EXPAND
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

        println!("source png: {}", self.file_path);
        println!("size (w/h): {} * {}", info.width, info.height);
        println!("line_size (bytes): {}", info.line_size);
        println!("size (bytes): {}", info.buffer_size());
        println!("color type: {:?}", info.color_type); // Grayscale, RGB, Indexed, GrayscaleAlpha, RGBA
        println!("bit depth: {:?}", info.bit_depth); // One, Two, Four, Eight, Sixteen

        // Allocate the export buffer.
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

        let source = InputField::from(ByteInputData::new(output_buffer, 127, info.width, info.height));

        Ok(source)
    }
}

impl Input for PngInput {
    fn source_field(&self) -> Result<InputField, InputError> {
        let source = self.get_source_from_png_file_input()?;
        Ok(source)
//        match res {
//            Ok(sourcefield) => Some(sourcefield),
//            Err(_) => None
//        }
    }
}

impl From<FileInputError> for InputError {
    fn from(err: FileInputError) -> Self {
        InputError::InvalidInput { message: err.to_string() }
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


