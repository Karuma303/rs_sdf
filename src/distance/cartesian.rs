use crate::data::Cell;
use crate::distance::TwoDimensionalDistanceCalculation;
use crate::utils::{i32_to_u8_clamped, i32_to_u16_clamped};

/// The cartesian distance on x- and y-axis to the nearest cell.
/// The difference is a tuple of signed values.
pub struct CartesianDistance;

impl TwoDimensionalDistanceCalculation<u8> for CartesianDistance {
	fn calculate(cell: &Cell) -> (u8, u8) {
		if let Some(nearest) = &cell.nearest_cell_position {
			let dx: i32 = nearest.x as i32 - cell.x as i32;
			let dy: i32 = nearest.y as i32 - cell.y as i32;
			(i32_to_u8_clamped(dx), i32_to_u8_clamped(dy))
		} else {
			(0u8, 0u8)
		}
	}
}

impl TwoDimensionalDistanceCalculation<i32> for CartesianDistance {
	fn calculate(cell: &Cell) -> (i32, i32) {
		if let Some(nearest) = &cell.nearest_cell_position {
			let dx: i32 = nearest.x as i32 - cell.x as i32;
			let dy: i32 = nearest.y as i32 - cell.y as i32;
			(dx, dy)
		} else {
			(0i32, 0i32)
		}
	}
}

impl TwoDimensionalDistanceCalculation<u16> for CartesianDistance {
	fn calculate(cell: &Cell) -> (u16, u16) {
		if let Some(nearest) = &cell.nearest_cell_position {
			let dx = nearest.x as i32 - cell.x as i32;
			let dy = nearest.y as i32 - cell.y as i32;
			(i32_to_u16_clamped(dx), i32_to_u16_clamped(dy))
		} else {
			(0u16, 0u16)
		}
	}
}

impl TwoDimensionalDistanceCalculation<u32> for CartesianDistance {
	fn calculate(_cell: &Cell) -> (u32, u32) {
		todo!()
	}
}
