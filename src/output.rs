use png::{Encoder, ColorType, Compression, BitDepth, FilterType};
use std::path::Path;
use std::io::{BufWriter};
use crate::naive::DistanceField;
use std::fs::File;

pub trait PngExporter<T> {
    fn export(&self, file_path: &Path);
}
impl PngExporter<i8> for DistanceField<i8> {
    fn export(&self, file_path : &Path) {
        // todo!("lala");
        foo();
    }
}

fn foo() {

}
impl PngExporter<u8> for DistanceField<u8> {
    fn export(&self, file_path: &Path) {
        // save_to_png_file(&self, &file_path);

        println!("{:?}", file_path);
        let file = File::create(file_path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut e = Encoder::new(w, self.width, self.height);
        e.set_color(ColorType::Grayscale);
        e.set_compression(Compression::Best);
        e.set_depth(BitDepth::Eight);
        e.set_filter(FilterType::NoFilter); // ???

        let mut writer = e.write_header().unwrap();

        // TODO: In this case, we write the byte content of the DistanceField directly into the image
        // There will be some buffer transformations happen here in future versions

        writer.write_image_data(&self.data).unwrap(); // Save
    }
}


#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use std::fs::{remove_file, create_dir_all, remove_dir};
    use crate::naive::DistanceField;
    use crate::output::PngExporter;

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
    fn generates_file() {
        // should generate a 1x1 pixel grey image
        let d: DistanceField<u8> = DistanceField {
            data: vec![127; 1],
            width: 1,
            height: 1,
        };

        create_temp_dir();

        // TODO: implement the exporter as a type (not as a trait)

        d.export(&get_temp_image_path());

        assert!(get_temp_image_path().is_file());

        delete_temp_image_file();
        delete_temp_dir();
    }
}

