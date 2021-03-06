extern crate png;

use std::path::PathBuf;

use rs_sdf::distance::{DistanceLayer, DistanceType};
use rs_sdf::export::image::{PngOutput, ImageFileWriter, DistanceTransformationResultWriter};
use rs_sdf::generator::DistanceGenerator;
use rs_sdf::input::image::PngInput;
use rs_sdf::processor::sweep::EightSideSweepProcessor;
use rs_sdf::data::DistanceField;
use rs_sdf::processor::Processor;
use rs_sdf::data::transformation::{DistanceTransformation, TransformationResult, TransformOutputGenerator};
use rs_sdf::data::builder::DistanceFieldBuilder;
use rs_sdf::result::{DistanceTransformationResult, ChannelBitDepth};

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

	//
	// Demonstrate different export types for the same input image
	//

	// Export PNG with 8-bit inner euclidean distance
	generate_sdf("example_1_rgba_512x512.png",
				 "example_1_512x512.png",
				 DistanceLayer::Foreground,
				 DistanceType::EuclideanDistance,
				 ChannelBitDepth::Eight);

	// Export PNG with 8-bit outer euclidean distance
	generate_sdf("example_1_rgba_512x512.png",
				 "example_1_512x512.png",
				 DistanceLayer::Background,
				 DistanceType::EuclideanDistance,
				 ChannelBitDepth::Eight);

	// Export PNG with 8-bit inner and outer euclidean distance (distances added in one channel)
	generate_sdf("example_1_rgba_512x512.png",
				 "example_1_512x512.png",
				 DistanceLayer::Combined,
				 DistanceType::EuclideanDistance,
				 ChannelBitDepth::Eight);

	// Export PNG with 8-bit inner and outer cartesian distance (distances added in one channel)
	/*
	generate_sdf("example_1_rgba_512x512.png",
				 "example_1_512x512.png",
				 DistanceLayer::Combined,
				 DistanceType::CartesianDistance,
				 ImageOutputChannelDepth::Eight);
	 */

	// Demonstrate single- and dual-channel export

	// 2.1. Export PNG with inner and outer euclidean distance added together in one 8-bit channel
	/*
	generate_sdf("example_2_rgba_512x512.png",
				 "example_2_512x512.png",
				 DistanceLayer::Combined,
				 DistanceType::EuclideanDistance,
				 ImageOutputChannelDepth::Eight);
	 */

	// 2.2. Export PNG with inner and outer euclidean distance separated to two 8-bit channels
	/*
	generate_sdf("example_2_rgba_512x512.png",
				 "example_2_512x512_2_channel.png",
				 DistanceLayer::Combined,
				 DistanceType::EuclideanDistance,
				 ImageOutputChannelDepth::Eight);
	*/

	// another example...
	/*
	generate_sdf("example_8_rgba_512x512.png",
				 "example_8_512x512.png",
				 DistanceLayer::Combined,
				 DistanceType::EuclideanDistance,
				 ImageOutputChannelDepth::Eight);
	 */

	/*
	generate_sdf("example_10_rgba_3100x900.png",
				 "example_10_3100x900.png",
				 DistanceLayer::Background,
				 DistanceType::EuclideanDistance,
				 ImageOutputChannelDepth::Eight);
	 */

	// very big example
	/*
	generate_sdf("example_6_rgba_16384x16384.png",
				 "example_6_16384x16384.png",
				 DistanceLayer::Combined,
				 DistanceType::EuclideanDistance,
				 ImageOutputChannelDepth::Sixteen);
	 */
}

fn generate_sdf(source_image_name: &str,
				target_image_name: &str,
				layer: DistanceLayer,
				distance_type: DistanceType,
				bit_depth: ChannelBitDepth) {
	let source_image_path = get_input_image_path(source_image_name);

	let num_channels = distance_type.dimensions();

	let target_image_path = generate_target_image_name("generator",
													   &target_image_name,
													   &layer,
													   &distance_type,
													   num_channels,
													   &bit_depth);

	// convenient way with the DistanceGenerator
	generate_with_distance_generator(&source_image_path, &target_image_path, &layer, &distance_type);

	let target_image_path = generate_target_image_name("builder",
													   &target_image_name,
													   &layer,
													   &distance_type,
													   num_channels,
													   &bit_depth);

	// the more flexible way with the DistanceFieldBuilder
	generate_with_distance_field_builder(&source_image_path, &target_image_path, &layer, &distance_type);
}

fn generate_with_distance_generator(source_image_path: &str,
									target_image_path: &str,
									layer: &DistanceLayer,
									distance_type: &DistanceType) {
	let g: DistanceGenerator = DistanceGenerator::new()
		.input(PngInput::new(&source_image_path))
		.output(PngOutput::new(&target_image_path))
		.export_filter(*layer)
		.distance_type(*distance_type)
		.processor(EightSideSweepProcessor {});

	// deactivated for now
	//	let result = g.generate();
	//	display_result(&result, &source_image_path, &target_image_path);
}

