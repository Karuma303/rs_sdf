use std::cmp::max;

use crate::data::Cell;
use crate::distance::OneDimensionalDistanceCalculation;

/// The Chebyshev distance (also known as the chessboard distance) to the nearest cell.
/// It is defined as the maximum of the horizontal distance and the vertical distance.
/// The distance is a single unsigned value.
pub struct ChebyshevDistance;

impl OneDimensionalDistanceCalculation<u16> for ChebyshevDistance {
	fn calculate(cell: &Cell) -> u16 {
		if let Some(nearest) = &cell.nearest_cell_position {
			let dx: u16 = (nearest.x as i32 - cell.x as i32).abs() as u16;
			let dy: u16 = (nearest.y as i32 - cell.y as i32).abs() as u16;
			max(dx, dy)
		} else {
			0u16
		}
	}
}