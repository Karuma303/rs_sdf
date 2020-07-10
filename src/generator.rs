use std::result::Result::Err;

use crate::naive::*;
use crate::input::get_source_from_png_file_input;
use crate::output::PngExporter;
use std::path::{PathBuf};
use crate::source::SourceField;

pub struct DistanceGenerator {
    input_path: Option<String>,
    output_path: Option<String>,
    strategy: GenerationStrategy,
    export_type: ExportType,
}

impl DistanceGenerator {
    pub fn new() -> DistanceGenerator {
        DistanceGenerator {
            input_path: None,
            output_path: None,
            strategy: GenerationStrategy::Naive, // default
            export_type: ExportType::SignedInnerOuterDistance,
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

    pub fn export_type(mut self, export_type: ExportType) -> Self {
        self.export_type = export_type;
        self
    }

    pub fn generate(&self) -> Result<(), String> {
        // input path is set?
        if let Some(path) = &self.input_path {
            let source = get_source_from_png_file_input(path);
            // let sdf = generate_sdf(&source.unwrap());
            // let sdf = generate_signed_df(&source.unwrap());

            match source {
                Ok(sourceField) => {
                    // generate_outer_df(&source.unwrap());
                    if let Some(path) = &self.output_path {
                        match self.export_type {
                            ExportType::UnsignedOuterDistance => {
                                let sdf =
                                    generate_outer_df(&sourceField);
                                sdf.export(&PathBuf::from(path));
                            }
                            ExportType::UnsignedInnerDistance => {
                                let sdf =
                                    generate_inner_df(&sourceField);
                                sdf.export(&PathBuf::from(path));
                            }

                            ExportType::SignedInnerOuterDistance => {
                                let sdf =
                                    generate_signed_df(&sourceField);
                                sdf.export(&PathBuf::from(path));
                            }
                        };
                    }
                }
                Err(e) => {
                    let msg = format!("Invalid input file! {}", e);
                    return Err(msg);
                }
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
    SignedInnerOuterDistance,
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