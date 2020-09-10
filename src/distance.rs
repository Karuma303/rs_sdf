pub mod cartesian;
pub mod chebyshev;
pub mod euclid;
pub mod nearest_cell;
pub mod rectilinear;

use crate::data::Cell;
use crate::distance::DistanceValueType::{TupleU16, U16, U32, I32};

#[derive(Clone, Copy)]
pub enum DistanceLayer {
	Foreground,
	Background,
	Combined,
}

/// Specification of all the different distance types that the library is able to calculate.
#[derive(Clone, Copy)]
pub enum DistanceType {
	/// The euclidean distance to the nearest cell.
	/// The distance is a single, unsigned value.
	EuclideanDistance,

	/// The squared euclidean distance to the nearest cell.
	/// The distance is a single, unsigned value.
	EuclideanDistanceSquared,

	/// The Chebyshev distance (also known as the chessboard distance) to the nearest cell.
	/// It is defined as the maximum of the horizontal distance and the vertical distance.
	/// The distance is a single unsigned value.
	ChebyshevDistance,

	/// The Rectilinear distance (also know as the Manhattan distance) to the nearest cell.
	/// It is defined as the sum of the horizontal distance and the vertical distance.
	/// The distance is a single, unsigned value.
	RectilinearDistance,

	/// The cartesian distance on x- and y-axis to the nearest cell.
	/// The difference is a tuple of signed values.
	CartesianDistance,

	/// The absolute index of the nearest cell in the 1-dimensional array of cells.
	/// This is a single, unsigned value.
	NearestCellIndex,

	/// The relative index of the nearest cell in the 1-dimensional array of cells.
	/// It is measured as an offset from the current cell.
	/// This is a single, signed value.
	NearestCellIndexOffset,

	/// The absolute cartesian position (on x- and y-axis) of the nearest cell.
	/// This is a tuple of unsigned values.
	NearestCellPosition,
}

pub enum DistanceValueType {
	U16,
	U32,
	I16,
	I32,
	F32,
	F64,
	TupleU16,
}

pub type CalculationFunction<T> = fn(&Cell) -> T;

pub enum Calculator<T> {
	OneDimensional(CalculationFunction<T>),
	TwoDimensional(CalculationFunction<(T, T)>),
	ThreeDimensional(CalculationFunction<(T, T, T)>),
}

impl DistanceType {
	pub fn human_readable_name(&self) -> String {
		match self {
			DistanceType::EuclideanDistance => String::from("euclidean"),
			DistanceType::EuclideanDistanceSquared => String::from("euclidean_squared"),
			DistanceType::CartesianDistance => String::from("cartesian"),
			DistanceType::ChebyshevDistance => String::from("chebyshev"),
			DistanceType::RectilinearDistance => String::from("rectilinear"),
			DistanceType::NearestCellPosition => String::from("nearest_cell_pos"),
			DistanceType::NearestCellIndex => String::from("nearest_cell_index"),
			DistanceType::NearestCellIndexOffset => String::from("nearest_cell_index_offset"),
		}
	}

	/*
		pub fn calculator<T>(&self) -> Calculator<T> {
			match self {
				DistanceType::EuclideanDistance => {
					let calc : Calculator<T> = EuclideanDistance::calculator();
					return calc;
				},
				DistanceType::CartesianDistance => {
					return CartesianDistance::calculator()
				}
				_ => panic!("not implemented"),
			}
			todo!()
			// let a: fn(&Cell) -> u16 = EuclideanDistance::calculate;
			// let a = EuclideanDistance::calculate;

			/*
			match self {
				DistanceType::EuclideanDistance => EuclideanDistance::calculate,
				DistanceType::EuclideanDistanceSquared => EuclideanDistanceSquared::calculate,
				DistanceType::ChebyshevDistance => ChebyshevDistance::calculate,
				DistanceType::RectilinearDistance => RectilinearDistance::calculate,
				DistanceType::CartesianDistance => CartesianDistance::calculate,
				DistanceType::NearestCellIndex => NearestCellIndex::calculate,
				DistanceType::NearestCellIndexOffset => NearestCellIndexOffset::calculate,
				DistanceType::NearestCellPosition => NearestCellPosition::calculate,
			}

			 */
		}
	 */

	/*
	pub fn calculation_function<T>(&self) -> CalculationFunction<T> {
		match self {
			DistanceType::EuclideanDistance => todo!(),
			DistanceType::EuclideanDistanceSquared => euclidean_distance_squared_function,
			DistanceType::ChebyshevDistance => chebyshev_distance_function,
			DistanceType::RectilinearDistance => rectilinear_distance_function,
			DistanceType::CartesianDistance => cartesian_distance_function,
			DistanceType::NearestCellIndex => nearest_cell_index_function,
			DistanceType::NearestCellIndexOffset => nearest_cell_index_offset_function,
			DistanceType::NearestCellPosition => nearest_cell_position_function,
		}
		// function
	}
	*/

	pub fn value_type(&self) -> DistanceValueType {
		match self {
			DistanceType::EuclideanDistance => U16,
			DistanceType::EuclideanDistanceSquared => U32,
			DistanceType::ChebyshevDistance => U16,
			DistanceType::RectilinearDistance => U16,
			DistanceType::CartesianDistance => TupleU16,
			DistanceType::NearestCellIndex => U32,
			DistanceType::NearestCellIndexOffset => I32,
			DistanceType::NearestCellPosition => TupleU16,
		}
	}

	pub fn dimensions(&self) -> u8 {
		match self {
			DistanceType::EuclideanDistance => 1,
			DistanceType::EuclideanDistanceSquared => 1,
			DistanceType::ChebyshevDistance => 1,
			DistanceType::RectilinearDistance => 1,
			DistanceType::CartesianDistance => 2,
			DistanceType::NearestCellIndex => 1,
			DistanceType::NearestCellIndexOffset => 1,
			DistanceType::NearestCellPosition => 1,
		}
	}
}


// TODO: implement capping to u8 / u16 as functions with inline tag
// currently capping is in export/image

// ******************************************************
// new stuff - a lot of other code will be deprecated !!!
// ******************************************************

pub trait OneDimensionalDistanceCalculation<T> {
	fn calculate_legacy(cell: &Cell) -> T;
}

pub trait TwoDimensionalDistanceCalculation<T> {
	fn calculate_legacy(cell: &Cell) -> (T, T);
}

pub trait ThreeDimensionalDistanceCalculation<T> {
	fn calculate_legacy(cell: &Cell) -> (T, T, T);
}




