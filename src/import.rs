use crate::source::SourceField;

/// Module for image based import functionality
pub mod image;

/// Type for a valid input for the distance field generator.
/// Implementors of this trait provide a SourceField that can be further processed by the generator.
pub trait FieldInput {
    fn get_source_field(&self) -> Option<SourceField>;
}