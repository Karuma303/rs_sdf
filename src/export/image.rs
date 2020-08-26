use std::fs::File;
use std::io::BufWriter;

use png::{ColorType, Compression, Encoder, FilterType};

use crate::data::transformation::{TransformationData, TransformationResult, DataDescriptor};
use crate::export::BitDepth;

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

pub trait ImageFileWriter {
	fn write(&self, result_writer: &dyn TransformationResultWriter);
}

impl ImageFileWriter for PngOutput {
	fn write(&self, result_writer: &dyn TransformationResultWriter) {

		let buffer = result_writer.write_to_buffer();
		let descriptor = result_writer.descriptor();
		// TODO: continue here by using descriptor!
		// and here comes the enchilada

		let num_channels = descriptor.num_channels;
		let bit_depth = descriptor.bit_depth;

		/*
		match num_channels {
			1 => self.output_single_channel(buffer, bit_depth),
			2 => self.output_dual_channel(buffer, bit_depth),
			3 => self.output_triple_channel(buffer, bit_depth),
			_ => panic!("number of channels not supported: {}", num_channels),
		}
		 */
		let width = descriptor.width;
		let height = descriptor.height;
		let depth = match bit_depth {
			BitDepth::Eight => ImageOutputChannelDepth::Eight,
			BitDepth::Sixten => ImageOutputChannelDepth::Sixteen,
			BitDepth::ThirtyTwo => ImageOutputChannelDepth::ThirtyTwo,
		};
		self.output_image_file(buffer, width, height, num_channels, depth);
	}
}

