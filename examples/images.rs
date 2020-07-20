extern crate png;

use std::path::PathBuf;

use rs_sdf_gen::generator::{DistanceGenerator, ExportType};
use rs_sdf_gen::input::PngInput;
use rs_sdf_gen::output::image::{ImageOutputChannelDepth, ImageOutputChannels, PngOutput};
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

    // 1. Demonstrate different export types for the same input image

    // 1.1. Export PNG with 8-bit inner distance
    generate_sdf("example_1_rgba_512x512.png",
                 "example_1_512x512.png",
                 ExportType::UnsignedInnerDistance,
                 ImageOutputChannels::One);

    // 1.2. Export PNG with 8-bit outer distance
    generate_sdf("example_1_rgba_512x512.png",
                 "example_1_512x512.png",
                 ExportType::UnsignedOuterDistance,
                 ImageOutputChannels::One);

    // 1.3. Export PNG with 8-bit inner and outer distance (distances added in one channel)
    generate_sdf("example_1_rgba_512x512.png",
                 "example_1_512x512.png",
                 ExportType::UnsignedInnerOuterDistance,
                 ImageOutputChannels::One);

    // 2. Demonstrate single- and dual-channel export

    // 2.1. Export PNG with inner and outer distance added together in one 8-bit channel
    generate_sdf("example_2_rgba_512x512.png",
                 "example_2_512x512.png",
                 ExportType::UnsignedInnerOuterDistance,
                 ImageOutputChannels::One);

    // 2.2. Export PNG with inner and outer distance separated to two 8-bit channels
    generate_sdf("example_2_rgba_512x512.png",
                 "example_2_512x512_2_channel.png",
                 ExportType::UnsignedInnerOuterDistance,
                 ImageOutputChannels::Two);


    // another example...
    generate_sdf("example_8_rgba_512x512.png",
                 "example_8_512x512.png",
                 ExportType::UnsignedInnerOuterDistance,
                 ImageOutputChannels::One);

    /*


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

    let g = DistanceGenerator::new()
        .input(PngInput::new(&source_image_path))
        .output(PngOutput::new(&target_image_path,
                               num_channels,
                               ImageOutputChannelDepth::Eight))
        .export_type(export_type)
        .processor(EightSideSweepProcessor {});

    let result = g.generate();
    display_result(&result, &source_image_path, &target_image_path);
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
