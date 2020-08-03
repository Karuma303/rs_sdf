//! Input-related types and functionality.
//! An input type provides some source data that is either
//! imported from a file, or dynamically generated.
//! After import/generation of the data the input type
//! will provide a SourceField, which can then be used for further processing.

use crate::data::source::SourceField;
use std::fmt;

/// Image-based input types and functionality.
pub mod image;

/// Type for a valid input for distance field generation.
/// Implementors of this trait provide a SourceField that can be transformed to a distance field.
pub trait Input {
    fn source_field(&self) -> Result<SourceField, InputError>;
}

/// Error type for all kinds of errors that can happen on generating the input.
#[derive(Debug, PartialEq)]
pub enum InputError {
    InvalidInput { message: String },
}


impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error: {:?}", self)
    }
}
