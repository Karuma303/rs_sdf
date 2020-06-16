use std::result::Result::Err;

use crate::naive::generate_sdf;
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

            // TODO: generate output file here


            // let x = Decoder::new_with_limits()

            // TODO: process data according to strategy
            // TODO: write file


//                let mut buffer = vec![0; info.buffer_size()];


            // we should test and maybe microbenchmark at least two known apporches here:
            // 1) brute force O(n²)
            // 2) the old EightPointSeqEuclideanDistTrans O(n)


// Notes from old C# repository:

// edges detecten und markieren

// next: make it signed
// next: vector field

// vectoren zeichnen
// später: brute force circle methode auch mal ausprobieren : https://github.com/chriscummings100/signeddistancefields/blob/master/Assets/SignedDistanceFields/SignedDistanceFieldGenerator.cs


// https://github.com/chriscummings100/signeddistancefields
// https://shaderfun.com/
// https://shaderfun.com/2018/03/23/signed-distance-fields-part-1-unsigned-distance-fields/


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