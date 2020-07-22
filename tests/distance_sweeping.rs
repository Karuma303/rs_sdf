#[cfg(test)]
mod tests {
    use rs_sdf::source::SourceField;
    use rs_sdf::processor::sweep::EightSideSweepProcessor;
    use rs_sdf::distance_field::{SourceProcessor};

    // helper method to get an empty 1x1 source field
    fn get_source_1_1_empty() -> SourceField {
        SourceField::from_booleans(&[
            false,
        ], 1, 1)
    }

    // helper method to get an filled 1x1 source field
    fn get_source_1_1_filled() -> SourceField {
        SourceField::from_booleans(&[
            true,
        ], 1, 1)
    }

    // helper method to get an 2x2 source field width a checkered pattern
    fn get_source_2_2_checker() -> SourceField {
        SourceField::from_booleans(&[true, false, false, true], 2, 2)
    }

    // helper method to get an empty 3x3 source field
    fn get_source_3_3_empty() -> SourceField {
        SourceField::from_bytes(&[
            0, 0, 0,
            0, 0, 0,
            0, 0, 0,
        ], 127, 3, 3)
    }

    // helper method to get a filled 3x3 source field
    fn get_source_3_3_filled() -> SourceField {
        SourceField::from_bytes(&[
            255, 255, 255,
            255, 255, 255,
            255, 255, 255,
        ], 127, 3, 3)
    }

    // helper method to get a 3x3 source field with just a centered dot (other cells are empty)
    fn get_source_3_3_centered_dot() -> SourceField {
        SourceField::from_booleans(&[
            false, false, false,
            false, true, false,
            false, false, false], 3, 3)
    }

    #[test]
    fn correct_nearest_cells_for_1x1_source() {
        let s = get_source_1_1_empty();
        let p = EightSideSweepProcessor {};
        let df = p.process(&s);

        assert!(df.data[0].nearest_cell_position.is_none());

        let s = get_source_1_1_filled();
        let p = EightSideSweepProcessor {};
        let df = p.process(&s);

        assert!(df.data[0].nearest_cell_position.is_none());
    }

    #[test]
    fn correct_nearest_cells_for_2x2_checker() {
        let s = get_source_2_2_checker();
        let p = EightSideSweepProcessor {};
        let df = p.process(&s);

        // we cannot make assumptions here, what the exact nearest cell for each
        // point in the field will be, so we just check for not equality
        // with the cell itself and the other cell from the same layer.

        assert!(df.data[0].nearest_cell_position.is_some());
        assert!(df.data[1].nearest_cell_position.is_some());
        assert!(df.data[2].nearest_cell_position.is_some());
        assert!(df.data[3].nearest_cell_position.is_some());

        assert_ne!(df.data[0].nearest_cell_position.unwrap(), (1, 1));
        assert_ne!(df.data[0].nearest_cell_position.unwrap(), (0, 0));

        assert_ne!(df.data[1].nearest_cell_position.unwrap(), (1, 0));
        assert_ne!(df.data[1].nearest_cell_position.unwrap(), (0, 1));

        assert_ne!(df.data[2].nearest_cell_position.unwrap(), (1, 0));
        assert_ne!(df.data[2].nearest_cell_position.unwrap(), (0, 1));

        assert_ne!(df.data[3].nearest_cell_position.unwrap(), (1, 1));
        assert_ne!(df.data[3].nearest_cell_position.unwrap(), (0, 0));
    }

    #[test]
    fn correct_nearest_cells_for_3x3_empty() {
        let s = get_source_3_3_empty();
        let p = EightSideSweepProcessor {};
        let df = p.process(&s);

        // totally empty or filled source fields will generate no meaningful output
        // because we cannot detect where the nearest cells are
        for n in 0..9 {
            assert_eq!(df.data[n].nearest_cell_position, None);
        }
    }

    #[test]
    fn correct_nearest_cells_for_3x3_filled() {
        let s = get_source_3_3_filled();
        let p = EightSideSweepProcessor {};
        let df = p.process(&s);

        // totally empty or filled source fields will generate no meaningful output
        // because we cannot detect where the nearest cells are
        for n in 0..9 {
            assert_eq!(df.data[n].nearest_cell_position, None);
        }
    }

    #[test]
    fn correct_nearest_cells_for_single_centered_dot() {
        let b = vec![
            false, false, false,
            false, true, false,
            false, false, false];
        let s = SourceField::from_booleans(&b, 3, 3);
        let processor = EightSideSweepProcessor {};
        let df = processor.process(&s);

        assert_eq!(df.data[0].get_nearest_cell_position(), Some((1, 1)));
        assert_eq!(df.data[1].get_nearest_cell_position(), Some((1, 1)));
        assert_eq!(df.data[2].get_nearest_cell_position(), Some((1, 1)));

        assert_eq!(df.data[3].get_nearest_cell_position(), Some((1, 1)));
        // because the center "dot" has many nearest cells we cannot
        // predict the exact one that was set during sweeping,
        // so we just test for is_some() here!
        assert!(df.data[4].get_nearest_cell_position().is_some());
        assert_eq!(df.data[5].get_nearest_cell_position(), Some((1, 1)));

        assert_eq!(df.data[6].get_nearest_cell_position(), Some((1, 1)));
        assert_eq!(df.data[7].get_nearest_cell_position(), Some((1, 1)));
        assert_eq!(df.data[8].get_nearest_cell_position(), Some((1, 1)));
    }
}