use std::result::Result::Err;

use crate::input::Input;
use crate::export::{DistanceFieldExporter};
use crate::processor::SourceProcessor;
use crate::distance::{DistanceType, DistanceLayer};

pub struct DistanceGenerator {
    input: Option<Box<dyn Input>>,
    output: Option<Box<dyn DistanceFieldExporter>>,
    processor: Option<Box<dyn SourceProcessor>>,
    distance_type: DistanceType,
    distance_layer: DistanceLayer,
}

impl DistanceGenerator {
    pub fn new() -> DistanceGenerator {
        DistanceGenerator {
            input: None,
            output: None,
            processor: None,
            distance_type: DistanceType::EuclideanDistance,
            distance_layer: DistanceLayer::Combined,
        }
    }

    pub fn input(mut self, input: impl Input + 'static) -> Self {
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

    pub fn export_filter(mut self, export_selection: DistanceLayer) -> Self {
        self.distance_layer = export_selection;
        self
    }

    pub fn distance_type(mut self, export_type: DistanceType) -> Self {
        self.distance_type = export_type;
        self
    }

    pub fn generate(&self) -> Result<(), String> {
        // input path is set?
        if let Some(input) = &self.input {

            // TODO: add matching here !!!
            let source = input.source_field().ok().unwrap(); //unwrap();

            if let Some(processor) = &self.processor {
                let df = processor.process(&source);

                if let Some(output) = &self.output {
                    output.export(&df, &self.distance_type, &self.distance_layer);
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