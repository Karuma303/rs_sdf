use crate::data::Cell;
use crate::distance::{OneDimensionalDistanceCalculation, TwoDimensionalDistanceCalculation};
use crate::utils::{u16_to_u8_clamped, u32_to_u16_clamped, u32_to_u8_clamped};

/// The absolute index of the nearest cell in the 1-dimensional array of cells.
/// This is a single, unsigned value.
pub struct NearestCellIndex;

impl NearestCellIndex {
	// This is the default calculation for this distance type with maximum precision
	pub fn calculate_u32(cell: &Cell) -> u32 {
		if let Some(nearest) = &cell.nearest_cell_position {
			nearest.index
		} else {
			0u32
		}
	}
}

impl OneDimensionalDistanceCalculation<u8> for NearestCellIndex {
	fn calculate(cell: &Cell) -> u8 {
		u32_to_u8_clamped(NearestCellIndex::calculate(&cell))
	}
}

impl OneDimensionalDistanceCalculation<u16> for NearestCellIndex {
	fn calculate(cell: &Cell) -> u16 {
		u32_to_u16_clamped(NearestCellIndex::calculate(&cell))
	}
}

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

impl NearestCellPosition {
	// This is the default calculation for this distance type with maximum precision
	pub fn calculate_u16(cell : &Cell) -> (u16, u16) {
		if let Some(nearest) = &cell.nearest_cell_position {
			(nearest.x, nearest.y)
		} else {
			(0u16, 0u16)
		}
	}
}

impl TwoDimensionalDistanceCalculation<u8> for NearestCellPosition {
	fn calculate(cell: &Cell) -> (u8, u8) {
		let (x, y) = NearestCellPosition::calculate(&cell);
		(u16_to_u8_clamped(x), u16_to_u8_clamped(y))
	}
}

impl TwoDimensionalDistanceCalculation<u16> for NearestCellPosition {
	fn calculate(cell: &Cell) -> (u16, u16) {
		if let Some(nearest) = &cell.nearest_cell_position {
			(nearest.x, nearest.y)
		} else {
			(0u16, 0u16)
		}
	}
}

impl TwoDimensionalDistanceCalculation<u32> for NearestCellPosition {
	fn calculate(cell: &Cell) -> (u32, u32) {
		if let Some(nearest) = &cell.nearest_cell_position {
			(nearest.x as u32, nearest.y as u32)
		} else {
			(0u32, 0u32)
		}
	}
}

/// The relative distance of the nearest cell in the 1-dimensional array of cells.
/// This is a single, signed value.
pub struct NearestCellIndexOffset;

impl NearestCellIndexOffset {
	// This is the default calculation for this distance type with maximum precision
	pub fn calculate_i32(cell : &Cell) -> i32 {
		if let Some(nearest) = &cell.nearest_cell_position {
			nearest.index as i32 - cell.index as i32
		} else {
			0i32
		}
	}
}

impl OneDimensionalDistanceCalculation<u8> for NearestCellIndexOffset {
	fn calculate(cell: &Cell) -> u8 {
		let val: i32 = NearestCellIndexOffset::calculate(&cell);
		u32_to_u8_clamped(val as u32)
	}
}

impl OneDimensionalDistanceCalculation<u16> for NearestCellIndexOffset {
	fn calculate(cell: &Cell) -> u16 {
		let val: i32 = NearestCellIndexOffset::calculate(&cell);
		u32_to_u16_clamped(val as u32)
	}
}

impl OneDimensionalDistanceCalculation<u32> for NearestCellIndexOffset {
	fn calculate(cell: &Cell) -> u32 {
		let val: i32 = NearestCellIndexOffset::calculate(&cell);
		val as u32
	}
}

impl OneDimensionalDistanceCalculation<i32> for NearestCellIndexOffset {
	fn calculate(cell: &Cell) -> i32 {
		if let Some(nearest) = &cell.nearest_cell_position {
			nearest.index as i32 - cell.index as i32
		} else {
			0i32
		}
	}
}