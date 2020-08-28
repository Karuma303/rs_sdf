use crate::distance::{OneDimensionalDistanceCalculation, TwoDimensionalDistanceCalculation};
use crate::data::Cell;

/// The absolute index of the nearest cell in the 1-dimensional array of cells.
/// This is a single, unsigned value.
pub struct NearestCellIndex;

impl OneDimensionalDistanceCalculation<u32> for NearestCellIndex {
	fn calculate(cell: &Cell) -> u32 {
		if let Some(nearest) = &cell.nearest_cell_position {
			nearest.index
		} else {
			0u32
		}
	}
}


/// The absolute cartesian position (on x- and y-axis) of the nearest cell.
/// This is a tuple of unsigned values.
pub struct NearestCellPosition;

impl TwoDimensionalDistanceCalculation<u16> for NearestCellPosition {
	fn calculate(cell: &Cell) -> (u16, u16) {
		if let Some(nearest) = &cell.nearest_cell_position {
			(nearest.x, nearest.y)
		} else {
			(0u16, 0u16)
		}
	}
}