fn generate_with_distance_field_builder(source_image_path: &str,
										target_image_path: &str,
										layer: &DistanceLayer,
										distance_type: &DistanceType) {
	let input = PngInput::new(&source_image_path);
	let builder = DistanceFieldBuilder::from(input);

	let df: DistanceField = builder.build(Processor::from(EightSideSweepProcessor {}));

	// let b : DistanceFieldBuilder = PngInput::new(&source_image_path).into(); // works too !
	// let b = DistanceFieldBuilder::new(PngInput::new(&source_image_path));

	// let input : DistanceInput = PngInput::new(&source_image_path); X (ist kein DistanceInput sondern PngInput!)
	// let builder : DistanceFieldBuilder = DistanceFieldBuilder::new(input); X (geht nur mit box)
	// let builder : DistanceFieldBuilder = DistanceFieldBuilder::from(input); ! (geht)
	// let builder : DistanceFieldBuilder = input::into::<DistanceFieldBuilder>();
	// let df : DistanceField = builder::build(EightSidedSweeping); // distance calculation method
	// let dt : DistanceTransformation = df.filter(InnerDistance).transform(Cartesian).scale(0.5f);

	// DistanceTransformation provides n-channels with distances (bit depth?)
	// let dt : DistanceTransformation = df.transformation();
	let mut dt: DistanceTransformation = DistanceTransformation::from(df);
	dt.filter(*layer);
	dt.distance_type(*distance_type);
	dt.scale(0.9); // u8 -> 0 = orig, 1 = 2^1 = orig / 2, 2 = 2^2 = orig / 4, etc...

	// to generate the transformation result:
	// #1 (turbofish)
	// let res = dt.transform::<u8>();
	// let res = dt.transform::<u16>();
	// #2 (explicit type)
	// let res : TransformationResult<u8> = dt.transform();
	// let res : TransformationResult<u16> = dt.transform();


	// very new - with non-generic TransformationResult
	let res : DistanceTransformationResult = dt.result();
	let output = PngOutput::new(&target_image_path);
	output.write_result(res); // TODO
	// end of very new ;)

	// new
	let trans_res: TransformationResult<u8> = dt.transform(); // -> u8
	let trans_res_u16: TransformationResult<u16> = dt.transform(); // -> u8

	// old
	// let trans_res = dt.transform::<u8>();

	let output = PngOutput::new(&target_image_path);
	output.write(&trans_res);





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


fn generate_target_image_name(prefix: &str,
							  target_image_name: &str,
							  layer: &DistanceLayer,
							  distance_type: &DistanceType,
							  num_channels: u8,
							  bit_depth: &ChannelBitDepth) -> String {
	let mut prefixes: Vec<String> = Vec::new();
	prefixes.push(String::from(prefix));
	prefixes.push(get_distance_type_prefix(&distance_type));
	prefixes.push(get_layer_prefix(&layer));
	prefixes.push(get_num_channels_prefix(num_channels));
	prefixes.push(get_bit_depth_prefix(&bit_depth));

	get_output_image_path(target_image_name,
						  prefixes.join("_").as_str())
}


fn get_input_image_path(filename: &str) -> String {
	let mut image_path_buff = PathBuf::new();
	image_path_buff.push(BASE_ASSET_FOLDER);
	image_path_buff.push(filename);
	image_path_buff.into_os_string().into_string().unwrap()
}


fn get_output_image_path(filename: &str, prefix: &str) -> String {
	let mut file_path_buff = PathBuf::new();
	file_path_buff.push(BASE_OUTPUT_FOLDER);
	file_path_buff.push(prefix.to_owned() + "_" + filename);
	file_path_buff.into_os_string().into_string().unwrap()
}

fn get_distance_type_prefix(distance_type: &DistanceType) -> String {
	distance_type.human_readable_name()
}

fn get_layer_prefix(export_type: &DistanceLayer) -> String {
	match export_type {
		DistanceLayer::Foreground => String::from("idf"),
		DistanceLayer::Background => String::from("odf"),
		DistanceLayer::Combined => String::from("cdf"),
	}
}

fn get_bit_depth_prefix(channel_depth: &ChannelBitDepth) -> String {
	match channel_depth {
		ChannelBitDepth::Eight => String::from("8bit"),
		ChannelBitDepth::Sixteen => String::from("16bit"),
		ChannelBitDepth::ThirtyTwo => String::from("32bit"),
		ChannelBitDepth::SixtyFour => String::from("64bit"),
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
