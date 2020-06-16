use std::fs::File;
use std::result::Result::Err;

use png::Decoder;
use png::Transformations;

use crate::naive::generate_sdf;
use crate::source::SourceField;
use crate::input::get_source_from_png_file_input;

pub struct DistanceGenerator {
    input_path: Option<String>,
    output_path: Option<String>,
    strategy: GenerationStrategy,
}

impl DistanceGenerator {
    pub fn new() -> DistanceGenerator {
        DistanceGenerator {
            input_path: None,
            output_path: None,
            strategy: GenerationStrategy::Naive, // default
        }
    }

    pub fn input(mut self, path: &str) -> Self {
        self.input_path = Some(String::from(path));
        self
    }

    pub fn output(mut self, path: &str) -> Self {
        self.output_path = Some(String::from(path));
        self
    }

    pub fn strategy(mut self, strategy: GenerationStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    pub fn generate(&self) -> Result<(), &str> {
        // input path is set?
        if let Some(path) = &self.input_path {

            let source = get_source_from_png_file_input(path);
            let sdf = generate_sdf(&source.unwrap());
            //
            // // moved to input.rs
            // if let Ok(file) = File::open(path) {
            //     // NOTE: There's also a new_with_limits() constructor !
            //
            //     // The decoder is a build for reader and can be used to set various decoding options
            //     // via `Transformations`. The default output transformation is `Transformations::EXPAND
            //     // | Transformations::STRIP_ALPHA`.
            //
            //     let mut d = Decoder::new(file);
            //     d.set_transformations(Transformations::IDENTITY);
            //
            //     if let Ok(read_info) = d.read_info() {
            //         let info = read_info.0;
            //         let mut reader = read_info.1;
            //         println!("source png: {}", path);
            //         println!("size (w/h): {} * {}", info.width, info.height);
            //         println!("line_size (bytes): {}", info.line_size);
            //         println!("size (bytes): {}", info.buffer_size());
            //         println!("color type: {:?}", info.color_type); // Grayscale, RGB, Indexed, GrayscaleAlpha, RGBA
            //         println!("bit depth: {:?}", info.bit_depth); // One, Two, Four, Eight, Sixteen
            //
            //         // Allocate the output buffer.
            //         let mut buffer = vec![0; info.buffer_size()];
            //         // Read the next frame. Currently this function should only called once.
            //         // The default options
            //         reader.next_frame(&mut buffer).unwrap();
            //
            //
            //         // the source is RGBA (8 bit per channel)
            //         // in this case, we just take a look at the 8-bit alpha channel
            //         let mut alpha_buffer = vec![0u8; info.buffer_size() / 4];
            //         let mut index = 0;
            //         for mut x in &alpha_buffer {
            //             // x = &buffer[index];
            //             buffer[index] = 10;
            //             index += 4;
            //         }
            //
            //         let field = SourceField::new(&alpha_buffer, info.width, info.height);
            //         let sdf = generate_sdf(&field);

                    // TODO: generate output file here


            //         // test
            //     } else {
            //         return Err("given input file is not decodable");
            //     }
            // } else {
            //     return Err("not a valid file path");
            // }

            // let x = Decoder::new_with_limits()

            // TODO: process data according to strategy
            // TODO: write file

//                let (info, mut reader) = d.read_info().unwrap();
            // Allocate the output buffer.
//                let mut buffer = vec![0; info.buffer_size()];
        } else {
            return Err("no input path specified");
        }
        Ok(())
    }
}

pub enum GenerationStrategy {
    Naive,
    BruteForceRectangular,
    BruteForceCircular,
}

pub struct Configuration {}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::generator::DistanceGenerator;

    #[test]
    fn no_input_path() {
        let gen = DistanceGenerator::new();
        assert!(gen.generate().is_err(), "non existing input path should generate an error");
    }

    // moved to input.rs
    #[test]
    fn input_file_is_valid_rgba() {
        let gen = DistanceGenerator::new()
            .input(r"assets\SDF_Test_Texture_RGBA.png");
        assert!(gen.generate().is_ok());
    }

    /*
    #[test]
    fn generates_output_file() {
        let outputPath = r"output\test_output.png";
        let res = DistanceGenerator::new()
            .input(r"assets\SDF_Test_Texture_RGBA.png")
            .output(outputPath).generate();
        let f = File::open(outputPath);
        assert!(f.is_ok(), "output file was not generated");
    }
     */
}