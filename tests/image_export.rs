#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use std::fs::{remove_file, create_dir_all, remove_dir};
    use rs_sdf::data::{DistanceField, Cell, CellLayer};
    use rs_sdf::export::image::{PngOutput, write_the_final_solution};
    use rs_sdf::distance::{DistanceType, DistanceLayer};
    use rs_sdf::data::transformation::{DistanceTransformation, TransformOutputGenerator, TransformationResult};
    use rs_sdf::data::output::TransformationOutputWriter;

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

        // TODO: we should generate a transformation result here

        let out = PngOutput::new(
            &get_temp_image_path().into_os_string().into_string().unwrap());

        let mut dt: DistanceTransformation = DistanceTransformation::from(d);
        dt.filter(DistanceLayer::Combined);
        dt.distance_type(DistanceType::EuclideanDistance);
        dt.scale(0.9); // u8 -> 0 = orig, 1 = 2^1 = orig / 2, 2 = 2^2 = orig / 4, etc...

        let trans:TransformationResult<u8> = dt.transform();

        // TODO: das hier sollte gehen:
        // out.write(trans);

        // stattdessen machen wir das hier
        write_the_final_solution(&trans);


        assert!(get_temp_image_path().is_file());

        delete_temp_image_file();
        delete_temp_dir();
    }
}