use png::{Encoder, ColorType, Compression, BitDepth, FilterType};
use std::path::Path;
use std::io::{BufWriter};
use std::fs::File;
use crate::df::{DistanceField, Cell};
use std::cmp::{max, min};

pub trait FieldOutput {
    fn output(&self, df: DistanceField);
}

pub struct PngOutput {
    file_path: String,
}

impl PngOutput {
    pub fn new(file_path: &String) -> Self {
        Self {
            file_path: String::from(file_path),
        }
    }
}

impl FieldOutput for PngOutput {

    // TODO: we should add more options here!
    // - What to output (inner, outer, both)
    // - single channel, multi channel
    // ...

    fn output(&self, df: DistanceField) {
        let e = get_standard_encoder(&self.file_path, df.width, df.height);

        let mut writer = e.write_header().unwrap();

        let data = df.data.into_iter().map(|cell: Cell| {
            // TODO: what to to if distance is none?
            // max((cell.distance_to_nearest_squared().unwrap() as f64).sqrt() as u8, 255) as u8
            let square_root = (cell.distance_to_nearest_squared().unwrap() as f32).sqrt();
            // let val = min(square_root, 255f32);
            if square_root > 255f32 { 255u8 } else {
                square_root as u8
            }
        }).collect::<Vec<u8>>();

        writer.write_image_data(&data).unwrap(); // Save
    }

    /*
    fn output_i8(&self, df : DistanceField<i8>) {
        let e = get_standard_encoder(&self.file_path, df.width, df.height);

        let mut writer = e.write_header().unwrap();

        let dest = &df.data
            .iter()
            .map(|element| element.clone() as u8)
            .collect::<Vec<u8>>();
        writer.write_image_data(&dest).unwrap(); // Save
    }
    */
}

fn get_standard_encoder(file_path: &String, width: u32, height: u32) -> Encoder<BufWriter<File>> {
    println!("{:?}", file_path);
    let file = File::create(file_path).unwrap();
    let mut w = BufWriter::new(file);

    let mut e = Encoder::new(w, width, height);
    e.set_color(ColorType::Grayscale);
    e.set_compression(Compression::Best);
    e.set_depth(BitDepth::Eight);
    e.set_filter(FilterType::NoFilter); // ???
    e
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use std::fs::{remove_file, create_dir_all, remove_dir};
    use crate::output::{PngOutput, FieldOutput};
    use crate::df::{DistanceField, Cell, CellLayer};

    const TEMP_DIR: &str = r"__tmp__output__dir__/";
    const TEMP_IMAGE_FILE: &str = r"image.png";

    fn create_temp_dir() {
        let p = Path::new(TEMP_DIR);
        create_dir_all(p);
    }

    fn delete_temp_dir() {
        let p = Path::new(TEMP_DIR);
        remove_dir(p);
    }

    fn delete_temp_image_file() {
        remove_file(get_temp_image_path());
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

        let out = PngOutput::new(&get_temp_image_path().into_os_string().into_string().unwrap());
        out.output(d);

        assert!(get_temp_image_path().is_file());

        delete_temp_image_file();
        delete_temp_dir();
    }
}

