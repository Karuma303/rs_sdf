use crate::data::transformation::{TransformationResult, TransformationData};
use std::io::Write;

pub enum ChannelDataType {
	UnsignedInt(BitDepth),
	SignedInt(BitDepth),
	Float(BitDepth),
}

#[derive(PartialEq, Clone)]
pub enum BitDepth {
	Eight,
	Sixten,
	ThirtyTwo,
	SixtyFour,
}

pub struct DistanceTransformationResult {
	pub width: u16,
	pub height: u16,
	pub num_channels: u8,
	pub channel_data_type: ChannelDataType,
	pub data: Vec<u8>,
}

pub fn get_u8_vec_from_single_f64(data: &[f64]) -> Vec<u8> {
	let mut result = Vec::<u8>::with_capacity(8 * data.len());
	data.iter().for_each(|element| {
		push_f64_to_buffer(&element,&mut result);
	});
	result
}

pub fn get_u8_vec_from_double_f64(data: &[(f64, f64)]) -> Vec<u8> {
	let mut result = Vec::<u8>::with_capacity(8 * 2 * data.len());
	data.iter().for_each(|element| {
		push_f64_to_buffer(&element.0,&mut result);
		push_f64_to_buffer(&element.1,&mut result);
	});
	result
}

pub fn get_u8_vec_from_triple_f64(data: &[(f64, f64, f64)]) -> Vec<u8> {
	let mut result = Vec::<u8>::with_capacity(8 * 3 * data.len());
	data.iter().for_each(|element| {
		push_f64_to_buffer(&element.0,&mut result);
		push_f64_to_buffer(&element.1,&mut result);
		push_f64_to_buffer(&element.2,&mut result);
	});
	result
}

fn push_f64_to_buffer(value : &f64, buffer : &mut Vec<u8>) {
	let bytes = value.to_le_bytes();
	buffer.push(bytes[0]);
	buffer.push(bytes[1]);
	buffer.push(bytes[2]);
	buffer.push(bytes[3]);
	buffer.push(bytes[4]);
	buffer.push(bytes[5]);
	buffer.push(bytes[6]);
	buffer.push(bytes[7]);
}

impl From<TransformationResult<f64>> for DistanceTransformationResult {
	fn from(result: TransformationResult<f64>) -> Self {
		let width: u16;
		let height: u16;
		let num_channels: u8;
		let data: Vec<u8>;
		match result {
			TransformationResult::OneDimensional(trans_data) => {
				width = trans_data.width;
				height = trans_data.height;
				num_channels = 1;
				data = get_u8_vec_from_single_f64(&trans_data.data);
			}
			TransformationResult::TwoDimensional(trans_data) => {
				width = trans_data.width;
				height = trans_data.height;
				num_channels = 2;
				data = get_u8_vec_from_double_f64(&trans_data.data);
			}
			TransformationResult::ThreeDimensional(trans_data) => {
				width = trans_data.width;
				height = trans_data.height;
				num_channels = 3;
				data = get_u8_vec_from_triple_f64(&trans_data.data);
			}
		}

		DistanceTransformationResult {
			width,
			height,
			data,
			channel_data_type: ChannelDataType::Float(BitDepth::SixtyFour),
			num_channels,
		}
	}
}
