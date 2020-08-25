use crate::data::{DistanceField};
use crate::distance::{DistanceType, DistanceLayer};

/// Module for image-based export (PNG)
pub mod image;

pub struct ExportData {
	//    pub channels: Vec<Box<dyn ExportChannel>>,
	pub channels: Vec<ExportChannel>,
}

pub struct ExportChannel {
	pub data: Vec<u8>,
	pub bit_depth: BitDepth,
	pub number_type: NumberType,
}

impl ExportChannel {
	fn new(num_elements: usize, bit_depth: BitDepth, number_type: NumberType) -> Self {
		match bit_depth {
			BitDepth::Eight => {
				ExportChannel { data: Vec::<u8>::with_capacity(num_elements), bit_depth, number_type }
			}
			BitDepth::Sixten => {
				ExportChannel { data: Vec::<u8>::with_capacity(num_elements * 2), bit_depth, number_type }
			}
			BitDepth::ThirtyTwo => {
				ExportChannel { data: Vec::<u8>::with_capacity(num_elements * 4), bit_depth, number_type }
			}
		}
	}
}

/*
pub trait ExportChannel {
    fn data(&self) -> Vec<u8>;
    fn bit_depth(&self) -> BitDepth;
    fn number_type(&self) -> NumberType;
}
*/

/*
impl <T> ExportChannel<T> for Channel<T> {
    fn data(&self) -> Vec<T> {
        unimplemented!()
    }

    fn bit_depth(&self) -> BitDepth {
        unimplemented!()
    }

    fn number_type(&self) -> NumberType {
        unimplemented!()
    }
}
 */

/*
impl ExportChannel for Channel<u8> {
    // Implement methods here
    fn bit_depth(&self) -> BitDepth {
        BitDepth::Eight
    }

    fn number_type(&self) -> NumberType {
        NumberType::Unsigned
    }
}

impl ExportChannel for Channel<u16> {
    // Implement methods here
    fn bit_depth(&self) -> BitDepth {
        BitDepth::Sixten
    }

    fn number_type(&self) -> NumberType {
        NumberType::Unsigned
    }
}

impl ExportChannel for Channel<u32> {
    // Implement methods here
    fn bit_depth(&self) -> BitDepth {
        BitDepth::ThirtyTwo
    }

    fn number_type(&self) -> NumberType {
        NumberType::Unsigned
    }
}
*/

pub struct Channel<T> {
	//    bit_depth: BitDepth,
//    number_type: NumberType,
	pub data: Vec<T>,

}

/*
impl<T> Channel<T> {
    fn new(num_elements: usize) -> Self {
        Channel::<T> {
            data: Vec::with_capacity(num_elements),
        }
    }
}
*/
/*
impl Channel<u16> {
    /*
        fn new() -> Self {
            Channel::<u16> {
                data: Vec::new(),
            }
        }

     */
}
*/
#[derive(PartialEq, Clone)]
pub enum BitDepth {
	Eight,
	Sixten,
	ThirtyTwo,
}

#[derive(PartialEq)]
pub enum NumberType {
	Integer,
	Unsigned,
	Float,
}

/// Exporter for generated data
pub trait Exporter {
	fn export(&self, data: &ExportData);
}

// TODO: we need some kind of converter to turn a DistanceField into ExportData
pub trait Converter {
//    fn convert(distance_field: &DistanceField,
//               distance_type: &DistanceType,
//               distance_layer: &DistanceLayer,
//    ) -> ExportData;
}

// struct DistanceConverter;

impl dyn Converter /* for DistanceConverter */ {
	pub fn convert(distance_field: &DistanceField,
				   distance_type: &DistanceType,
				   distance_layer: &DistanceLayer,
				   bit_depth: BitDepth) -> ExportData {

		// The number of channels is dependent of the distance type
		// TODO: set the number of channels !
		let mut channels: Vec<ExportChannel> = Vec::new();

		let dimensions = distance_type.dimensions();

		match dimensions {
			1 => {
				let channel = ExportChannel::new(1, bit_depth, NumberType::Unsigned);
				// ... distance_type.calculation_function()
				// TODO: iterate, use calc function, put data into channel (use closure here)
				// TODO: fill the values according to the bit_depth
				channels.push(channel);
			}
			2 => {
				let channel_one = ExportChannel::new(1, bit_depth.clone(), NumberType::Unsigned);
				let channel_two = ExportChannel::new(1, bit_depth, NumberType::Unsigned);
				// TODO: iterate, use calc function, put data into channel (use closure here)
				// TODO: fill the values according to the bit_depth
				channels.push(channel_one);
				channels.push(channel_two);
			}
			_ => panic!("converter cannot handle {} dimensions", dimensions),
		}


		ExportData {
			channels,
		}
	}
}

/// Exporter for a distance field
pub trait DistanceFieldExporter {
	fn export(
		&self,
		distance_field: &DistanceField,
		export_type: &DistanceType,
		export_filter: &DistanceLayer,
	);
}

// ******************************************************
// new stuff - a lot of other code will be deprecated !!!
// ******************************************************

pub struct ExportBufferData {
	data: Vec<u8>,
	bit_depth: BitDepth,
	num_channels: u8,
	width: usize,
	height: usize,
}

// must be implemented by the providers
pub trait ExportBufferDataProvider {
	fn export_buffer_data(&self) -> ExportBufferData;
}


// png export
// raw export
// ... and more...
pub trait DistanceDataExporter {
	fn export_data(&self, export_buffer_data: &ExportBufferData);
}

pub struct RawExporter {
	pub file_path: String,
}

pub struct PngExporter {
	pub file_path: String,
}

impl DistanceDataExporter for PngExporter {
	fn export_data(&self, export_buffer_data: &ExportBufferData) {
		println!("Exporting png file '{}' (width:{}, height:{}, size[byte]:{})",
				 self.file_path,
				 export_buffer_data.width,
				 export_buffer_data.height,
				 export_buffer_data.data.len());
		// TODO: implement real png output
	}
}

impl DistanceDataExporter for RawExporter {
	fn export_data(&self, export_buffer_data: &ExportBufferData) {
		println!("Exporting raw file '{}' (width:{}, height:{}, size[byte]:{})",
				 self.file_path,
				 export_buffer_data.width,
				 export_buffer_data.height,
				 export_buffer_data.data.len());

		// TODO: implement real raw output
	}
}