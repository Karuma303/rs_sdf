use bitvec::vec::{BitVec};

/// Spatial input type as a source for distance field generation.
/// An input field is defined by its dimensions and a binary data block.
/// The number of items inside the data block match the dimension of the input
/// and a set data value specifies a foreground cell, whereas an unset data
/// value  specifies a background cell.
#[derive(Debug)]
pub struct InputField {
    pub data: BitVec,
    pub width: u32,
    pub height: u32,
}


pub struct InputByteBuffer {
    data: Vec<u8>,
    width: u32,
    height: u32,
    threshold: u8,
}

impl InputByteBuffer {
    pub fn new(buffer: &[u8], threshold: u8, width: u32, height: u32) -> Self {
        check_dimensions_and_buffer_size(width, height, buffer.len());
        Self {
            data: Vec::from(buffer), // TODO: box here ?
            threshold,
            width,
            height,
        }
    }
}

pub struct InputBoolBuffer {
    data: Vec<bool>,
    width: u32,
    height: u32,
}

impl InputBoolBuffer {
    pub fn new(buffer: &[bool], width: u32, height: u32) -> Self {
        check_dimensions_and_buffer_size(width, height, buffer.len());
        Self {
            data: Vec::from(buffer), // TODO: box here ?
            width,
            height,
        }
    }
}

impl From<InputByteBuffer> for InputField {
    fn from(buffer: InputByteBuffer) -> Self {
        let mut data = BitVec::new();
        buffer.data.iter().for_each(|elem| data.push(*elem > buffer.threshold));
        InputField::new(data, buffer.width, buffer.height)
    }
}

impl From<InputBoolBuffer> for InputField {
    fn from(buffer: InputBoolBuffer) -> Self {
        let mut data = BitVec::new();
        buffer.data.iter().for_each(|elem| data.push(*elem));
        InputField::new(data, buffer.width, buffer.height)
    }
}

/// The implementation of an InputField provides several constructors
/// for building a field from arrays of primitive types.
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
