extern crate png;

use std::path::PathBuf;

use rs_sdf::distance::{DistanceLayer, DistanceType};
use rs_sdf::export::image::{ImageOutputChannelDepth, PngOutput, write_the_final_solution};
use rs_sdf::generator::DistanceGenerator;
use rs_sdf::input::DistanceInput;
use rs_sdf::input::image::PngInput;
use rs_sdf::processor::sweep::EightSideSweepProcessor;
use rs_sdf::data::builder::DistanceFieldBuilder;
use rs_sdf::data::DistanceField;
use rs_sdf::processor::Processor;
use rs_sdf::data::transformation::{DistanceTransformation, TransformationResult, TransformOutputGenerator};
use rs_sdf::data::output::TransformationOutputWriter;

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
                 1);

    // 1.2. Export PNG with 8-bit outer distance
    generate_sdf("example_1_rgba_512x512.png",
                 "example_1_512x512.png",
                 DistanceLayer::Background,
                 ImageOutputChannelDepth::Eight,
                 1);

    // 1.3. Export PNG with 8-bit inner and outer distance (distances added in one channel)
    generate_sdf("example_1_rgba_512x512.png",
                 "example_1_512x512.png",
                 DistanceLayer::Combined,
                 ImageOutputChannelDepth::Eight,
                 1);

    // 2. Demonstrate single- and dual-channel export

    // 2.1. Export PNG with inner and outer distance added together in one 8-bit channel
    generate_sdf("example_2_rgba_512x512.png",
                 "example_2_512x512.png",
                 DistanceLayer::Combined,
                 ImageOutputChannelDepth::Eight,
                 1);

    // 2.2. Export PNG with inner and outer distance separated to two 8-bit channels
    generate_sdf("example_2_rgba_512x512.png",
                 "example_2_512x512_2_channel.png",
                 DistanceLayer::Combined,
                 ImageOutputChannelDepth::Eight,
                 2);


    // another example...
    generate_sdf("example_8_rgba_512x512.png",
                 "example_8_512x512.png",
                 DistanceLayer::Combined,
                 ImageOutputChannelDepth::Eight,
                 1);

    generate_sdf("example_10_rgba_3100x900.png",
                 "example_10_3100x900.png",
                 DistanceLayer::Background,
                 ImageOutputChannelDepth::Eight,
                 1);


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
                num_channels: u8) {
    let mut image_path_buff = PathBuf::new();
    image_path_buff.push(BASE_ASSET_FOLDER);
    image_path_buff.push(source_image_name);

    let source_image_path = image_path_buff.into_os_string().into_string().unwrap();

    let mut prefixes: Vec<String> = Vec::new();
    prefixes.push(get_type_prefix(&export_type));
    prefixes.push(get_num_channels_prefix(num_channels));
    prefixes.push(get_bit_depth_prefix(&bit_depth));

    let prefix: String = prefixes.join("_");

    let target_image_path = get_output_image_file_path(target_image_name,
                                                       prefix.as_str());

    let mut output_writer = PngOutput::new(&target_image_path);

    let g: DistanceGenerator = DistanceGenerator::new()
        .input(PngInput::new(&source_image_path))
        .output(output_writer)
        .export_filter(export_type)
        .processor(EightSideSweepProcessor {});

    let result = g.generate();
    display_result(&result, &source_image_path, &target_image_path);

    // new methods

    let input: Box<dyn DistanceInput> = Box::new(PngInput::new(&source_image_path));
    let builder: DistanceFieldBuilder = DistanceFieldBuilder::new(input);

    let input_2 = PngInput::new(&source_image_path);
    let builder_2 = DistanceFieldBuilder::from(input_2);

    let df: DistanceField = builder_2.build(Processor::from(EightSideSweepProcessor {}));
    let builder_3: DistanceFieldBuilder = PngInput::new(&source_image_path).into(); // works too !

    // let builder_2 = DistanceFieldBuilder::new(PngInput::new(&source_image_path));

    // let input : DistanceInput = PngInput::new(&source_image_path); X (ist kein DistanceInput sondern PngInput!)
    // let builder : DistanceFieldBuilder = DistanceFieldBuilder::new(input); X (geht nur mit box)
    // let builder : DistanceFieldBuilder = DistanceFieldBuilder::from(input); ! (geht)
    // let builder : DistanceFieldBuilder = input::into::<DistanceFieldBuilder>();
    // let df : DistanceField = builder::build(EightSidedSweeping); // distance calculation method
    // let dt : DistanceTransformation = df.filter(InnerDistance).transform(Cartesian).scale(0.5f);

    // DistanceTransformation provides n-channels with distances (bit depth?)
    // let dt : DistanceTransformation = df.transformation();
    let mut dt: DistanceTransformation = DistanceTransformation::from(df);
    dt.filter(DistanceLayer::Foreground); // TODO: rename to inner/outer/combined
    dt.distance_type(DistanceType::EuclideanDistance);
    dt.scale(0.9); // u8 -> 0 = orig, 1 = 2^1 = orig / 2, 2 = 2^2 = orig / 4, etc...

    // to generate the transformation result:
    // #1 (turbofish)
    // let res = dt.transform::<u8>();
    // let res = dt.transform::<u16>();
    // #2 (explicit type)
    // let res : TransformationResult<u8> = dt.transform();
    // let res : TransformationResult<u16> = dt.transform();

    // new
    let trans_res: TransformationResult<u8> = dt.transform(); // -> u8
    let trans_res_u16: TransformationResult<u16> = dt.transform(); // -> u8

    // old
    // let trans_res = dt.transform::<u8>();

    let mut output = PngOutput::new(&target_image_path);

    // TODO: das hier sollte gehene
    // output.write(trans_res);

    // stattdessen machen wir das hier
    write_the_final_solution(&trans_res);

    // hier muss es weiter gehen...
    // TODO: reactivate
    //   output.write(transformation_result);

    // short:
    // PngOutput::new(
    //          DistanceFieldBuilder::new(
    //                  PngInput::new(&source_image_path))
    //          .build(EightSidedSweep)
    //          .transform(Cartesian))
    // .write();

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

fn get_num_channels_prefix(num_channels: u8) -> String {
    format!("{}c", num_channels)
}

fn display_result(result: &Result<(), String>, source_image_path: &str, target_image_path: &str) {
    match result {
        Ok(_) =>
            println!("generated sdf {} for input image {}", target_image_path, source_image_path),
        Err(err) =>
            println!("error generating sdf: {}", err),
    }
}
