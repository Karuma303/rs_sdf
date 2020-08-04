extern crate png;

use std::path::PathBuf;

use rs_sdf::generator::{DistanceGenerator};
use rs_sdf::input::image::PngInput;
use rs_sdf::export::image::{ImageOutputChannelDepth, ImageOutputChannels, PngOutput};
use rs_sdf::processor::sweep::EightSideSweepProcessor;
use rs_sdf::distance::DistanceLayer;
use rs_sdf::data::DistanceField;

const BASE_ASSET_FOLDER: &str = r"examples/assets";
const BASE_OUTPUT_FOLDER: &str = r"examples/output";

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
                 DistanceLayer::Foreground,
                 ImageOutputChannelDepth::Eight,
                 ImageOutputChannels::One);

    // 1.2. Export PNG with 8-bit outer distance
    generate_sdf("example_1_rgba_512x512.png",
                 "example_1_512x512.png",
                 DistanceLayer::Background,
                 ImageOutputChannelDepth::Eight,
                 ImageOutputChannels::One);

    // 1.3. Export PNG with 8-bit inner and outer distance (distances added in one channel)
    generate_sdf("example_1_rgba_512x512.png",
                 "example_1_512x512.png",
                 DistanceLayer::Combined,
                 ImageOutputChannelDepth::Eight,
                 ImageOutputChannels::One);

    // 2. Demonstrate single- and dual-channel export

    // 2.1. Export PNG with inner and outer distance added together in one 8-bit channel
    generate_sdf("example_2_rgba_512x512.png",
                 "example_2_512x512.png",
                 DistanceLayer::Combined,
                 ImageOutputChannelDepth::Eight,
                 ImageOutputChannels::One);

    // 2.2. Export PNG with inner and outer distance separated to two 8-bit channels
    generate_sdf("example_2_rgba_512x512.png",
                 "example_2_512x512_2_channel.png",
                 DistanceLayer::Combined,
                 ImageOutputChannelDepth::Eight,
                 ImageOutputChannels::Two);


    // another example...
    generate_sdf("example_8_rgba_512x512.png",
                 "example_8_512x512.png",
                 DistanceLayer::Combined,
                 ImageOutputChannelDepth::Eight,
                 ImageOutputChannels::One);

    generate_sdf("example_10_rgba_3100x900.png",
                 "example_10_3100x900.png",
                 DistanceLayer::Background,
                 ImageOutputChannelDepth::Eight,
                 ImageOutputChannels::One);


    /*


    generate_sdf("example_3_rgba_512x512.png", "example_3_512x512.png", ExportType::UnsignedInnerOuterDistance,ImageOutputChannels::One);

    generate_sdf("example_4_rgba_512x512.png", "example_4_512x512.png", ExportType::UnsignedInnerOuterDistance, ImageOutputChannels::Two);
    */

    // very big example
    // generate_sdf("example_6_rgba_16384x16384.png", "example_6_16384x16384.png", &ExportType::UnsignedInnerOuterDistance);
}


fn generate_sdf(source_image_name: &str,
                target_image_name: &str,
                export_type: DistanceLayer,
                bit_depth: ImageOutputChannelDepth,
                num_channels: ImageOutputChannels) {
    let mut image_path_buff = PathBuf::new();
    image_path_buff.push(BASE_ASSET_FOLDER);
    image_path_buff.push(source_image_name);

    let source_image_path = image_path_buff.into_os_string().into_string().unwrap();

    let mut prefixes: Vec<String> = Vec::new();
    prefixes.push(get_type_prefix(&export_type));
    prefixes.push(get_num_channels_prefix(&num_channels));
    prefixes.push(get_bit_depth_prefix(&bit_depth));

    let prefix: String = prefixes.join("_");

    let target_image_path = get_output_image_file_path(target_image_name,
                                                       prefix.as_str());

    let g = DistanceGenerator::new()
        .input(PngInput::new(&source_image_path))
        .output(PngOutput::new(&target_image_path,
                               num_channels,
                               bit_depth))
        .export_filter(export_type)
        .processor(EightSideSweepProcessor {});

    let result = g.generate();
    display_result(&result, &source_image_path, &target_image_path);


    // let input : DistanceInput = PngInput::new(&source_image_path);
    // let builder : DistanceFieldBuilder = DistanceFieldBuilder::new(input);
    // let builder : DistanceFieldBuilder = DistanceFieldBuilder::from(input);
    // let builder : DistanceFieldBuilder = input::into::<DistanceFieldBuilder>();
    // let df : DistanceField = builder::build(EightSidedSweeping); // distance calculation method
    // let dt : DistanceTransformation = df.filter(InnerDistance).transform(Cartesian).scale(0.5f);

    // DistanceTransformation provides n-channels with distances (bit depth?)

    // let output : DistanceOutput = PngOutput::new(dt, ImageOutputChannelConfiguration::new(...));
    // output.save();


    // short:
    // PngOutput::new(
    //          DistanceFieldBuilder::new(
    //                  PngInput::new(&source_image_path))
    //          ::build(EightSidedSweep)
    //          ::transform(Cartesian))
    // .save();

    // let df = DistanceFieldBuilder
    //                  ::from(PngInput::new(&source_image_path)
    //                  ::build(EightSidedSweep);
}

fn get_output_image_file_path(filename: &str, prefix: &str) -> String {
    let mut file_path_buff = PathBuf::new();
    file_path_buff.push(BASE_OUTPUT_FOLDER);
    file_path_buff.push(prefix.to_owned() + "_" + filename);
    file_path_buff.into_os_string().into_string().unwrap()
}

fn get_type_prefix(export_type: &DistanceLayer) -> String {
    match export_type {
        DistanceLayer::Foreground => String::from("idf"),
        DistanceLayer::Background => String::from("odf"),
        DistanceLayer::Combined => String::from("cdf"),
    }
}

fn get_bit_depth_prefix(channel_depth: &ImageOutputChannelDepth) -> String {
    match channel_depth {
        ImageOutputChannelDepth::Eight => String::from("8bit"),
        ImageOutputChannelDepth::Sixteen => String::from("16bit"),
        ImageOutputChannelDepth::ThirtyTwo => String::from("32bit"),
    }
}

fn get_num_channels_prefix(num_channels: &ImageOutputChannels) -> String {
    match num_channels {
        ImageOutputChannels::One => String::from("1chan"),
        ImageOutputChannels::Two => String::from("2chan"),
    }
}

fn display_result(result: &Result<(), String>, source_image_path: &String, target_image_path: &String) {
    match result {
        Ok(_) =>
            println!("generated sdf {} for input image {}", target_image_path, source_image_path),
        Err(err) =>
            println!("error generating sdf: {}", err),
    }
}
