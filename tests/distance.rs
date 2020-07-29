#[cfg(test)]
mod tests {
    use rs_sdf::distance::DistanceType;
    use rs_sdf::data::{Cell, CellLayer};

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
        let fun = DistanceType::EuclideanDistance.calculation_function();

        // zero distance
        let c = setup_cell(0, 0, 0, 0);
        assert_eq!(fun(&c), 0u16);

        let c = setup_cell(0, 0, 3, 4);
        assert_eq!(fun(&c), 5u16);

        let c = setup_cell(3, 4, 0, 0);
        assert_eq!(fun(&c), 5u16);

        // TODO: add some test for maximum range here!
    }

    #[test]
    fn calculate_euclidean_distance_squared() {
        let fun = DistanceType::EuclideanDistanceSquared.calculation_function();

        // zero distance
        let c = setup_cell(0, 0, 0, 0);
        assert_eq!(fun(&c), 0u16);

        let c = setup_cell(0, 0, 3, 4);
        assert_eq!(fun(&c), 25u16);

        let c = setup_cell(3, 4, 0, 0);
        assert_eq!(fun(&c), 25u16);

        // TODO: add some test for maximum range here!
    }

    #[test]
    fn calculate_chebyshev_distance() {
        unimplemented!()
    }

    #[test]
    fn calculate_rectilinear_distance() {
        panic!()
    }

    #[test]
    fn calculate_cartesian_distance() {
        panic!()
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