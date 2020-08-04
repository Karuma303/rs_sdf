use bitvec::vec::{BitVec};

/// Spatial input type as a source for distance field generation.
/// An input field is defined by its dimensions and a binary data block.
/// The number of items inside the data block match the dimension of the input.
/// A set bit in the data block specifies a foreground cell, whereas an unset bit
/// specifies a background cell.
#[derive(Debug)]
pub struct InputField {
    pub data: BitVec,
    pub width: u32,
    pub height: u32,
}

struct DimensionalVector<T> {
    data: Vec<T>,
    pub width: u32,
    pub height: u32,
}

impl<T> DimensionalVector<T> {
    pub fn new(data: Vec<T>, width: u32, height: u32) -> Self {
        check_dimensions_and_buffer_size(width, height, data.len());
        DimensionalVector {
            data,
            width,
            height,
        }
    }
}

/// A helper structure to provide input data based on unsigned byte values and a threshold.
/// All values equal or greater than the given threshold define the foreground. All other
/// values define the background.
pub struct ByteInputData {
    buffer: DimensionalVector<u8>,
    threshold: u8,
}

impl ByteInputData {
    pub fn new(buffer: Vec<u8>, threshold: u8, width: u32, height: u32) -> Self {
        Self {
            buffer: DimensionalVector::new(buffer, width, height),
            threshold,
        }
    }
}

pub struct BoolInputData {
    buffer: DimensionalVector<bool>,
}

impl BoolInputData {
    pub fn new(buffer: Vec<bool>, width: u32, height: u32) -> Self {
        Self {
            buffer: DimensionalVector::new(buffer, width, height),
        }
    }
}

/// Implementation of the From trait.
impl From<ByteInputData> for InputField {
    fn from(input: ByteInputData) -> Self {
        let mut data = BitVec::new();
        input.buffer.data.iter().for_each(|elem| data.push(*elem >= input.threshold));
        InputField::new(data, input.buffer.width, input.buffer.height)
    }
}

/// Implementation of the From trait.
impl From<BoolInputData> for InputField {
    fn from(input: BoolInputData) -> Self {
        let mut data = BitVec::new();
        input.buffer.data.iter().for_each(|elem| data.push(*elem));
        InputField::new(data, input.buffer.width, input.buffer.height)
    }
}

/// Implementation of the InputField type.
impl InputField {
    pub fn new(data: BitVec, width: u32, height: u32) -> Self {
        check_dimensions_and_buffer_size(width, height, data.len());
        InputField {
            data,
            width,
            height,
        }
    }

    pub fn invert(&mut self) {
        self.data.iter_mut().for_each(|mut i| *i = !*i);
    }
}

/// Helper method, that checks if the length of a buffer is equal to its width times its height.
/// The method will panic, if this is not the case.
fn check_dimensions_and_buffer_size(width: u32, height: u32, buffer_len: usize) {
    if width == 0 {
        panic!("width must be greater than zero"); // maybe an error type "incorrect dimensions" would be better here!
    }
    if height == 0 {
        panic!("height must be greater than zero"); // maybe an error type "incorrect dimensions" would be better here!
    }
    let size = (width * height) as usize;
    if buffer_len != size {
        panic!("buffer size should be {}", width * height);
    }
}
