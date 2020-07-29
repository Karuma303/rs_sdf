use crate::data::{DistanceField};
use crate::distance::{DistanceType};

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

