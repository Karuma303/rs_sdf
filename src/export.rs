use crate::data::{DistanceField, Cell};
use crate::distance::{DistanceType, CalculationFunction8, CalculationFunction16};

/// Module for image based export functionality
pub mod image;

/// Exporter for a distance field
pub trait DistanceFieldExporter {
    fn export(
        &self,
        distance_field: &DistanceField,
        export_type: &DistanceType,
        export_filter: &ExportFilter,
    );
}

#[derive(Clone)]
pub enum ExportFilter {
    Foreground,
    Background,
    All,
}

