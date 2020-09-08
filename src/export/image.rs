use std::fs::File;
use std::io::BufWriter;

use png::{ColorType, Compression, Encoder, FilterType};

use crate::data::transformation::{TransformationData, TransformationResult, DataDescriptor};
use crate::export::BitDepth;
use crate::result::DistanceTransformationResult;

// Todo: We have to add to the image exporter something like a color channel definition,
// that maps the export channels to color channels

pub enum ImageOutputChannelDepth {
	Eight = 8,
	Sixteen = 16,
	ThirtyTwo = 32,
}

pub struct PngOutput {
	file_path: String,
}

impl PngOutput {
	pub fn new(file_path: &str) -> Self {
		Self {
			file_path: String::from(file_path),
		}
	}
}

pub trait DistanceTransformationResultWriter {
	fn write_result(&self, trans_res: DistanceTransformationResult);
}

impl DistanceTransformationResultWriter for PngOutput {
	fn write_result(&self, trans_res: DistanceTransformationResult) {
		todo!("bla foo")
	}
}


pub trait ImageFileWriter {
	fn write(&self, result_writer: &dyn TransformationResultWriter);
}

impl ImageFileWriter for PngOutput {
	fn write(&self, result_writer: &dyn TransformationResultWriter) {
		let descriptor = result_writer.descriptor();

		let depth = match descriptor.bit_depth {
			BitDepth::Eight => ImageOutputChannelDepth::Eight,
			BitDepth::Sixten => ImageOutputChannelDepth::Sixteen,
			BitDepth::ThirtyTwo => ImageOutputChannelDepth::ThirtyTwo,
		};
		self.output_image_file(result_writer.write_to_buffer(),
							   descriptor.width,
							   descriptor.height,
							   descriptor.num_channels,
							   depth);
	}
}

impl PngOutput {
	fn output_image_file(&self, image_data_buffer: Vec<u8>,
						 width: u16,
						 height: u16,
						 num_channels: u8,
						 bit_depth: ImageOutputChannelDepth) {
		let encoder = get_standard_encoder(&self.file_path,
										   width,
										   height,
										   &bit_depth,
										   num_channels);

		// TODO: wir haben auch configuration.channel_depth! Was nutzen wir hier?

		let mut writer = encoder.write_header().unwrap();

		// TODO: for two channels we need to add some padding bytes before !!!

		writer.write_image_data(&image_data_buffer).unwrap(); // Save
	}

	pub fn init_buffer<T>(&self, num_values: usize, bit_depth: ImageOutputChannelDepth, num_channels: usize) -> Vec<T> {
		let byte_multiplier = match bit_depth {
			ImageOutputChannelDepth::Eight => 1,
			ImageOutputChannelDepth::Sixteen => 2,
			ImageOutputChannelDepth::ThirtyTwo => 4,
		};
		let size = num_values as usize * byte_multiplier * num_channels;
		Vec::<T>::with_capacity(size)
	}

	fn output_single_channel_u8<T: Into<u8> + Copy>(&self, channel_data: &[T], buffer: &mut Vec<u8>) {
		channel_data.iter().for_each(|value: &T| {
			buffer.push((*value).into());
		});
	}

	fn output_single_channel_u16<T: Into<u16> + Copy>(&self, channel_data: &[T], buffer: &mut Vec<u8>) {
		channel_data.iter().for_each(|value: &T| {
			let val: u16 = (*value).into();
			let higher_byte = (val >> 8) as u8;
			let lower_byte = (val & 0xFF) as u8;
			buffer.push(higher_byte);
			buffer.push(lower_byte);
		});
	}

	fn output_single_channel_u32<T: Into<u32> + Copy>(&self, channel_data: &[T], _buffer: &mut Vec<u8>) {
		channel_data.iter().for_each(|_value: &T| {
			todo!()
// buffer.push(value);
		});
	}

	fn output_dual_channel_u8<T: Into<u8> + Copy>(&self,
												  channel_data: &[(T, T)],
												  buffer: &mut Vec<u8>) {
		channel_data.iter().for_each(|(value_1, value_2): &(T, T)| {
			buffer.push((*value_1).into());
			buffer.push((*value_2).into());
		});
	}

	fn output_dual_channel_u16<T: Into<u16> + Copy>(&self,
													_channel_data: &[(T, T)],
													_buffer: &mut Vec<u8>) {
// TODO: take a look at the old code in transformation.rs
		todo!()
	}

	fn output_dual_channel_u32<T: Into<u32> + Copy>(&self,
													_channel_data: &[(T, T)],
													_buffer: &mut Vec<u8>) {
		todo!()
	}
}


fn get_standard_encoder(file_path: &str,
						width: u16,
						height: u16,
						channel_depth: &ImageOutputChannelDepth,
						num_channels: u8) -> Encoder<BufWriter<File>> {
	println!("{:?}", file_path);
	let file = File::create(file_path).unwrap();
	let w = BufWriter::new(file);

	let mut e = Encoder::new(w, width as u32, height as u32);
	match num_channels {
		1 => e.set_color(ColorType::Grayscale),
		2 => e.set_color(ColorType::GrayscaleAlpha),
		3 => e.set_color(ColorType::RGB),
		4 => e.set_color(ColorType::RGBA),
		_ => panic!("number of channels ({}) is not supported", num_channels),
	}
	e.set_compression(Compression::Best);
	e.set_depth(match channel_depth {
		ImageOutputChannelDepth::Eight => png::BitDepth::Eight,
		ImageOutputChannelDepth::Sixteen => png::BitDepth::Sixteen,
		_ => unimplemented!(),
	});
	e.set_filter(FilterType::NoFilter); // ???
	e
}

