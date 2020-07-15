use crate::generator::{DistanceGenerator, GenerationStrategy, ExportType};
use crate::input::PngInput;
use crate::output::{PngOutput, ImageOutputChannels, ImageOutputChannelDepth};
use std::path::{Path, PathBuf};

mod df;
mod input;
mod source;
mod generator;
mod naive;
mod output;

const BASE_ASSET_PATH: &str = r"assets/";
const BASE_OUTPUT_PATH: &str = r"output/";

fn main() {

    // generate some distance field output images based on input images

    // TODO: remove
    // generate_sdf("test/test_rgba_2x2_checkerboard.png", "test_checkerboard.png");

    generate_sdf("example_1_rgba_512x512.png", "example_1_512x512.png");

    generate_sdf("example_2_rgba_512x512.png", "example_2_512x512.png");

    generate_sdf("example_3_rgba_512x512.png", "example_3_512x512.png");

    generate_sdf("example_4_rgba_512x512.png", "example_4_512x512.png");

    generate_sdf("example_6_rgba_16384x16384.png", "example_6_16384x16384.png");
}

fn generate_sdf(source_image_name: &str, target_image_name: &str) {
    let source_image_path = BASE_ASSET_PATH.to_owned().clone() + source_image_name;

    let target_image_path = BASE_OUTPUT_PATH.to_owned().clone() + "odf_" + target_image_name;

    let g = DistanceGenerator::new()
        .input(PngInput::new(&source_image_path))
        .output(PngOutput::new(&target_image_path,
                               ImageOutputChannels::One,
                               ImageOutputChannelDepth::Eight))
        .export_type(ExportType::UnsignedOuterDistance)
        .strategy(GenerationStrategy::Naive); // maybe rename to process_strategy

    let result = g.generate();
    display_result(&result, &source_image_path, &target_image_path);

    let target_image_path = BASE_OUTPUT_PATH.to_owned().clone() + "idf_" + target_image_name;
    let g = g.output(PngOutput::new(&target_image_path,
                                    ImageOutputChannels::One,
                                    ImageOutputChannelDepth::Eight))
        .export_type(ExportType::UnsignedInnerDistance);

    let result = g.generate();
    display_result(&result, &source_image_path, &target_image_path);

    let target_image_path = BASE_OUTPUT_PATH.to_owned().clone() + "cdf_" + target_image_name;
    let g = g.output(PngOutput::new(&target_image_path,
                                    ImageOutputChannels::One,
                                    ImageOutputChannelDepth::Eight))
        .export_type(ExportType::UnsignedInnerOuterDistance);

    let result = g.generate();
    display_result(&result, &source_image_path, &target_image_path);

    /* example for 16-bit / single channel output
    let target_image_path = BASE_OUTPUT_PATH.to_owned().clone() + "cdf_16_" + target_image_name;
    let g = g.output(PngOutput::new(&target_image_path,
                                    ImageOutputChannels::One,
                                    ImageOutputChannelDepth::Sixteen))
        .export_type(ExportType::UnsignedInnerOuterDistance);

    let result = g.generate();
    display_result(&result, &source_image_path, &target_image_path);
    */
}

fn display_result(result: &Result<(), String>, source_image_path: &String, target_image_path: &String) {
    match result {
        Ok(_) => {
            println!("generated sdf {} for input image {}", target_image_path, source_image_path);
        }
        Err(err) => {
            println!("error generating sdf: {}", err);
        }
    }
}
