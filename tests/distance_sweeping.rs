#[cfg(test)]
mod tests {
    use rs_sdf::source::SourceField;
    use rs_sdf::processor::sweep::EightSideSweepProcessor;
    use rs_sdf::generator::DistanceGenerator;
    use rs_sdf::distance_field::{SourceProcessor};

    // helper method to get an empty source field
    fn get_source_0_0() -> SourceField {
        SourceField::from_booleans(&[], 0, 0)
    }

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

    /* TODO: reactivate this tests !
    #[test]
    fn generates_buffer_with_additional_border() {
        let b_1x1_empty = init_buffer(&get_source_1_1_empty(), 0, 0);
        assert_eq!(b_1x1_empty.len(), 3 * 3);

        let b_1x1_filled = init_buffer(&get_source_1_1_filled(), 0, 0);
        assert_eq!(b_1x1_filled.len(), 3 * 3);

        let b_2x2 = init_buffer(&get_source_2_2_checker(), 0, 0);
        assert_eq!(b_2x2.len(), 4 * 4);
    }
     */

    /* TODO: reactivate this tests
    #[test]
    fn get_filled_buffer_for_outer_distance() {
        let b = init_buffer_for_outer_distances(&get_source_2_2_checker());
        let m = u8::MAX;
        assert_eq!(b, [
            m, m, m, m,
            m, 0, m, m,
            m, m, 0, m,
            m, m, m, m,
        ]);
    }
     */

    /* TODO: reactivate this tests
    #[test]
    fn get_filled_buffer_for_inner_distance() {
        let b = init_buffer_for_inner_distances(&get_source_2_2_checker());
        let m = u8::MAX;
        assert_eq!(b, [
            0, 0, 0, 0,
            0, m, 0, 0,
            0, 0, m, 0,
            0, 0, 0, 0,
        ]);
    }
     */

    /* TODO: reactivate this tests
    #[test]
    fn gets_correct_distance_field_size_from_oversize_buffer() {
        let b_filled = init_buffer_for_outer_distances(&get_source_1_1_filled());
        let df_filled = get_df_from_buffer(&b_filled, 1, 1);
        assert_eq!(df_filled.data.len(), 1);
        assert_eq!(df_filled.data[0], 0);

        let b_empty = init_buffer_for_outer_distances(&get_source_1_1_empty());
        let df_filled = get_df_from_buffer(&b_empty, 1, 1);
        assert_eq!(df_filled.data.len(), 1);
        assert_eq!(df_filled.data[0], u8::MAX);
    }
     */

    /* TODO: reactivate this tests
    #[test]
    fn generates_outer_distance_field() {
        let df_checker = generate_outer_df(&get_source_2_2_checker());
        assert_eq!(df_checker.data, vec![0, 1, 1, 0]);

        let df_empty = generate_outer_df(&get_source_1_1_empty());
        assert_eq!(df_empty.data, vec![u8::MAX]);

        let df_empty_big = generate_outer_df(&get_source_3_3_empty());
        assert_eq!(df_empty_big.data, vec![u8::MAX, u8::MAX, u8::MAX,
                                           u8::MAX, u8::MAX, u8::MAX,
                                           u8::MAX, u8::MAX, u8::MAX]);

        let df_filled = generate_outer_df(&get_source_1_1_filled());
        assert_eq!(df_filled.data, vec![0]);

        let df_filled_big = generate_outer_df(&get_source_3_3_filled());
        assert_eq!(df_filled_big.data, vec![0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
     */

    /* TODO: reactivate this tests
    #[test]
    fn generates_inner_distance_field() {
        let df_checker = generate_inner_df(&get_source_2_2_checker());
        assert_eq!(df_checker.data, vec![1, 0, 0, 1]);

        let df_empty = generate_inner_df(&get_source_1_1_empty());
        assert_eq!(df_empty.data, vec![0]);

        let df_empty_big = generate_inner_df(&get_source_3_3_empty());
        assert_eq!(df_empty_big.data, vec![0, 0, 0, 0, 0, 0, 0, 0, 0]);

        let df_filled = generate_inner_df(&get_source_1_1_filled());
        assert_eq!(df_filled.data, vec![1]);

        let df_filled_big = generate_inner_df(&get_source_3_3_filled());
        assert_eq!(df_filled_big.data, vec![1, 1, 1, 1, 2, 1, 1, 1, 1]);
    }
     */

    // TODO: generate signed distance field
// TODO: check for max ranges and clamping
    /*
    #[test]
    fn generates_signed_distance_field_i8_3x3() {
        let b = vec![0, 0, 0, 0, 1, 0, 0, 0, 0];
        let s = SourceField::new(&b,3,3);
        let df = generate_sdf(&s);
        assert!(df.data == vec![2, 1, 2, 1, -1, 1, 2, 1, 2]);
    }
     */
    #[test]
    fn generates_correct_distances() {
        let b = vec![
            false, false, false,
            false, true, false,
            false, false, false];
        let s = SourceField::from_booleans(&b, 3, 3);
        let processor = EightSideSweepProcessor {};
        let df = processor.process(&s);

        assert_eq!(df.width, 3);
        assert_eq!(df.height, 3);

        assert_eq!(df.data[0].get_nearest_cell_position(), Some((1, 1)));

        // TODO: we should add more assertions here !
    }
}