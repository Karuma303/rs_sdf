#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use std::fs::{remove_file, create_dir_all, remove_dir};
    use rs_sdf::distance_field::{DistanceField, Cell, CellLayer};
    use rs_sdf::export::image::{PngOutput, ImageOutputChannels, ImageOutputChannelDepth};
    use rs_sdf::export::{DistanceFieldExporter, ExportType};

    const TEMP_DIR: &str = r"__tmp__output__dir__/";
    const TEMP_IMAGE_FILE: &str = r"image.png";

    fn create_temp_dir() {
        let p = Path::new(TEMP_DIR);
        create_dir_all(p).unwrap();
    }

    fn delete_temp_dir() {
        let p = Path::new(TEMP_DIR);
        remove_dir(p).unwrap();
    }

    fn delete_temp_image_file() {
        remove_file(get_temp_image_path()).unwrap();
    }

    fn get_temp_image_path() -> PathBuf {
        let mut b = PathBuf::new();
        b.push(Path::new(TEMP_DIR));
        b.push(Path::new(TEMP_IMAGE_FILE));
        b
    }

    #[test]
    fn generates_png_file() {
        // should generate a 1x1 pixel grey image
        let d: DistanceField = DistanceField {
            data: vec![Cell::new(CellLayer::Foreground, 90, 90); 1], // Foreground(Distance::new(180, 180))
            width: 1,
            height: 1,
        };

        create_temp_dir();

        // TODO: implement the exporter as a type (not as a trait)

        let out = PngOutput::new(
            &get_temp_image_path().into_os_string().into_string().unwrap(),
            ImageOutputChannels::One,
            ImageOutputChannelDepth::Eight);
        out.export(&d, &ExportType::EuclideanDistance);

        assert!(get_temp_image_path().is_file());

        delete_temp_image_file();
        delete_temp_dir();
    }
}