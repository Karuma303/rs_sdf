use crate::distance_field::DistanceField;

/// Module for image based export functionality
pub mod image;

/// Exporter for a distance field
pub trait DistanceFieldExporter {
    fn export(&self, df: &DistanceField);
}
