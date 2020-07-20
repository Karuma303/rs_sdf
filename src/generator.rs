use std::result::Result::Err;

use crate::distance_field::{DistanceField, SourceProcessor};
use crate::import::FieldInput;
use crate::export::DistanceFieldExporter;

pub struct DistanceGenerator {
    input: Option<Box<dyn FieldInput>>,
    output: Option<Box<dyn DistanceFieldExporter>>,
    processor: Option<Box<dyn SourceProcessor>>,
    export_type: ExportType,
}

impl DistanceGenerator {
    pub fn new() -> DistanceGenerator {
        DistanceGenerator {
            input: None,
            output: None,
            processor: None,
            export_type: ExportType::UnsignedInnerOuterDistance,
        }
    }

    pub fn input(mut self, input: impl FieldInput + 'static) -> Self {
        self.input = Some(Box::new(input));
        self
    }

    pub fn output(mut self, output: impl DistanceFieldExporter + 'static) -> Self {
        self.output = Some(Box::new(output));
        self
    }

    pub fn processor(mut self, processor: impl SourceProcessor + 'static) -> Self {
        self.processor = Some(Box::new(processor));
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

            if let Some(processor) = &self.processor {
                let mut df = processor.process(&source);

                if let Some(output) = &self.output {

                    match self.export_type {
                        ExportType::UnsignedOuterDistance => {
                            df = DistanceField::filter_outer(&df);
                        }
                        ExportType::UnsignedInnerDistance => {
                            df = DistanceField::filter_inner(&df)
                        }
                        ExportType::UnsignedInnerOuterDistance => {}
                    };
                    output.export(&df);
                } else {
                    panic!("no export file specified");
                }
            } else {
                panic!("no processor specified");
            }

            // we should tests and maybe micro-benchmark at least two known approaches here:
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

#[derive(Clone)]
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
    #[tests]
    fn generates_output_file() {
        let outputPath = r"export\test_output.png";
        let res = DistanceGenerator::new()
            .input(r"tests\example_1_rgba_512x512.png")
            .export(outputPath).generate();
        let f = File::open(outputPath);
        assert!(f.is_ok(), "export file was not generated");
    }
     */
}