// neu neu neu - und möglicherweise besser als der andere rotz
/*
impl TransformationOutputWriter<u8> for PngOutput {
	fn write(&self, output: TransformationResult<u8>) {
		match output {
			TransformationResult::OneDimensional(trans_data) => {
				let mut buffer: Vec<u8> = self.init_buffer(trans_data.data.len(),
														   ImageOutputChannelDepth::Eight,
														   1);
				self.output_single_channel_u8(&trans_data.data,
											  &mut buffer);
				self.output_image_file(buffer,
									   trans_data.width,
									   trans_data.height,
									   1,
									   ImageOutputChannelDepth::Eight);
			}
			TransformationResult::TwoDimensional(trans_data) => {
				let mut buffer: Vec<u8> = self.init_buffer(trans_data.data.len(),
														   ImageOutputChannelDepth::Eight,
														   2);
				self.output_dual_channel_u8(&trans_data.data,
											&mut buffer);
				self.output_image_file(buffer,
									   trans_data.width,
									   trans_data.height,
									   2,
									   ImageOutputChannelDepth::Eight);
			}
			TransformationResult::ThreeDimensional(trans_data) => {
				todo!()
			}
		}
	}
}

impl TransformationOutputWriter<u16> for PngOutput {
	fn write(&self, output: TransformationResult<u16>) {
		match output {
			TransformationResult::OneDimensional(trans_data) => {
				let mut buffer: Vec<u8> = self.init_buffer(trans_data.data.len(),
														   ImageOutputChannelDepth::Sixteen,
														   1);
				self.output_single_channel_u16(&trans_data.data,
											   &mut buffer);
				self.output_image_file(buffer,
									   trans_data.width,
									   trans_data.height,
									   1,
									   ImageOutputChannelDepth::Sixteen);
			}
			TransformationResult::TwoDimensional(trans_data) => {
				let mut buffer: Vec<u8> = self.init_buffer(trans_data.data.len(),
														   ImageOutputChannelDepth::Sixteen,
														   2);
				self.output_dual_channel_u16(&trans_data.data,
											 &mut buffer);
				self.output_image_file(buffer,
									   trans_data.width,
									   trans_data.height,
									   2,
									   ImageOutputChannelDepth::Sixteen);
			}
			TransformationResult::ThreeDimensional(_trans_data) => {
				todo!()
			}
		}
	}
}

impl TransformationOutputWriter<u32> for PngOutput {
	fn write(&self, output: TransformationResult<u32>) {
		match output {
			TransformationResult::OneDimensional(trans_data) => {
				let mut buffer: Vec<u8> = self.init_buffer(trans_data.data.len(),
														   ImageOutputChannelDepth::ThirtyTwo,
														   1);
				self.output_single_channel_u32(&trans_data.data,
											   &mut buffer);
				self.output_image_file(buffer,
									   trans_data.width,
									   trans_data.height,
									   1,
									   ImageOutputChannelDepth::ThirtyTwo);
			}
			TransformationResult::TwoDimensional(trans_data) => {
				let mut buffer: Vec<u8> = self.init_buffer(trans_data.data.len(),
														   ImageOutputChannelDepth::ThirtyTwo,
														   2);
				self.output_dual_channel_u32(&trans_data.data,
											 &mut buffer);
				self.output_image_file(buffer,
									   trans_data.width,
									   trans_data.height,
									   2,
									   ImageOutputChannelDepth::ThirtyTwo);
			}
			TransformationResult::ThreeDimensional(trans_data) => {
				todo!()
			}
		}
	}
}
*/
impl PngOutput {
	fn output_image_file(&self, image_data_buffer: Vec<u8>,
						 width: u32,
						 height: u32,
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

	/*

	fn output_dual_channel(&self, df: &DistanceField, distance_type: &DistanceType, buffer: &mut Vec<u8>) {
// inner and outer go on a separate channel

		let dummy_depth = ImageOutputChannelDepth::Sixteen;

		match dummy_depth {
			ImageOutputChannelDepth::Eight => {
				let function = distance_type.calculator();
				df.data.iter().for_each(|cell: &Cell| {
// let distance = self.get_8_bit_distance(&cell);
					let distance: u8 = u16_to_u8_clamped(function(&cell));
					match cell.layer {
						CellLayer::Foreground => {
							buffer.push(distance);
							buffer.push(0x00);
							buffer.push(0x00);
						}
						CellLayer::Background => {
							buffer.push(0x00);
							buffer.push(distance);
							buffer.push(0x00);
						}
					}
				});
			}
			ImageOutputChannelDepth::Sixteen => {
				let function = distance_type.calculator();
				df.data.iter().for_each(|cell: &Cell| {
// let distance = self.get_8_bit_distance(&cell);
					let distance: u16 = function(&cell);
					let higher_byte = (distance >> 8) as u8;
					let lower_byte = (distance & 0xFF) as u8;
					match cell.layer {
						CellLayer::Foreground => {
							buffer.push(higher_byte);
							buffer.push(lower_byte);
							buffer.push(0x00);
							buffer.push(0x00);
							buffer.push(0x00);
							buffer.push(0x00);
						}
						CellLayer::Background => {
							buffer.push(0x00);
							buffer.push(0x00);
							buffer.push(higher_byte);
							buffer.push(lower_byte);
							buffer.push(0x00);
							buffer.push(0x00);
						}
					}
				});

// TODO: we have to implement the 32 bit output (use for example for squared distance output)
				todo!();
// mode rgb width 16 bit per channel needed here
			}
			_ => unimplemented!(),
		}
	} */

// export quad channel
}


fn get_standard_encoder(file_path: &str,
						width: u32,
						height: u32,
						channel_depth: &ImageOutputChannelDepth,
						num_channels: u8) -> Encoder<BufWriter<File>> {
	println!("{:?}", file_path);
	let file = File::create(file_path).unwrap();
	let w = BufWriter::new(file);

	let mut e = Encoder::new(w, width, height);
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
		let (num_channels, width, height) = match self {
			TransformationResult::OneDimensional(one) => (1, one.width, one.height),
			TransformationResult::TwoDimensional(two) => (2, two.width, two.height),
			TransformationResult::ThreeDimensional(three) => (3, three.width, three.height),
		};
		DataDescriptor {
			width,
			height,
			num_channels,
			bit_depth : BitDepth::Eight,
		}
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
		let (num_channels, width, height) = match self {
			TransformationResult::OneDimensional(one) => (1, one.width, one.height),
			TransformationResult::TwoDimensional(two) => (2, two.width, two.height),
			TransformationResult::ThreeDimensional(three) => (3, three.width, three.height),
		};
		DataDescriptor {
			width,
			height,
			num_channels,
			bit_depth : BitDepth::Eight,
		}
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

