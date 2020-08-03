#[derive(Debug)]
pub struct SourceField {
    pub data: Vec<bool>,
    pub width: u32,
    pub height: u32,
}

impl SourceField {

    // TODO
    // The SourceField should be kinda independent from the input buffer.
    // Maybe we should add another abstraction layer here. Sth. like a generator trait
    // for SourceFields and some implementations for different input types!

    pub fn from_bytes(buffer: &[u8], threshold: u8, width: u32, height: u32) -> Self {
        Self::error_if_invalid_params(width, height, buffer.len());
        let mut data: Vec<bool> = vec!();
        buffer.iter().for_each(|elem| data.push(*elem > threshold));
        SourceField {
            data,
            width,
            height,
        }
    }

    pub fn from_booleans(buffer: &[bool], width: u32, height: u32) -> Self {
        Self::error_if_invalid_params(width, height, buffer.len());
        let mut data: Vec<bool> = vec!();
        buffer.iter().for_each(|elem| data.push(*elem));
        SourceField {
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

    /*
    pub fn new(buffer: &[u8], width: u32, height: u32) -> Self {
        if width == 0 {
            panic!("width must be greater than zero"); // maybe an error type "incorrect dimensions" would be better here!
        }
        if height == 0 {
            panic!("height must be greater than zero"); // maybe an error type "incorrect dimensions" would be better here!
        }

        let size = (width * height) as usize;
        if buffer.len() != size {
            panic!("buffer size should be {}", width * height);
        }
        let mut data: Vec<bool> = vec!();
        buffer.iter().for_each(|elem| data.push(*elem > 127));
        SourceField {
            data,
            width,
            height,
        }
    }

     */

    pub fn invert(&mut self) {
        self.data.iter_mut().for_each(|i| *i = !*i);
    }
}
