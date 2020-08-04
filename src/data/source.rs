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

/// The implementation of an InputField provides several constructors
/// for building a field from arrays of primitive types.
impl InputField {

    // TODO
    // The SourceField should be kinda independent from the input buffer.
    // Maybe we should add another abstraction layer here. Sth. like a generator trait
    // for SourceFields and some implementations for different input types!

    pub fn from_bytes(buffer: &[u8], threshold: u8, width: u32, height: u32) -> Self {
        Self::error_if_invalid_params(width, height, buffer.len());
        // let mut data: Vec<bool> = vec!();
        let mut data = BitVec::new();
        // buffer.iter().for_each(|elem| data.push(*elem > threshold));
        buffer.iter().for_each(|elem| data.push(*elem > threshold));
        InputField {
            // data,
            data,
            width,
            height,
        }
    }

    pub fn from_booleans(buffer: &[bool], width: u32, height: u32) -> Self {
        Self::error_if_invalid_params(width, height, buffer.len());
        let mut data = BitVec::new();
        buffer.iter().for_each(|elem| data.push(*elem));
        InputField {
            data,
            width,
            height,
        }
    }

    fn error_if_invalid_params(width: u32, height: u32, buffer_len: usize) {
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

    pub fn invert(&mut self) {
        self.data.iter_mut().for_each(|mut i| *i = !*i);
    }
}
