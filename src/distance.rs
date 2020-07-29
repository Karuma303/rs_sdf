use crate::data::Cell;
use std::cmp::min;

/// Specification of all the different distance types that the library is able to calculate.
pub enum DistanceType {
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

impl DistanceType {
    pub(crate) fn calculator_8_bit(&self) -> CalculationFunction8 {
        let function: CalculationFunction8 = match self {
            DistanceType::EuclideanDistance => get_8_bit_euclidean_distance,
            DistanceType::ChebyshevDistance => get_8_bit_chebyshev_distance,
            // TODO: continue here...
            _ => get_8_bit_euclidean_distance,
        };
        function
    }

    pub(crate) fn calculator_16_bit(&self) -> CalculationFunction16 {
        match self {
            DistanceType::EuclideanDistance => get_16_bit_euclidean_distance,
            DistanceType::ChebyshevDistance => get_16_bit_chebyshev_distance,
            // TODO: continue here...
            _ => get_16_bit_euclidean_distance,
        }
    }
}

pub type CalculationFunction8 = fn(&Cell) -> u8;
pub type CalculationFunction16 = fn(&Cell) -> u16;

// TODO: implement capping to u8 / u16 as functions with inline tag


// TODO: write tests for this
fn get_8_bit_chebyshev_distance(cell: &Cell) -> u8 {
    if let Some((nearest_x, nearest_y)) = cell.nearest_cell_position {
        let dx = (cell.x as i16 - nearest_x as i16).abs() as u16;
        let dy = (cell.y as i16 - nearest_y as i16).abs() as u16;
        let min_distance = min(dx, dy);
        return if min(dx, dy) > 255u16 {
            255u8
        } else {
            min_distance as u8
        };
    }
    0
}

// TODO: write tests for this
fn get_16_bit_chebyshev_distance(cell: &Cell) -> u16 {
    if let Some((nearest_x, nearest_y)) = cell.nearest_cell_position {
        let dx = (cell.x as i16 - nearest_x as i16).abs() as u16;
        let dy = (cell.y as i16 - nearest_y as i16).abs() as u16;
        return min(dx, dy) as u16;
    }
    0
}

fn get_8_bit_euclidean_distance(cell: &Cell) -> u8 {
    if let Some(distance_squared) = cell.distance_to_nearest_squared() {
        let square_root = (distance_squared as f32).sqrt();
        return if square_root > 255f32 {
            255u8
        } else {
            square_root as u8 //  ^ 0xffu8 to invert
        };
    }
    // TODO: We should think about the best behaviour of the None case here. For now, we just return 0.
    0
}

fn get_16_bit_euclidean_distance(cell: &Cell) -> u16 {
    if let Some(distance_squared) = cell.distance_to_nearest_squared() {
        let distance = (distance_squared as f32).sqrt();// * 16f32;

        if distance > 65535.0f32 {
            0xffff
        } else {
            distance.round() as u16
        };
    }
    // TODO: We should think about the best behaviour of the None case here. For now, we just return 0.
    0
}

