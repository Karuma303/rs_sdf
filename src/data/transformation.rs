use crate::distance::{DistanceLayer, DistanceType, OneDimensionalDistanceCalculation, TwoDimensionalDistanceCalculation};
use crate::data::{DistanceField, Cell, CellLayer};
use crate::export::BitDepth;
use crate::distance::euclid::{EuclideanDistance, EuclideanDistanceSquared};
use crate::distance::cartesian::CartesianDistance;
use crate::distance::chebyshev::ChebyshevDistance;
use crate::distance::rectilinear::RectilinearDistance;
use crate::distance::nearest_cell::{NearestCellIndex, NearestCellIndexOffset, NearestCellPosition};

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

	fn get_trans_result<T>(&self, width: u32, height: u32, data: Vec<T>) -> TransformationData<T> {
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
	pub width: u32,
	pub height: u32,
	pub data: Vec<T>,
	// length of the data vector should be equal width * height
}

pub struct DataDescriptor {
	pub width: u32,
	pub height: u32,
	pub bit_depth: BitDepth,
	pub num_channels: u8,
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

impl DistanceTransformation {
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

	fn two_dimensional_distance_transform<T>(&self, function: fn(&Cell) -> (T, T)) -> TransformationResult<T> {
		let mut buffer: Vec<(T, T)> = self.init_buffer();
		self.distance_field.data.iter().for_each(|cell: &Cell| {
			buffer.push(function(&cell));
		});
		TransformationResult::TwoDimensional(self.get_transformation_data(buffer))
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
			DistanceType::EuclideanDistance => self.one_dimensional_distance_transform(EuclideanDistance::calculate, 0),
			DistanceType::EuclideanDistanceSquared => self.one_dimensional_distance_transform(EuclideanDistanceSquared::calculate, 0),
			DistanceType::CartesianDistance => self.two_dimensional_distance_transform(CartesianDistance::calculate),
			DistanceType::ChebyshevDistance => self.one_dimensional_distance_transform(ChebyshevDistance::calculate, 0),
			DistanceType::RectilinearDistance => self.one_dimensional_distance_transform(RectilinearDistance::calculate, 0),
			DistanceType::NearestCellIndex => self.one_dimensional_distance_transform(NearestCellIndex::calculate, 0),
			DistanceType::NearestCellIndexOffset => self.one_dimensional_distance_transform(NearestCellIndexOffset::calculate, 0),
			DistanceType::NearestCellPosition => self.two_dimensional_distance_transform(NearestCellPosition::calculate),
		}
	}
}

impl TransformOutputGenerator<u16> for DistanceTransformation {
	fn transform(&self) -> TransformationResult<u16> {
		match self.distance_type {
			DistanceType::EuclideanDistance => self.one_dimensional_distance_transform(EuclideanDistance::calculate, 0),
			DistanceType::EuclideanDistanceSquared => self.one_dimensional_distance_transform(EuclideanDistanceSquared::calculate, 0),
			DistanceType::CartesianDistance => self.two_dimensional_distance_transform(CartesianDistance::calculate),
			DistanceType::ChebyshevDistance => self.one_dimensional_distance_transform(ChebyshevDistance::calculate, 0),
			DistanceType::RectilinearDistance => self.one_dimensional_distance_transform(RectilinearDistance::calculate, 0),
			DistanceType::NearestCellIndex => self.one_dimensional_distance_transform(NearestCellIndex::calculate, 0),
			DistanceType::NearestCellIndexOffset => self.one_dimensional_distance_transform(NearestCellIndexOffset::calculate, 0),
			DistanceType::NearestCellPosition => self.two_dimensional_distance_transform(NearestCellPosition::calculate),
		}
	}
}
