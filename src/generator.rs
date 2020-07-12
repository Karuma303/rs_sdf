use std::result::Result::Err;

use crate::naive::*;
use crate::input::{FieldInput};
use crate::output::{FieldOutput};
use std::path::{PathBuf};

pub struct DistanceGenerator {
    input: Option<Box<dyn FieldInput>>,
    output: Option<Box<dyn FieldOutput<DistanceFieldType = u8>>>,
    strategy: GenerationStrategy,
    export_type: ExportType,
}

impl DistanceGenerator {
    pub fn new() -> DistanceGenerator {
        DistanceGenerator {
            input: None,
            output: None,
            strategy: GenerationStrategy::Naive, // default
            export_type: ExportType::UnsignedInnerOuterDistance,
        }
    }

    pub fn input(mut self, input: impl FieldInput + 'static) -> Self {
        self.input = Some(Box::new(input));
        self
    }

    pub fn output(mut self, output: impl FieldOutput<DistanceFieldType = u8> + 'static) -> Self {
        self.output = Some(Box::new(output));
        self
    }

    pub fn strategy(mut self, strategy: GenerationStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    pub fn export_type(mut self, export_type: ExportType) -> Self {
        self.export_type = export_type;
        self
    }

    pub fn generate(&self) -> Result<(), String> {
        // input path is set?
        if let Some(input) = &self.input {
            let source = input.get_source_field().unwrap();
            if let Some(output) = &self.output {
                let sdf = match self.export_type {
                    ExportType::UnsignedOuterDistance => {
                        generate_outer_df(&source)
                    }
                    ExportType::UnsignedInnerDistance => {
                        generate_inner_df(&source)
                    }

                    ExportType::UnsignedInnerOuterDistance => {
                        generate_combined_df(&source)
                    }
                };
                output.output(sdf);
            }

            // we should test and maybe micro-benchmark at least two known approaches here:
            // 1) brute force O(n²)
            // 2) the old EightPointSeqEuclideanDistTrans O(n)

            // Notes from old C# repository:

            // detect edges and mark them

            // next: make it signed
            // next: vector field

            // draw distance vectors
            // implement brute force / circle method  : https://github.com/chriscummings100/signeddistancefields/blob/master/Assets/SignedDistanceFields/SignedDistanceFieldGenerator.cs
        } else {
            return Err(String::from("no input path specified"));
        }
        Ok(())
    }
}

pub enum GenerationStrategy {
    Naive,
    BruteForceRectangular,
    BruteForceCircular,
}

pub enum ExportType {
    UnsignedInnerDistance,
    UnsignedOuterDistance,
    UnsignedInnerOuterDistance,
}

pub struct Configuration {}

#[cfg(test)]
mod tests {
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
            .input(r"assets\example_1_rgba_512x512.png")
            .output(outputPath).generate();
        let f = File::open(outputPath);
        assert!(f.is_ok(), "output file was not generated");
    }
     */
}