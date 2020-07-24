use crate::distance_field::DistanceField;

/// Module for image based export functionality
pub mod image;

/// Exporter for a distance field
pub trait DistanceFieldExporter {
    fn export(
        &self,
        distance_field: &DistanceField,
        export_type: &ExportType,
        export_filter: &ExportFilter,
    );
}

/// Specification of the exported value.
pub enum ExportType {
    /// The euclidean distance to the nearest cell.
    /// The distance is a single, unsigned value.
    EuclideanDistance,

    /// The squared euclidean distance to the nearest cell.
    /// The distance is a single, unsigned value.
    EuclideanDistanceSquared,

    /// The Chebyshev distance (also known as the chessboard distance) to the nearest cell.
    /// It is defined as the minimum of the horizontal distance and the vertical distance.
    /// The distance is a single unsigned value.
    ChebyshevDistance,

    /// The Rectilinear distance (also know as the Manhattan distance) to the nearest cell.
    /// It is defined as the sum of the horizontal distance and the vertical distance.
    /// The distance is a single, unsigned value.
    RectilinearDistance,

    /// The cartesian distance on x- and y-axis to the nearest cell.
    /// The difference is a tuple of signed values.
    CartesianDistance,

    /// The absolute index of the nearest cell in the 1-dimensional array of cells.
    /// This is a single, unsigned value.
    NearestCellIndex,

    /// The relative index of the nearest cell in the 1-dimensional array of cells.
    /// It is measured as an offset from the current cell.
    /// This is a single, signed value.
    NearestCellIndexOffset,

    /// The absolute cartesian position (on x- and y-axis) of the nearest cell.
    /// This is a tuple of unsigned values.
    NearestCellPosition,
}

#[derive(Clone)]
pub enum ExportFilter {
    Foreground,
    Background,
    All,
}

