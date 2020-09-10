use crate::distance::{DistanceLayer, DistanceType, OneDimensionalDistanceCalculation, TwoDimensionalDistanceCalculation};
use crate::data::{DistanceField, Cell, CellLayer};
use crate::export::BitDepth;
use crate::distance::euclid::{EuclideanDistance, EuclideanDistanceSquared};
use crate::distance::cartesian::CartesianDistance;
use crate::distance::chebyshev::ChebyshevDistance;
use crate::distance::rectilinear::RectilinearDistance;
use crate::distance::nearest_cell::{NearestCellIndex, NearestCellIndexOffset, NearestCellPosition};
use crate::result::{DistanceTransformationResult, ChannelDataType, ChannelBitDepth};
use crate::data::serialize::ByteSerializer;

impl From<DistanceField> for DistanceTransformation {
	fn from(df: DistanceField) -> Self {
		DistanceTransformation::new(df)
	}
}

pub trait DistanceCalculator {
	fn calculate<T>(&self, cells: &[Cell], calc_function: fn(&Cell) -> T) -> Vec<T>;
}

impl DistanceCalculator for DistanceTransformation {
	fn calculate<T>(&self, cells: &[Cell], calc_function: fn(&Cell) -> T) -> Vec<T> {
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
			distance_type: DistanceType::EuclideanDistance,
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

	fn get_trans_result<T>(&self, width: u16, height: u16, data: Vec<T>) -> TransformationData<T> {
		TransformationData::<T> {
			width,
			height,
			data,
		}
	}
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
	pub width: u16,
	pub height: u16,
	pub data: Vec<T>,
	// length of the data vector should be equal width * height
}

pub struct DataDescriptor {
	pub width: u16,
	pub height: u16,
	pub bit_depth: BitDepth,
	pub num_channels: u8,
}

impl<T> TransformationData<T> {
	pub fn new(width: u16, height: u16, data: Vec<T>) -> Self {
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

impl DistanceTransformation {
	pub fn result(&self) -> DistanceTransformationResult {
		match self.distance_type {
			DistanceType::EuclideanDistance => self.fill_buffer_f64(EuclideanDistance::calculate, 0f64),
			DistanceType::EuclideanDistanceSquared => self.fill_buffer_u64(EuclideanDistanceSquared::calculate_u64, 0),

			// DistanceType::CartesianDistance => self.fill_buffer_i32_i32(CartesianDistance::calculate_i32, (0, 0)),
			DistanceType::CartesianDistance =>
				self.get_transformation_result(self.two_dimensional_result(CartesianDistance::calculate, (0, 0)), 2, ChannelDataType::UnsignedInt, ChannelBitDepth::ThirtyTwo),
			DistanceType::ChebyshevDistance => self.fill_buffer_u16(ChebyshevDistance::calculate, 0),
			DistanceType::RectilinearDistance => self.fill_buffer_u32(RectilinearDistance::calculate, 0),
			DistanceType::NearestCellIndex => self.fill_buffer_u32(NearestCellIndex::calculate_u32, 0),
			DistanceType::NearestCellIndexOffset => self.fill_buffer_i32(NearestCellIndexOffset::calculate_i32, 0),
			DistanceType::NearestCellPosition => self.fill_buffer_u16_u16(NearestCellPosition::calculate, (0, 0)),
		}
	}

	fn get_transformation_result<T: ByteSerializer>(&self, buffer: T, num_channels: u8, data_type: ChannelDataType, bit_depth: ChannelBitDepth) -> DistanceTransformationResult {
		DistanceTransformationResult {
			num_channels,
			data_type,
			bit_depth,
			width: self.distance_field.width,
			height: self.distance_field.height,
			data: buffer.serialize_to_bytes(),
		}
	}

	fn fill_buffer_f64(&self, function: fn(&Cell) -> f64, null_val: f64) -> DistanceTransformationResult {
		let buffer: Vec<f64> = self.one_dimensional_result(function, null_val);
		self.get_transformation_result(buffer, 1, ChannelDataType::Float, ChannelBitDepth::SixtyFour)
	}

	fn fill_buffer_u16(&self, function: fn(&Cell) -> u16, null_val: u16) -> DistanceTransformationResult {
		let buffer: Vec<u16> = self.one_dimensional_result(function, null_val);
		self.get_transformation_result(buffer, 1, ChannelDataType::UnsignedInt, ChannelBitDepth::Sixteen)
	}

	fn fill_buffer_u16_u16(&self, function: fn(&Cell) -> (u16, u16), null_val: (u16, u16)) -> DistanceTransformationResult {
		let buffer: Vec<(u16, u16)> = self.two_dimensional_result(function, null_val);
		self.get_transformation_result(buffer, 2, ChannelDataType::UnsignedInt, ChannelBitDepth::Sixteen)
	}

	fn fill_buffer_u32(&self, function: fn(&Cell) -> u32, null_val: u32) -> DistanceTransformationResult {
		let buffer: Vec<u32> = self.one_dimensional_result(function, null_val);
		self.get_transformation_result(buffer, 1, ChannelDataType::UnsignedInt, ChannelBitDepth::ThirtyTwo)
	}

	fn fill_buffer_i32(&self, function: fn(&Cell) -> i32, null_val: i32) -> DistanceTransformationResult {
		let buffer: Vec<i32> = self.one_dimensional_result(function, null_val);
		self.get_transformation_result(buffer, 1, ChannelDataType::SignedInt, ChannelBitDepth::ThirtyTwo)
	}

	fn fill_buffer_u64(&self, function: fn(&Cell) -> u64, null_val: u64) -> DistanceTransformationResult {
		let buffer: Vec<u64> = self.one_dimensional_result(function, null_val);
		self.get_transformation_result(buffer, 1, ChannelDataType::UnsignedInt, ChannelBitDepth::SixtyFour)
	}

	fn fill_buffer_i32_i32(&self, function: fn(&Cell) -> (i32, i32), null_val: (i32, i32)) -> DistanceTransformationResult {
		let buffer: Vec<(i32, i32)> = self.two_dimensional_result(function, null_val);
		self.get_transformation_result(buffer, 2, ChannelDataType::SignedInt, ChannelBitDepth::ThirtyTwo)
	}

	// fn one_dimensional_result<T: Copy>(&self, function: fn(&Cell) -> T, null_val: T) -> DistanceTransformationResult {
	fn one_dimensional_result<T: Copy>(&self, function: fn(&Cell) -> T, null_val: T) -> Vec<T> {

		// bytevec crate?
		let mut buffer: Vec<T> = self.init_buffer();
		let cell_filter = match self.filter {
			DistanceLayer::Combined => None,
			DistanceLayer::Foreground => Some(CellLayer::Foreground),
			DistanceLayer::Background => Some(CellLayer::Background),
		};
		match cell_filter {
			None => {
				self.distance_field.data.iter()
					.for_each(|cell: &Cell| {
						buffer.push(function(&cell));
					});
			}
			Some(filter_value) => {
				self.distance_field.data.iter()
					.for_each(|cell: &Cell| {
						if cell.layer == filter_value {
							buffer.push(function(&cell));
						} else {
							buffer.push(null_val);
						}
					});
			}
		}
		buffer
	}

	fn two_dimensional_result<T: Copy>(&self, function: fn(&Cell) -> (T, T), null_val: (T, T)) -> Vec<(T, T)> {
		// TODO: filter implementieren !
		let mut buffer: Vec<(T, T)> = self.init_buffer();
		self.distance_field.data.iter().for_each(|cell: &Cell| {
			buffer.push(function(&cell));
		});
		buffer
	}

	fn three_dimensional_result<T: Copy>(&self, function: fn(&Cell) -> (T, T, T), null_val: (T, T, T)) -> Vec<(T, T, T)> {
		// TODO: filter implementieren !
		let mut buffer: Vec<(T, T, T)> = self.init_buffer();
		self.distance_field.data.iter().for_each(|cell: &Cell| {
			buffer.push(function(&cell));
		});
		buffer
	}

	fn one_dimensional_distance_transform<T: Copy>(&self, function: fn(&Cell) -> T, null_val: T) -> TransformationResult<T> {
		let mut buffer: Vec<T> = self.init_buffer();

		let cell_filter = match self.filter {
			DistanceLayer::Combined => None,
			DistanceLayer::Foreground => Some(CellLayer::Foreground),
			DistanceLayer::Background => Some(CellLayer::Background),
		};

		match cell_filter {
			None => {
				self.distance_field.data.iter()
					.for_each(|cell: &Cell| {
						buffer.push(function(&cell));
					});
			}
			Some(filter_value) => {
				self.distance_field.data.iter()
					.for_each(|cell: &Cell| {
						if cell.layer == filter_value {
							buffer.push(function(&cell));
						} else {
							buffer.push(null_val);
						}
					});
			}
		}
		TransformationResult::OneDimensional(self.get_transformation_data(buffer))
	}

	fn two_dimensional_distance_transform<T>(&self, function: fn(&Cell) -> (T, T), null_val: (T, T)) -> TransformationResult<T> {
		// TODO: filter implementieren !
		let mut buffer: Vec<(T, T)> = self.init_buffer();
		self.distance_field.data.iter().for_each(|cell: &Cell| {
			buffer.push(function(&cell));
		});
		TransformationResult::TwoDimensional(self.get_transformation_data(buffer))
	}

	fn three_dimensional_distance_transform<T>(&self, function: fn(&Cell) -> (T, T, T), null_val: (T, T, T)) -> TransformationResult<T> {
		// TODO: filter implementieren !
		let mut buffer: Vec<(T, T, T)> = self.init_buffer();
		self.distance_field.data.iter().for_each(|cell: &Cell| {
			buffer.push(function(&cell));
		});
		TransformationResult::ThreeDimensional(self.get_transformation_data(buffer))
	}

	fn init_buffer<T>(&self) -> Vec<T> {
		Vec::with_capacity(self.distance_field.width as usize * self.distance_field.height as usize)
	}

	fn get_transformation_data<T>(&self, buffer: Vec<T>) -> TransformationData<T> {
		TransformationData::new(self.distance_field.width, self.distance_field.height, buffer)
	}
}

pub trait TransformOutputGenerator<T> {
	fn transform(&self) -> TransformationResult<T>;
}

// TODO: scale and filter are not taken into account atm. for TransformOutputGenerator !
// TODO: implement distance field -> TransformationData

impl TransformOutputGenerator<u8> for DistanceTransformation {
	fn transform(&self) -> TransformationResult<u8> {
		match self.distance_type {
			DistanceType::EuclideanDistance => self.one_dimensional_distance_transform(EuclideanDistance::calculate_legacy, 0),
			DistanceType::EuclideanDistanceSquared => self.one_dimensional_distance_transform(EuclideanDistanceSquared::calculate_legacy, 0),
			DistanceType::CartesianDistance => self.two_dimensional_distance_transform(CartesianDistance::calculate_legacy, (0, 0)),
			DistanceType::ChebyshevDistance => self.one_dimensional_distance_transform(ChebyshevDistance::calculate_legacy, 0),
			DistanceType::RectilinearDistance => self.one_dimensional_distance_transform(RectilinearDistance::calculate_legacy, 0),
			DistanceType::NearestCellIndex => self.one_dimensional_distance_transform(NearestCellIndex::calculate_legacy, 0),
			DistanceType::NearestCellIndexOffset => self.one_dimensional_distance_transform(NearestCellIndexOffset::calculate_legacy, 0),
			DistanceType::NearestCellPosition => self.two_dimensional_distance_transform(NearestCellPosition::calculate_legacy, (0, 0)),
		}
	}
}

impl TransformOutputGenerator<u16> for DistanceTransformation {
	fn transform(&self) -> TransformationResult<u16> {
		match self.distance_type {
			DistanceType::EuclideanDistance => self.one_dimensional_distance_transform(EuclideanDistance::calculate_legacy, 0),
			DistanceType::EuclideanDistanceSquared => self.one_dimensional_distance_transform(EuclideanDistanceSquared::calculate_legacy, 0),
			DistanceType::CartesianDistance => self.two_dimensional_distance_transform(CartesianDistance::calculate_legacy, (0, 0)),
			DistanceType::ChebyshevDistance => self.one_dimensional_distance_transform(ChebyshevDistance::calculate_legacy, 0),
			DistanceType::RectilinearDistance => self.one_dimensional_distance_transform(RectilinearDistance::calculate_legacy, 0),
			DistanceType::NearestCellIndex => self.one_dimensional_distance_transform(NearestCellIndex::calculate_legacy, 0),
			DistanceType::NearestCellIndexOffset => self.one_dimensional_distance_transform(NearestCellIndexOffset::calculate_legacy, 0),
			DistanceType::NearestCellPosition => self.two_dimensional_distance_transform(NearestCellPosition::calculate_legacy, (0, 0)),
		}
	}
}
