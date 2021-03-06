use crate::data::transformation::{TransformationResult};
use crate::data::serialize::ByteSerializer;

pub enum ChannelDataType {
	UnsignedInt,
	SignedInt,
	Float,
}

pub enum ChannelBitDepth {
	Eight,
	Sixteen,
	ThirtyTwo,
	SixtyFour,
}

impl ChannelBitDepth {
	pub fn number_of_bytes(&self) -> u8 {
		match self {
			ChannelBitDepth::Eight => 1,
			ChannelBitDepth::Sixteen => 2,
			ChannelBitDepth::ThirtyTwo => 4,
			ChannelBitDepth::SixtyFour => 8,
		}
	}
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
	pub data_type: ChannelDataType,
	pub bit_depth: ChannelBitDepth,
	pub data: Vec<u8>,
}

// TODO: more implementations here ...
// TODO: can we make this generic over T ?
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
				data = trans_data.data.serialize_to_bytes();
			}
			TransformationResult::TwoDimensional(trans_data) => {
				width = trans_data.width;
				height = trans_data.height;
				num_channels = 2;
				data = trans_data.data.serialize_to_bytes();
			}
			TransformationResult::ThreeDimensional(trans_data) => {
				width = trans_data.width;
				height = trans_data.height;
				num_channels = 3;
				data = trans_data.data.serialize_to_bytes();
			}
		}

		DistanceTransformationResult {
			width,
			height,
			data,
			data_type: ChannelDataType::Float,
			bit_depth: ChannelBitDepth::SixtyFour,
			num_channels,
		}
	}
}
