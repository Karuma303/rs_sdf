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
    configuration: ImageOutputConfiguration,
}

impl PngOutput {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: String::from(file_path),
            configuration: ImageOutputConfiguration { channel_depth: ImageOutputChannelDepth::Sixteen }, // default
        }
    }

    pub fn configuration(&mut self, configuration: ImageOutputConfiguration) {
        self.configuration = configuration;
    }
}

impl OutputWriter for PngOutput {
    fn write(&self, trans_result: TransformationResult::<u8>) {
        let channel_def = match trans_result.channels.len() {
            1 => ImageOutputChannels::One,
            2 => ImageOutputChannels::Two,
            _ => panic!("not a valid number of output channels"),
        };

        let e = get_standard_encoder(&self.file_path,
                                     trans_result.width,
                                     trans_result.height,
                                     &self.configuration.channel_depth,
                                     &channel_def);

        let mut writer = e.write_header().unwrap();

        let mut image_data_buffer: Vec<u8> = Vec::new();

        // TODO: this must come from the transformation result later!
        let distance_type = DistanceType::EuclideanDistance;

        match channel_def {
            ImageOutputChannels::One => {
                self.output_single_channel_u8(&trans_result.channels[0], &mut image_data_buffer)
            }
            ImageOutputChannels::Two => {
                self.output_dual_channel_u8(&trans_result.channels[0],
                                            &trans_result.channels[1],
                                            &mut image_data_buffer)
            }
        }

        writer.write_image_data(&image_data_buffer).unwrap(); // Save
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


    fn output_single_channel_u8<T: From<u8>>(&self, channel_data: &Vec<T>, buffer: &mut Vec<u8>) {
        channel_data.iter().for_each(|value: u8| {
            buffer.push(value);
        });
    }

    fn output_single_channel_u16<T: From<u16>>(&self, channel_data: &Vec<T>, buffer: &mut Vec<u16>) {
        channel_data.iter().for_each(|value: u16| {
            todo!()
            // buffer.push(value);
        });
    }

    fn output_single_channel_u32<T: From<u32>>(&self, channel_data: &Vec<T>, buffer: &mut Vec<u32>) {
        channel_data.iter().for_each(|value: u32| {
            todo!()
            // buffer.push(value);
        });
    }

    fn output_dual_channel_u8<T: From<u8>>(&self,
                                           channel_one_data: &Vec<T>,
                                           channel_two_data: &Vec<T>,
                                           _buffer: &mut Vec<u8>) {
        let zipped = channel_one_data.iter().zip(channel_two_data).for_each(|(a, b)| {
            _buffer.push(u8::from(a));
            _buffer.push(u8::from(b));
        });
    }

    fn output_dual_channel_u16<T: From<u16>>(&self,
                                             _channel_one_data: &Vec<T>,
                                             _channel_two_data: &Vec<T>,
                                             _buffer: &mut Vec<u16>) {
        todo!()
    }

    fn output_dual_channel_u32<T: From<u32>>(&self,
                                             _channel_one_data: &Vec<T>,
                                             _channel_two_data: &Vec<T>,
                                             _buffer: &mut Vec<u32>) {
        todo!()
    }


    // TODO: make this generic
    fn output_single_channel(&self, channel_data: &Vec<u32>, distance_type: &DistanceType, buffer: &mut Vec<u8>) {
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


