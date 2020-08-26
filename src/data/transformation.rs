use crate::distance::{DistanceLayer, DistanceType, OneDimensionalDistanceCalculation, TwoDimensionalDistanceCalculation};
use crate::distance::DistanceType::EuclideanDistance;
use crate::distance::euclid::EuclideanDistance as Euclid;
use crate::distance::cartesian::CartesianDistance as Cartesian;
use crate::data::{DistanceField, Cell};
use crate::export::BitDepth;

impl From<DistanceField> for DistanceTransformation {
	fn from(df: DistanceField) -> Self {
		DistanceTransformation::new(df)
	}
}

pub trait DistanceCalculator {
	fn calculate<T>(&self, cells: &Vec<Cell>, calc_function: fn(&Cell) -> T) -> Vec<T>;
}

impl DistanceCalculator for DistanceTransformation {
	fn calculate<T>(&self, cells: &Vec<Cell>, calc_function: fn(&Cell) -> T) -> Vec<T> {
		let mut channel = Vec::with_capacity(cells.len());
		cells.iter().for_each(|cell| channel.push(calc_function(cell)));
		channel
	}
}

impl DistanceTransformation {
	fn new(distance_field: DistanceField) -> Self {
		Self {
			distance_field,
			filter: DistanceLayer::Combined,
			scale: None,
			distance_type: EuclideanDistance,
		}
	}

	pub fn filter(&mut self, layer: DistanceLayer) {
		self.filter = layer;
	}

	pub fn scale(&mut self, scale: f32) {
		self.scale = Some(scale);
	}

	pub fn distance_type(&mut self, distance_type: DistanceType) {
		self.distance_type = distance_type;
	}


	fn get_trans_result<T>(&self, width: u32, height: u32, data: Vec<T>) -> TransformationData<T> {
		TransformationData::<T> {
			width,
			height,
			data,
		}
	}



	/* taken from former export mod:
			// inner / outer / combined ?
			// combined: add or sdf?
			// 8 bit / 16 bit

		fn output_single_channel(&self, channel_data: &Vec<u32>, distance_type: &DistanceType, buffer: &mut Vec<u8>) {
			match &self.configuration.channel_depth {
			ImageOutputChannelDepth::Eight => {
				let function = distance_type.calculation_function();
				df.data.iter().for_each(|cell: &Cell| {
					// TODO: right now, we just add the inner distances and the outer distances
					// We should add a feature to generate real 8-bit-signed distance field here!
					// buffer.push(self.get_8_bit_distance(&cell));
// checked!!!
					buffer.push(u16_to_u8_clamped(function(&cell)));
				});
			}
			ImageOutputChannelDepth::Sixteen => {
				let function = distance_type.calculation_function();
				df.data.iter().for_each(|cell: &Cell| {
					let distance = function(&cell);
					buffer.push((distance >> 8) as u8);
					buffer.push((distance & 0xFF) as u8);
				});
			}
			_ => {
				// TODO: we have to implement the 32 bit output (use for example for squared distance output)
				unimplemented!()
			}


	fn output_dual_channel(&self, df: &DistanceField, distance_type: &DistanceType, buffer: &mut Vec<u8>) {
		// inner and outer go on a separate channel
		match &self.configuration.channel_depth {
			ImageOutputChannelDepth::Eight => {
				let function = distance_type.calculation_function();
				df.data.iter().for_each(|cell: &Cell| {
					// let distance = self.get_8_bit_distance(&cell);
					let distance = u16_to_u8_clamped(function(&cell));
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
				let function = distance_type.calculation_function();
				df.data.iter().for_each(|cell: &Cell| {
					// let distance = self.get_8_bit_distance(&cell);
					let distance = function(&cell);
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
	}
	 */
}

// ******************************************************
// new stuff - a lot of other code will be deprecated !!!
// ******************************************************
pub enum TransformationResult<T> {
	OneDimensional(TransformationData<T>),
	TwoDimensional(TransformationData<(T, T)>),
	ThreeDimensional(TransformationData<(T, T, T)>),
}

pub struct TransformationData<T> {
	pub width: u32,
	pub height: u32,
	pub data: Vec<T>,
	// length of the data vector should be equal width * height
}

pub struct DataDescriptor {
	pub width : u32,
	pub height : u32,
	pub bit_depth : BitDepth,
	pub num_channels : u8,
}

impl<T> TransformationData<T> {
	pub fn new(width: u32, height: u32, data: Vec<T>) -> Self {
		Self {
			width,
			height,
			data,
		}
	}
}

pub struct DistanceTransformation {
	distance_field: DistanceField,
	filter: DistanceLayer,
	scale: Option<f32>,
	distance_type: DistanceType,
}

pub trait TransformOutputGenerator<T> {
	fn transform(&self) -> TransformationResult<T>;
}

// TODO: scale and filter are not taken into account atm. for TransformOutputGenerator !
// TODO: implement distance field -> TransformationData

impl TransformOutputGenerator<u8> for DistanceTransformation {
	fn transform(&self) -> TransformationResult<u8> {
		let width = self.distance_field.width;
		let height = self.distance_field.height;

		match self.distance_type {
			DistanceType::EuclideanDistance => {
				let mut buffer = Vec::with_capacity(width as usize * height as usize);
				let function = |cell: &Cell| Euclid::calculate(&cell);
				self.distance_field.data.iter().for_each(|cell: &Cell| {
					buffer.push(function(&cell));
				});

				TransformationResult::OneDimensional(TransformationData::new(width, height, buffer))
			},
			DistanceType::CartesianDistance => {
				let mut buffer = Vec::with_capacity(width as usize * height as usize);
				let function = |cell: &Cell| Cartesian::calculate(&cell);
				self.distance_field.data.iter().for_each(|cell: &Cell| {
					buffer.push(function(&cell));
				});

				TransformationResult::TwoDimensional(TransformationData::new(width, height, buffer))
			},
			_ => panic!("not implemented"),
		}
	}
}

impl TransformOutputGenerator<u16> for DistanceTransformation {
	fn transform(&self) -> TransformationResult<u16> {
		let width = self.distance_field.width;
		let height = self.distance_field.height;

		match self.distance_type {
			DistanceType::EuclideanDistance => {
				let mut buffer = Vec::with_capacity(width as usize * height as usize);
				let function = |cell: &Cell| Euclid::calculate(&cell);
				self.distance_field.data.iter().for_each(|cell: &Cell| {
					buffer.push(function(&cell));
				});

				TransformationResult::OneDimensional(TransformationData::new(width, height, buffer))
			},
			DistanceType::CartesianDistance => {
				let mut buffer = Vec::with_capacity(width as usize * height as usize);
				let function = |cell: &Cell| Cartesian::calculate(&cell);
				self.distance_field.data.iter().for_each(|cell: &Cell| {
					buffer.push(function(&cell));
				});

				TransformationResult::TwoDimensional(TransformationData::new(width, height, buffer))
			},
			_ => panic!("not implemented"),
		}
	}
}
