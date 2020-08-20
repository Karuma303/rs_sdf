/// The Chebyshev distance (also known as the chessboard distance) to the nearest cell.
/// It is defined as the maximum of the horizontal distance and the vertical distance.
/// The distance is a single unsigned value.
pub struct ChebyshevDistance;

/*
impl Calculator<u16> for ChebyshevDistance {
	// TODO: write tests for this
	fn calculate(cell: &Cell) -> u16 {
		if let Some((nearest_x, nearest_y)) = cell.nearest_cell_position {
			let dx = (cell.x as i16 - nearest_x as i16).abs() as u16;
			let dy = (cell.y as i16 - nearest_y as i16).abs() as u16;
			return min(dx, dy) as u16;
		}
		0
	}
}
 */