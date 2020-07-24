use std::fs::File;
use std::io::BufWriter;

use png::{BitDepth, ColorType, Compression, Encoder, FilterType};

use crate::distance_field::{Cell, CellLayer, DistanceField};
use crate::export::{DistanceFieldExporter, ExportFilter, ExportType};

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
    pub fn new(file_path: &str,
               num_channels: ImageOutputChannels,
               channel_depth: ImageOutputChannelDepth) -> Self {
        Self {
            file_path: String::from(file_path),
            num_channels,
            channel_depth,
        }
    }
}

impl PngOutput {
    fn get_8_bit_distance(&self, cell: &Cell) -> u8 {
        if let Some(distance_squared) = cell.distance_to_nearest_squared() {
            let square_root = (distance_squared as f32).sqrt();
            return if square_root > 255f32 {
                255u8
            } else {
                square_root as u8 //  ^ 0xffu8 to invert
            };
        }
        // TODO: We should think about the best behaviour of the None case here. For now, we just return 0.
        0
    }

    fn get_16_bit_distance(&self, cell: &Cell) -> (u8, u8) {
        if let Some(distance_squared) = cell.distance_to_nearest_squared() {
            let square_root = (distance_squared as f32).sqrt();// * 16f32;
            // let square_root = (cell.distance_to_nearest_squared().unwrap() as f32);

            if square_root > 65535.0f32 {
                (0xFF, 0xFF)
            } else {
                let val = square_root.round() as u16;
                ((val >> 8) as u8, (val & 0xFF) as u8)
            };
        }
        // TODO: We should think about the best behaviour of the None case here. For now, we just return 0.
        (0, 0)
    }

    fn output_df(&self, df: &DistanceField, _export_type: &ExportType) {

        // TODO: handle export_type !

        let e = get_standard_encoder(&self.file_path,
                                     df.width,
                                     df.height,
                                     &self.channel_depth, &self.num_channels);

        let mut writer = e.write_header().unwrap();

        let mut data: Vec<u8> = Vec::new();

        match &self.num_channels {
            ImageOutputChannels::One => {
                self.output_single_channel(df, &mut data)
            }
            ImageOutputChannels::Two => {
                self.output_dual_channel(df, &mut data)
            }
        }

        writer.write_image_data(&data).unwrap(); // Save
    }

    fn output_single_channel(&self, df: &DistanceField, buffer: &mut Vec<u8>) {
        // inner / outer / combined ?

        // combined: add or sdf?

        // 8 bit / 16 bit
        match &self.channel_depth {
            ImageOutputChannelDepth::Eight => {
                df.data.iter().for_each(|cell: &Cell| {
                    // TODO: right now, we just add the inner distances and the outer distances
                    // We should add a feature to generate real 8-bit-signed distance field here!
                    buffer.push(self.get_8_bit_distance(&cell));
                });
            }
            ImageOutputChannelDepth::Sixteen => {
                df.data.iter().for_each(|cell: &Cell| {
                    let (byte_1, byte_2) = self.get_16_bit_distance(&cell);
                    buffer.push(byte_1);
                    buffer.push(byte_2);
                });
            }
            _ => {
                unimplemented!()
            }
        }
    }

    fn output_dual_channel(&self, df: &DistanceField, buffer: &mut Vec<u8>) {
        // inner and outer go on a separate channel
        match &self.channel_depth {
            ImageOutputChannelDepth::Eight => {
                df.data.iter().for_each(|cell: &Cell| {
                    let distance = self.get_8_bit_distance(&cell);
                    match cell.layer {
                        CellLayer::Foreground => {
                            buffer.push(distance);
                            buffer.push(0x00);
                            buffer.push(0x00);
                        }
                        CellLayer::Background => {
                            buffer.push(0x00);
                            buffer.push(distance);
                            buffer.push(0x00);
                        }
                    }
                    // buffer.push(self.get_8_bit_distance(&cell));
                });
            }
            ImageOutputChannelDepth::Sixteen => {
                todo!();
                // mode rgb width 16 bit per channel needed here
            }
            _ => unimplemented!(),
        }
    }

    // export quad channel
}

impl DistanceFieldExporter for PngOutput {
    fn export(&self,
              distance_field: &DistanceField,
              export_type: &ExportType,
              export_filter: &ExportFilter) {
        match export_filter {
            ExportFilter::Background => self.output_df(&DistanceField::filter_outer(distance_field), export_type),
            ExportFilter::Foreground => self.output_df(&DistanceField::filter_outer(distance_field), export_type),
            ExportFilter::All => self.output_df(distance_field, export_type),
        };
    }
}

fn get_standard_encoder(file_path: &str,
                        width: u32,
                        height: u32,
                        channel_depth: &ImageOutputChannelDepth,
                        num_channels: &ImageOutputChannels) -> Encoder<BufWriter<File>> {
    println!("{:?}", file_path);
    let file = File::create(file_path).unwrap();
    let w = BufWriter::new(file);

    let mut e = Encoder::new(w, width, height);
    match num_channels {
        ImageOutputChannels::One => e.set_color(ColorType::Grayscale),
        ImageOutputChannels::Two => e.set_color(ColorType::RGB),
    }
    e.set_compression(Compression::Best);
    e.set_depth(match channel_depth {
        ImageOutputChannelDepth::Eight => BitDepth::Eight,
        ImageOutputChannelDepth::Sixteen => BitDepth::Sixteen,
        _ => unimplemented!(),
    });
    e.set_filter(FilterType::NoFilter); // ???
    e
}


