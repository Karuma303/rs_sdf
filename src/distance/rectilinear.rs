use crate::distance::OneDimensionalDistanceCalculation;
use crate::data::Cell;
use crate::utils::{u32_to_u16_clamped, u32_to_u8_clamped};

/// The Rectilinear distance (also know as the Manhattan distance) to the nearest cell.
/// It is defined as the sum of the horizontal distance and the vertical distance.
/// The distance is a single, unsigned value.
pub struct RectilinearDistance;

impl RectilinearDistance {
	// This is the default calculation for this distance type with maximum precision
	pub fn calculate_u32(cell : &Cell) -> u32 {
		if let Some(nearest) = &cell.nearest_cell_position {
			let dx: u32 = (nearest.x as i32 - cell.x as i32).abs() as u32;
			let dy: u32 = (nearest.y as i32 - cell.y as i32).abs() as u32;
			dx + dy
		} else {
			0u32
		}
	}
}

impl OneDimensionalDistanceCalculation<u8> for RectilinearDistance {
	fn calculate(cell: &Cell) -> u8 {
		u32_to_u8_clamped(RectilinearDistance::calculate(&cell))
	}
}

impl OneDimensionalDistanceCalculation<u16> for RectilinearDistance {
	fn calculate(cell: &Cell) -> u16 {
		u32_to_u16_clamped(RectilinearDistance::calculate(&cell))
	}
}

impl OneDimensionalDistanceCalculation<u32> for RectilinearDistance {
	fn calculate(cell: &Cell) -> u32 {
		if let Some(nearest) = &cell.nearest_cell_position {
			let dx: u32 = (nearest.x as i32 - cell.x as i32).abs() as u32;
			let dy: u32 = (nearest.y as i32 - cell.y as i32).abs() as u32;
			dx + dy
		} else {
			0u32
		}
	}
}
