#[cfg(test)]
mod tests {
	use rs_sdf::data::{Cell, CellLayer, CellPosition};
	use rs_sdf::distance::cartesian::CartesianDistance;
	use rs_sdf::distance::chebyshev::ChebyshevDistance;
	use rs_sdf::distance::euclid::{EuclideanDistance, EuclideanDistanceSquared};
	use rs_sdf::distance::nearest_cell::{NearestCellPosition, NearestCellIndex};
	use rs_sdf::distance::OneDimensionalDistanceCalculation;
	use rs_sdf::distance::rectilinear::RectilinearDistance;
	use rs_sdf::distance::TwoDimensionalDistanceCalculation;

	fn setup_cell(index: u32, source_x: u16, source_y: u16, nearest_x: u16, nearest_y: u16, nearest_index: u32) -> Cell {
		Cell {
			nearest_cell_position: Some(CellPosition {
				x: nearest_x,
				y: nearest_y,
				index: nearest_index,
			}),
			index,
			x: source_x,
			y: source_y,
			layer: CellLayer::Foreground,
		}
	}

	#[test]
	fn calculate_euclidean_distance() {
		// EuclideanDistance

		let fun = EuclideanDistance::calculate_legacy;

		let dummy_index = 0;

		// zero distance
		let c = setup_cell( 0,0, 0, 0, 0, dummy_index);
		let res: u16 = fun(&c);
		assert_eq!(res, 0u16);

		let c = setup_cell(0, 0, 0, 3, 4, dummy_index);
		let res: u16 = fun(&c);
		assert_eq!(res, 5u16);

		let c = setup_cell(0, 3, 4, 0, 0, dummy_index);
		let res: u16 = fun(&c);
		assert_eq!(res, 5u16);

		// TODO: add some test for maximum range here!
	}

	#[test]
	fn calculate_euclidean_distance_squared() {
		// EuclideanDistanceSquared

		let f = EuclideanDistanceSquared::calculate_legacy;

		let dummy_index = 0;

		// zero distance
		let c = setup_cell(0,0, 0, 0, 0, dummy_index);
		let res: u16 = f(&c);
		assert_eq!(res, 0);

		let c = setup_cell(0,0, 0, 3, 4, dummy_index);
		let res: u16 = f(&c);
		assert_eq!(res, 25);

		let c = setup_cell(0,3, 4, 0, 0, dummy_index);
		let res: u16 = f(&c);
		assert_eq!(res, 25);

		// TODO: add some test for maximum range here!
	}

	#[test]
	fn calculate_chebyshev_distance() {
		let f = ChebyshevDistance::calculate_legacy;

		let dummy_index = 0;

		// zero distance
		let c = setup_cell(0,0, 0, 0, 0, dummy_index);
		let res: u16 = f(&c);
		assert_eq!(res, 0);

		let c = setup_cell(0,0, 0, 3, 4, dummy_index);
		let res: u16 = f(&c);
		assert_eq!(res, 4);

		let c = setup_cell(0,3, 4, 0, 0, dummy_index);
		let res: u16 = f(&c);
		assert_eq!(res, 4);

		// TODO: add some test for maximum range here!
	}

	#[test]
	fn calculate_rectilinear_distance() {
		let f = RectilinearDistance::calculate_legacy;

		let dummy_index = 0;

		// zero distance
		let c = setup_cell(0,0, 0, 0, 0, dummy_index);
		let res: u32 = f(&c);
		assert_eq!(res, 0);

		let c = setup_cell(0,0, 0, 3, 4, dummy_index);
		let res: u32 = f(&c);
		assert_eq!(res, 7);

		let c = setup_cell(0,3, 4, 0, 0, dummy_index);
		let res: u32 = f(&c);
		assert_eq!(res, 7);

		// TODO: add some test for maximum range here!
	}

	#[test]
	fn calculate_cartesian_distance() {

		// Cartesian Distance

		let f = CartesianDistance::calculate_legacy;

		let dummy_index = 0;

		// zero distance
		let c = setup_cell(0,0, 0, 0, 0, dummy_index);
		let res: (i32, i32) = f(&c);
		assert_eq!(res, (0, 0));

		let c = setup_cell(0,0, 0, 3, 4, dummy_index);
		let res: (i32, i32) = f(&c);
		assert_eq!(res, (3, 4));

		let c = setup_cell(0,3, 4, 0, 0, dummy_index);
		let res: (i32, i32) = f(&c);
		assert_eq!(res, (-3, -4));

		// TODO: add some test for maximum range here!
	}

	#[test]
	fn get_nearest_cell_index() {
		// Nearest Cell Index

		let f = NearestCellIndex::calculate_legacy;

		let c = setup_cell(0,0, 0, 0, 0, 99);
		let res: u32 = f(&c);
		assert_eq!(res, 99);
	}

	#[test]
	fn get_nearest_cell_position() {

		// Nearest Cell Position

		let f = NearestCellPosition::calculate_legacy;

		let c = setup_cell(0,0, 0, 99, 99, 0);
		let res: (u16, u16) = f(&c);
		assert_eq!(res, (99, 99));
	}
}