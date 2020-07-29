use crate::source::SourceField;
use crate::data::DistanceField;

pub mod sweep;

/// A SourceProcessor takes a SourceField and turns it into a DistanceField
/// (based on some internal algorithm to calculate the distances).
pub trait SourceProcessor {
    /// Generate a distance field for the source field.
    fn process(&self, field: &SourceField) -> DistanceField;
}