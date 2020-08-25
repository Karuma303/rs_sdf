use crate::distance::{DistanceLayer, DistanceType, OneDimensionalDistanceCalculation, TwoDimensionalDistanceCalculation};
use crate::distance::DistanceType::EuclideanDistance;
use crate::distance::euclid::EuclideanDistance as Euclid;
use crate::distance::cartesian::CartesianDistance as Cartesian;
use crate::data::{DistanceField, Cell};
use crate::utils::u16_to_u8_clamped;

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

/*
	// TODO: type argument must be set according to distance calculation method
	pub fn transform<T>(&self) -> TransformationResult<T> {
		let width = self.distance_field.width;
		let height = self.distance_field.height;

		let calculator: Calculator<T> = self.distance_type.calculator();

		match calculator {
			Calculator::OneDimensional(calc_function) => {
				let data = self.calculate::<T>(&self.distance_field.data, calc_function);
				TransformationResult::OneDimensional(self.get_trans_result(width, height, data))
			}
			Calculator::TwoDimensional(calc_function) => {
				let data = self.calculate::<(T, T)>(&self.distance_field.data, calc_function);
				TransformationResult::TwoDimensional(self.get_trans_result(width, height, data))
				//			let trans_data = self.get_trans_result::<(T, T)>(width, height, data);
				//			trans_data
			}
			Calculator::ThreeDimensional(calc_function) => {
				let data = self.calculate::<(T, T, T)>(&self.distance_field.data, calc_function);
				TransformationResult::ThreeDimensional(self.get_trans_result(width, height, data))
				//	self.get_trans_result::<(T,T,T)>(width, height, data)
			}
		}

		// let data = self.calculate(&self.distance_field.data, calculator);

		// self.get_trans_result(width, height, data)
	}
*/
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

impl <T>TransformationData<T> {
	pub fn new(width : u32, height : u32, data : Vec<T>) -> Self {
		Self{
			width,
			height,
			data,
		}
	}
}
/*
impl TransformationData<(u8, u8)>{
	pub fn new(width : u32, height : u32, data : Vec<(u8, u8)>) -> Self {
		Self{
			width,
			height,
			data,
		}
	}
}
*/
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
		let width  = self.distance_field.width;
		let height = self.distance_field.height;

		match self.distance_type {
			DistanceType::EuclideanDistance => {
					let mut buffer = Vec::with_capacity(width as usize * height as usize);
					// TODO: das ist quatsch. "calculate" sollten keine Instanzmethode sein
					let calc_instance = Euclid{};
					let function = |cell : &Cell| calc_instance.calculate(&cell);
					self.distance_field.data.iter().for_each(|cell: &Cell| {
						// TODO: check, if we need to clamp here - should be done by the distance implementation
						buffer.push(u16_to_u8_clamped(function(&cell)));
					});

				TransformationResult::OneDimensional(TransformationData::new(width, height, buffer))
			},
			DistanceType::CartesianDistance => {
				let mut buffer = Vec::with_capacity(width as usize * height as usize);
				// TODO: das ist quatsch. "calculate" sollten keine Instanzmethode sein
				let calc_instance = Cartesian{};
				let function = |cell : &Cell| calc_instance.calculate(&cell);

				self.distance_field.data.iter().for_each(|cell: &Cell| {
					// TODO: no clamping here like in euclid (see above)
					buffer.push(function(&cell));
				});

				let res: TransformationData<(u8, u8)> = TransformationData::new(width, height, buffer);
				TransformationResult::TwoDimensional(res)
			},
			_ => panic!("not implemented"),
		}
	}
}

impl TransformOutputGenerator<u16> for DistanceTransformation {
	fn transform(&self) -> TransformationResult<u16> {
		match self.distance_type {
			DistanceType::EuclideanDistance => {
				// TODO: implement !
				let res: TransformationData<u16> = TransformationData {
					width: self.distance_field.width,
					height: self.distance_field.height,
					data: vec![1],
				};
				TransformationResult::OneDimensional(res)
			},
			DistanceType::CartesianDistance => {
				// TODO: implement !
				let res: TransformationData<(u16, u16)> = TransformationData {
					width: self.distance_field.width,
					height: self.distance_field.height,
					data: vec![(1, 2)],
				};
				TransformationResult::TwoDimensional(res)
			},
			_ => panic!("not implemented"),
		}
	}
}
