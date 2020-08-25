use crate::data::Cell;
use crate::distance::TwoDimensionalDistanceCalculation;

/// The cartesian distance on x- and y-axis to the nearest cell.
/// The difference is a tuple of signed values.
/// TODO: check if this distance really makes sense - in theory we can substitute
///  it with nearest_cell_position
pub struct CartesianDistance;

impl TwoDimensionalDistanceCalculation<u8> for CartesianDistance {
	fn calculate(_cell: &Cell) -> (u8, u8) {
		// TODO: implement
		(0u8, 0u8)
	}
}

impl TwoDimensionalDistanceCalculation<u16> for CartesianDistance {
	fn calculate(_cell: &Cell) -> (u16, u16) {
		// TODO: implement
		(0u16, 0u16)
	}
}

impl TwoDimensionalDistanceCalculation<u32> for CartesianDistance {
	fn calculate(_cell: &Cell) -> (u32, u32) {
		// TODO: implement
		(0u32, 0u32)
	}
}
