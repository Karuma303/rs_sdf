/// The absolute index of the nearest cell in the 1-dimensional array of cells.
/// This is a single, unsigned value.
pub struct NearestCellIndex;

/*
impl Calculator<u32> for NearestCellIndex {
	fn calculate(cell: &Cell) -> u32 {
		todo!()
	}
}
 */

/// The relative index of the nearest cell in the 1-dimensional array of cells.
/// It is measured as an offset from the current cell.
/// This is a single, signed value.
pub struct NearestCellIndexOffset;

/*
impl Calculator<i32> for NearestCellIndexOffset {
	fn calculate(cell: &Cell) -> i32 {
		todo!()
	}
}
 */

/// The absolute cartesian position (on x- and y-axis) of the nearest cell.
/// This is a tuple of unsigned values.
pub struct NearestCellPosition;

/*
impl Calculator<(u16, u16)> for NearestCellPosition {
	fn calculate(cell: &Cell) -> (u16, u16) {
		todo!()
	}
}
*/
