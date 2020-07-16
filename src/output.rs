use png::{Encoder, ColorType, Compression, BitDepth, FilterType};
use std::io::{BufWriter};
use std::fs::File;
use crate::df::{DistanceField, Cell};

pub trait FieldOutput {
    fn output(&self, df: DistanceField);
}

pub enum ImageOutputChannelDepth {
    Eight = 8,
    Sixteen = 16,
    ThirtyTwo = 32,
}

pub enum ImageOutputChannels {
    One = 1,
    Two = 2,
}

pub struct PngOutput {
    file_path: String,
    channel_depth: ImageOutputChannelDepth,
    num_channels: ImageOutputChannels,
}

impl PngOutput {
    pub fn new(file_path: &String,
               num_channels: ImageOutputChannels,
               channel_depth: ImageOutputChannelDepth) -> Self {
        Self {
            file_path: String::from(file_path),
            num_channels,
            channel_depth,
        }
    }
}

impl FieldOutput for PngOutput {

    // TODO: we should add more options here!
    // - What to output (inner, outer, both)
    // - single channel, multi channel
    // ...

    fn output(&self, df: DistanceField) {
        let e = get_standard_encoder(&self.file_path, df.width, df.height, &self.channel_depth);

        let mut writer = e.write_header().unwrap();

        let mut data: Vec<u8> = Vec::new();

        match &self.channel_depth {
            ImageOutputChannelDepth::Eight => {
                df.data.into_iter().for_each(|cell: Cell| {
                    // TODO: There might be cases, where the distance is None. How do we handle that?
                    let square_root = (cell.distance_to_nearest_squared().unwrap() as f32).sqrt();
                    // let val = min(square_root, 255f32);
                    data.push(if square_root > 255f32 { 255u8 } else {
                        square_root as u8
                    });
                });
            }
            ImageOutputChannelDepth::Sixteen => {
                df.data.into_iter().for_each(|cell: Cell| {
                    // TODO: There might be cases, where the distance is None. How do we handle that?
                    let square_root = (cell.distance_to_nearest_squared().unwrap() as f32).sqrt();// * 16f32;
                    // let square_root = (cell.distance_to_nearest_squared().unwrap() as f32);

                    if square_root > 65535.0f32 {
                        data.push(0xFFu8);
                        data.push(0xFFu8);
                    } else {
                        let val = square_root.round() as u16;
                        data.push((val >> 8) as u8);
                        data.push((val & 0xFF) as u8);
                    }
                });
            }
            _ => {
                unimplemented!()
            }
        }


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

fn get_standard_encoder(file_path: &String, width: u32, height: u32, channel_depth: &ImageOutputChannelDepth) -> Encoder<BufWriter<File>> {
    println!("{:?}", file_path);
    let file = File::create(file_path).unwrap();
    let w = BufWriter::new(file);

    let mut e = Encoder::new(w, width, height);
    e.set_color(ColorType::Grayscale);
    e.set_compression(Compression::Best);
    e.set_depth(match channel_depth {
        ImageOutputChannelDepth::Eight => BitDepth::Eight,
        ImageOutputChannelDepth::Sixteen => BitDepth::Sixteen,
        _ => unimplemented!(),
    });
    e.set_filter(FilterType::NoFilter); // ???
    e
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use std::fs::{remove_file, create_dir_all, remove_dir};
    use crate::output::{PngOutput, FieldOutput, ImageOutputChannels, ImageOutputChannelDepth};
    use crate::df::{DistanceField, Cell, CellLayer};

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
        out.output(d);

        assert!(get_temp_image_path().is_file());

        delete_temp_image_file();
        delete_temp_dir();
    }
}