// ******************************************************
// new stuff - a lot of other code will be deprecated !!!
// ******************************************************

pub trait TransformationResultWriter {
	fn write_to_buffer(&self) -> Vec<u8>;
	fn descriptor(&self) -> DataDescriptor;
}

impl dyn TransformationResultWriter {
	pub fn get_descriptor<T>(&self, res: &TransformationResult<T>) -> DataDescriptor {
		let (num_channels, width, height) = match res {
			TransformationResult::OneDimensional(one) => (1, one.width, one.height),
			TransformationResult::TwoDimensional(two) => (2, two.width, two.height),
			TransformationResult::ThreeDimensional(three) => (3, three.width, three.height),
		};
		DataDescriptor {
			width,
			height,
			num_channels,
			bit_depth: BitDepth::Eight,
		}
	}
}

impl TransformationResultWriter for TransformationResult<u8> {
	fn write_to_buffer(&self) -> Vec<u8> {
		match self {
			TransformationResult::OneDimensional(one_dimensional_data) =>
				one_dimensional_data.write(),
			TransformationResult::TwoDimensional(two_dimensional_data) =>
				two_dimensional_data.write(),
			TransformationResult::ThreeDimensional(three_dimensional_data) =>
				three_dimensional_data.write(),
		}
	}

	fn descriptor(&self) -> DataDescriptor {
		(self as &dyn TransformationResultWriter).get_descriptor(self) // this is really weird
	}
}

impl TransformationResultWriter for TransformationResult<u16> {
	fn write_to_buffer(&self) -> Vec<u8> {
		match self {
			TransformationResult::OneDimensional(one_dimensional_data) =>
				one_dimensional_data.write(),
			TransformationResult::TwoDimensional(two_dimensional_data) =>
				two_dimensional_data.write(),
			TransformationResult::ThreeDimensional(three_dimensional_data) =>
				three_dimensional_data.write(),
		}
	}

	fn descriptor(&self) -> DataDescriptor {
		(self as &dyn TransformationResultWriter).get_descriptor(self) // this is really weird
	}
}


/// Writes to a byte output buffer
pub trait ImageBufferWriter {
	fn write(&self) -> Vec<u8>;
}

impl ImageBufferWriter for &TransformationData<u8> {
	fn write(&self) -> Vec<u8> {
		println!("writing transformation result u8 !!!");
		let mut buffer: Vec<u8> = Vec::new();
		self.data.iter().for_each(|element| buffer.push(*element));
		buffer
	}
}

impl ImageBufferWriter for &TransformationData<u16> {
	fn write(&self) -> Vec<u8> {
		println!("writing transformation result u16 !!!");
		let mut buffer: Vec<u8> = Vec::new();
		self.data.iter().for_each(|element| {
			buffer.push((element >> 8) as u8);
			buffer.push((element & 0xFF) as u8);
		});
		buffer
	}
}

impl ImageBufferWriter for &TransformationData<(u8, u8)> {
	fn write(&self) -> Vec<u8> {
		println!("writing transformation result (u8, u8) !!!");
		let mut buffer: Vec<u8> = Vec::new();
		self.data.iter().for_each(|(e1, e2)| {
			buffer.push(*e1);
			buffer.push(*e2);
		});
		buffer
	}
}

impl ImageBufferWriter for &TransformationData<(u16, u16)> {
	fn write(&self) -> Vec<u8> {
		println!("writing transformation result (u16, u16) !!!");
		let mut buffer: Vec<u8> = Vec::new();
		self.data.iter().for_each(|(e1, e2)| {
			buffer.push((e1 >> 8) as u8);
			buffer.push((e1 & 0xFF) as u8);
			buffer.push((e2 >> 8) as u8);
			buffer.push((e2 & 0xFF) as u8);
		});
		buffer
	}
}

impl ImageBufferWriter for &TransformationData<(u8, u8, u8)> {
	fn write(&self) -> Vec<u8> {
		println!("writing transformation result (u8, u8, u8) !!!");
		let mut buffer: Vec<u8> = Vec::new();
		self.data.iter().for_each(|(e1, e2, e3)| {
			buffer.push(*e1);
			buffer.push(*e2);
			buffer.push(*e3);
		});
		buffer
	}
}

impl ImageBufferWriter for &TransformationData<(u16, u16, u16)> {
	fn write(&self) -> Vec<u8> {
		println!("writing transformation result (u16, u16, u16) !!!");
		let mut buffer: Vec<u8> = Vec::new();
		self.data.iter().for_each(|(e1, e2, e3)| {
			buffer.push((e1 >> 8) as u8);
			buffer.push((e1 & 0xFF) as u8);
			buffer.push((e2 >> 8) as u8);
			buffer.push((e2 & 0xFF) as u8);
			buffer.push((e3 >> 8) as u8);
			buffer.push((e3 & 0xFF) as u8);
		});
		buffer
	}
}

