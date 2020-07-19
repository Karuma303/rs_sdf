extern crate png;

use std::path::PathBuf;

use rs_sdf_gen::generator::{DistanceGenerator, ExportType};
use rs_sdf_gen::input::PngInput;
use rs_sdf_gen::output::{ImageOutputChannelDepth, ImageOutputChannels, PngOutput};
use rs_sdf_gen::processor::sweep::EightSideSweepProcessor;

const BASE_ASSET_FOLDER: &str = r"assets";
const BASE_OUTPUT_FOLDER: &str = r"output";

///
/// Examples that show how to generate distance fields based on input images
///
/// Run with:
///
/// ´cargo run --example images´
///
fn main() {
    /*
    generate_sdf("example_2_rgba_512x512.png",
                 "example_2_512x512_2_channel.png",
    */

    generate_sdf("example_8_rgba_512x512.png",
                 "example_8_512x512.png",
                 ExportType::UnsignedInnerOuterDistance,
                 ImageOutputChannels::One);

    /*
    generate_sdf("example_1_rgba_512x512.png", "example_1_512x512.png", ExportType::UnsignedInnerDistance, ImageOutputChannels::One);
    generate_sdf("example_1_rgba_512x512.png", "example_1_512x512.png", ExportType::UnsignedOuterDistance,ImageOutputChannels::One);
    generate_sdf("example_1_rgba_512x512.png", "example_1_512x512.png", ExportType::UnsignedInnerOuterDistance,ImageOutputChannels::One);

    generate_sdf("example_2_rgba_512x512.png", "example_2_512x512.png", ExportType::UnsignedInnerOuterDistance,ImageOutputChannels::One);

    generate_sdf("example_3_rgba_512x512.png", "example_3_512x512.png", ExportType::UnsignedInnerOuterDistance,ImageOutputChannels::One);

    generate_sdf("example_4_rgba_512x512.png", "example_4_512x512.png", ExportType::UnsignedInnerOuterDistance, ImageOutputChannels::Two);
    */

    // very big example
    // generate_sdf("example_6_rgba_16384x16384.png", "example_6_16384x16384.png", &ExportType::UnsignedInnerOuterDistance);
}


fn generate_sdf(source_image_name: &str, target_image_name: &str, export_type: ExportType, num_channels: ImageOutputChannels) {
    let mut image_path_buff = PathBuf::new();
    image_path_buff.push(BASE_ASSET_FOLDER);
    image_path_buff.push(source_image_name);

    let source_image_path = image_path_buff.into_os_string().into_string().unwrap();

    let prefix: String = match export_type {
        ExportType::UnsignedInnerDistance => String::from("idf"),
        ExportType::UnsignedOuterDistance => String::from("odf"),
        ExportType::UnsignedInnerOuterDistance => String::from("cdf"),
    };

    let target_image_path = get_output_image_file_path(target_image_name, &prefix);

    // let target_image_path = BASE_OUTPUT_FOLDER.to_owned().clone() + "odf_" + target_image_name;

    let g = DistanceGenerator::new()
        .input(PngInput::new(&source_image_path))
        .output(PngOutput::new(&target_image_path,
                               num_channels,
                               ImageOutputChannelDepth::Eight))
        .export_type(export_type)
        .processor(EightSideSweepProcessor {});

    let result = g.generate();
    display_result(&result, &source_image_path, &target_image_path);
    /*
        // let target_image_path = BASE_OUTPUT_PATH.to_owned().clone() + "idf_" + target_image_name;
        let target_image_path = get_output_image_file_path(target_image_name, "idf");

        let g = g.output(PngOutput::new(&target_image_path,
                                        ImageOutputChannels::One,
                                        ImageOutputChannelDepth::Eight))
            .export_type(ExportType::UnsignedInnerDistance);

        let result = g.generate();
        display_result(&result, &source_image_path, &target_image_path);

        // let target_image_path = BASE_OUTPUT_PATH.to_owned().clone() + "cdf_" + target_image_name;
        let target_image_path = get_output_image_file_path(target_image_name, "cdf");
        let g = g.output(PngOutput::new(&target_image_path,
                                        ImageOutputChannels::One,
                                        ImageOutputChannelDepth::Eight))
            .export_type(ExportType::UnsignedInnerOuterDistance);

        let result = g.generate();
        display_result(&result, &source_image_path, &target_image_path);
    */
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

fn get_output_image_file_path(filename: &str, prefix: &str) -> String {
    let mut file_path_buff = PathBuf::new();
    file_path_buff.push(BASE_OUTPUT_FOLDER);
    file_path_buff.push(prefix.to_owned() + "_" + filename);
    file_path_buff.into_os_string().into_string().unwrap()
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
