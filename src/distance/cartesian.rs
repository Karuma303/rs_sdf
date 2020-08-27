use crate::data::Cell;
use crate::distance::TwoDimensionalDistanceCalculation;
use crate::utils::{u16_to_u8_clamped, i32_to_u8_clamped, i32_to_u16_clamped};

/// The cartesian distance on x- and y-axis to the nearest cell.
/// The difference is a tuple of signed values.
/// TODO: check if this distance really makes sense - in theory we can substitute
///  it with nearest_cell_position_offset
pub struct CartesianDistance;

impl TwoDimensionalDistanceCalculation<u8> for CartesianDistance {
	fn calculate(cell: &Cell) -> (u8, u8) {
		if let Some((nearest_x, nearest_y)) = cell.nearest_cell_position {
			let dx: i32 = cell.x as i32 - nearest_x as i32;
			let dy: i32 = cell.y as i32 - nearest_y as i32;
			(i32_to_u8_clamped(dx), i32_to_u8_clamped(dy))
		} else {
			(0u8, 0u8)
		}
	}
}

impl TwoDimensionalDistanceCalculation<u16> for CartesianDistance {
	fn calculate(cell: &Cell) -> (u16, u16) {
		if let Some((nearest_x, nearest_y)) = cell.nearest_cell_position {
			let dx: i32 = cell.x as i32 - nearest_x as i32;
			let dy: i32 = cell.y as i32 - nearest_y as i32;
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
