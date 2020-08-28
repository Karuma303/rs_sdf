#[cfg(test)]
mod tests {
	use rs_sdf::data::{Cell, CellLayer};
	use rs_sdf::distance::DistanceType;
	use rs_sdf::distance::euclid::{EuclideanDistance, EuclideanDistanceSquared};
	use rs_sdf::distance::OneDimensionalDistanceCalculation;
	use rs_sdf::distance::TwoDimensionalDistanceCalculation;
	use rs_sdf::distance::cartesian::CartesianDistance;
	use rs_sdf::distance::rectilinear::RectilinearDistance;

	fn setup_cell(source_x: u16, source_y: u16, nearest_x: u16, nearest_y: u16) -> Cell {
		Cell {
			nearest_cell_position: Some((nearest_x, nearest_y)),
			x: source_x,
			y: source_y,
			layer: CellLayer::Foreground,
		}
	}

	#[test]
	fn calculate_euclidean_distance() {
		// EuclideanDistance

		let fun = EuclideanDistance::calculate;

		// zero distance
		let c = setup_cell(0, 0, 0, 0);
		let res: u16 = fun(&c);
		assert_eq!(res, 0u16);

		let c = setup_cell(0, 0, 3, 4);
		let res: u16 = fun(&c);
		assert_eq!(res, 5u16);

		let c = setup_cell(3, 4, 0, 0);
		let res: u16 = fun(&c);
		assert_eq!(res, 5u16);

		// TODO: add some test for maximum range here!
	}

	#[test]
	fn calculate_euclidean_distance_squared() {
		// EuclideanDistanceSquared

		let f = EuclideanDistanceSquared::calculate;

		// zero distance
		let c = setup_cell(0, 0, 0, 0);
		let res: u16 = f(&c);
		assert_eq!(res, 0);

		let c = setup_cell(0, 0, 3, 4);
		let res: u16 = f(&c);
		assert_eq!(res, 25);

		let c = setup_cell(3, 4, 0, 0);
		let res: u16 = f(&c);
		assert_eq!(res, 25);

		// TODO: add some test for maximum range here!
	}

	#[test]
	fn calculate_chebyshev_distance() {
		unimplemented!()
	}

	#[test]
	fn calculate_rectilinear_distance() {
		let f = RectilinearDistance::calculate;

		// zero distance
		let c = setup_cell(0, 0, 0, 0);
		let res: u32 = f(&c);
		assert_eq!(res, 0);

		let c = setup_cell(0, 0, 3, 4);
		let res: u32 = f(&c);
		assert_eq!(res, 7);

		let c = setup_cell(3, 4, 0, 0);
		let res: u32 = f(&c);
		assert_eq!(res, 7);

		// TODO: add some test for maximum range here!
	}

	#[test]
	fn calculate_cartesian_distance() {

		// Cartesian Distance

		let f = CartesianDistance::calculate;

		// zero distance
		let c = setup_cell(0, 0, 0, 0);
		let res: (i32, i32) = f(&c);
		assert_eq!(res, (0, 0));

		let c = setup_cell(0, 0, 3, 4);
		let res: (i32, i32) = f(&c);
		assert_eq!(res, (3, 4));

		let c = setup_cell(3, 4, 0, 0);
		let res: (i32, i32) = f(&c);
		assert_eq!(res, (-3, -4));

		// TODO: add some test for maximum range here!
	}

	#[test]
	fn get_nearest_cell_index() {
		panic!()
	}

	#[test]
	fn get_nearest_cell_index_offset() {
		panic!()
	}

	#[test]
	fn get_nearest_cell_position() {
		panic!()
	}
}