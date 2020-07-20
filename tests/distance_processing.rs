#[cfg(test)]
mod tests {
    use rs_sdf::source::SourceField;

    #[test]
    fn distance_field_initializes_correctly() {
        let sf: SourceField = SourceField {
            width: 2,
            height: 2,
            data: Vec::from([true, false, false, true]),
        };
        assert_eq!(sf.width, 2);
        assert_eq!(sf.height, 2);
        assert_eq!(sf.data[0], true);
        assert_eq!(sf.data[1], false);
        assert_eq!(sf.data[2], false);
        assert_eq!(sf.data[3], true);
        // TODO: add more checks here!
    }
}