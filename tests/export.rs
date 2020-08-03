#[cfg(test)]
mod tests {
    use rs_sdf::data::{DistanceField, Cell, CellLayer};
    use rs_sdf::distance::{DistanceType, DistanceLayer};
    use rs_sdf::export::{Converter, ExportData, NumberType, BitDepth};

    fn get_distance_field() -> DistanceField {
        let c = Cell::new(CellLayer::Foreground, 0, 0);
        DistanceField {
            width: 1,
            height: 1,
            data: vec![c; 1],
        }
    }

    fn convert_to_export_data(distance_type: &DistanceType,
                              bit_depth: BitDepth) -> ExportData {
        Converter::convert(&get_distance_field(),
                           distance_type,
                           &DistanceLayer::Foreground,
                           bit_depth)
    }

    #[test]
    fn converts_distance_field_to_export_data() {
        /*
        let export_data: ExportData = Converter::convert(&get_distance_field(),
                                                         &DistanceType::EuclideanDistance,
                                                         &DistanceLayer::Foreground,
                                                         BitDepth::Eight);
    */
        let export =
            convert_to_export_data(&DistanceType::EuclideanDistance, BitDepth::Eight);

        // one channel
        assert_eq!(export.channels.len(), 1);
        let channel = export.channels.get(0).unwrap();
        assert!(channel.number_type == NumberType::Unsigned);
        assert!(channel.bit_depth == BitDepth::Eight);
        assert_eq!(channel.data.len(), 1);

        let export =
            convert_to_export_data(&DistanceType::EuclideanDistance, BitDepth::Sixten);

        // one channel
        assert_eq!(export.channels.len(), 1);
        let channel = export.channels.get(0).unwrap();
        assert!(channel.number_type == NumberType::Unsigned);
        assert!(channel.bit_depth == BitDepth::Sixten);
        assert_eq!(channel.data.len(), 2);

        let export =
            convert_to_export_data(&DistanceType::EuclideanDistance, BitDepth::ThirtyTwo);

        // one channel
        assert_eq!(export.channels.len(), 1);
        let channel = export.channels.get(0).unwrap();
        assert!(channel.number_type == NumberType::Unsigned);
        assert!(channel.bit_depth == BitDepth::ThirtyTwo);
        assert_eq!(channel.data.len(), 4);

    }
}