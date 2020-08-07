use std::fs::File;
use std::io::BufWriter;

use png::{BitDepth, ColorType, Compression, Encoder, FilterType};

use crate::data::{Cell, CellLayer, DistanceField};
use crate::export::{DistanceFieldExporter};
use crate::distance::{DistanceType, DistanceLayer};
use crate::data::output::OutputWriter;
use crate::data::transformation::TransformationResult;
use crate::utils::u16_to_u8_clamped;

// Todo: We have to add to the image exporter something like a color channel definition,
// that maps the export channels to color channels

pub struct ImageOutputConfiguration {
    channel_depth: ImageOutputChannelDepth,
}

impl ImageOutputConfiguration {
    pub fn new(channel_depth: ImageOutputChannelDepth) -> Self {
        Self {
            channel_depth,
        }
    }

    pub fn channel_depth(&mut self, channel_depth: ImageOutputChannelDepth) {
        self.channel_depth = channel_depth;
    }
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
    // deprecated
    num_channels: ImageOutputChannels,
    // TODO: do we still need that?
    configuration: ImageOutputConfiguration,
}

impl PngOutput {
    pub fn new(file_path: &str,
               num_channels: ImageOutputChannels,
    ) -> Self {
        Self {
            file_path: String::from(file_path),
            num_channels,
            configuration: ImageOutputConfiguration { channel_depth: ImageOutputChannelDepth::Sixteen }, // default
        }
    }

    pub fn configuration(&mut self, configuration: ImageOutputConfiguration) {
        self.configuration = configuration;
    }
}

impl OutputWriter for PngOutput {
    fn write(&self, output: TransformationResult) {
        // TODO !
        unimplemented!()
    }
}

impl PngOutput {

    // deprecated! will be replaced by trait method 'write' !
    fn output_df(&self, df: &DistanceField, distance_type: &DistanceType) {

        // TODO: handle export_type !

        let e = get_standard_encoder(&self.file_path,
                                     df.width,
                                     df.height,
                                     &self.configuration.channel_depth,
                                     &self.num_channels);

        let mut writer = e.write_header().unwrap();

        let mut data: Vec<u8> = Vec::new();

        match &self.num_channels {
            ImageOutputChannels::One => {
                self.output_single_channel(df, &distance_type, &mut data)
            }
            ImageOutputChannels::Two => {
                self.output_dual_channel(df, &distance_type, &mut data)
            }
        }

        writer.write_image_data(&data).unwrap(); // Save
    }


    fn output_single_channel(&self, df: &DistanceField, distance_type: &DistanceType, buffer: &mut Vec<u8>) {
        // inner / outer / combined ?

        // combined: add or sdf?

        // 8 bit / 16 bit
        match &self.configuration.channel_depth {
            ImageOutputChannelDepth::Eight => {
                let function = distance_type.calculation_function();
                df.data.iter().for_each(|cell: &Cell| {
                    // TODO: right now, we just add the inner distances and the outer distances
                    // We should add a feature to generate real 8-bit-signed distance field here!
                    // buffer.push(self.get_8_bit_distance(&cell));
                    buffer.push(u16_to_u8_clamped(function(&cell)));
                });
            }
            ImageOutputChannelDepth::Sixteen => {
                let function = distance_type.calculation_function();
                df.data.iter().for_each(|cell: &Cell| {
                    let distance = function(&cell);
                    buffer.push((distance >> 8) as u8);
                    buffer.push((distance & 0xFF) as u8);
                });
            }
            _ => {
                // TODO: we have to implement the 32 bit output (use for example for squared distance output)
                unimplemented!()
            }
        }
    }

    fn output_dual_channel(&self, df: &DistanceField, distance_type: &DistanceType, buffer: &mut Vec<u8>) {
        // inner and outer go on a separate channel
        match &self.configuration.channel_depth {
            ImageOutputChannelDepth::Eight => {
                let function = distance_type.calculation_function();
                df.data.iter().for_each(|cell: &Cell| {
                    // let distance = self.get_8_bit_distance(&cell);
                    let distance = u16_to_u8_clamped(function(&cell));
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
                });
            }
            ImageOutputChannelDepth::Sixteen => {
                let function = distance_type.calculation_function();
                df.data.iter().for_each(|cell: &Cell| {
                    // let distance = self.get_8_bit_distance(&cell);
                    let distance = function(&cell);
                    let higher_byte = (distance >> 8) as u8;
                    let lower_byte = (distance & 0xFF) as u8;
                    match cell.layer {
                        CellLayer::Foreground => {
                            buffer.push(higher_byte);
                            buffer.push(lower_byte);
                            buffer.push(0x00);
                            buffer.push(0x00);
                            buffer.push(0x00);
                            buffer.push(0x00);
                        }
                        CellLayer::Background => {
                            buffer.push(0x00);
                            buffer.push(0x00);
                            buffer.push(higher_byte);
                            buffer.push(lower_byte);
                            buffer.push(0x00);
                            buffer.push(0x00);
                        }
                    }
                });

                // TODO: we have to implement the 32 bit output (use for example for squared distance output)
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
              distance_type: &DistanceType,
              export_filter: &DistanceLayer) {
        match export_filter {
            DistanceLayer::Background => self.output_df(&DistanceField::filter_outer(distance_field), distance_type),
            DistanceLayer::Foreground => self.output_df(&DistanceField::filter_outer(distance_field), distance_type),
            DistanceLayer::Combined => self.output_df(distance_field, distance_type),
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


