use crate::data::Cell;
use std::cmp::min;
use crate::distance::DistanceValueType::{TupleU16, U16, U32, I32};

#[derive(Clone)]
pub enum DistanceLayer {
    Foreground,
    Background,
    Combined,
}

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

pub enum DistanceValueType {
    U16,
    U32,
    I16,
    I32,
    F32,
    F64,
    TupleU16,
}

impl DistanceType {
    pub fn calculation_function(&self) -> CalculationFunction<u16> {
        match self {
            DistanceType::EuclideanDistance => euclidean_distance_function,
            DistanceType::EuclideanDistanceSquared => euclidean_distance_squared_function,
            DistanceType::ChebyshevDistance => chebyshev_distance_function,
            DistanceType::RectilinearDistance => rectilinear_distance_function,
            DistanceType::CartesianDistance => cartesian_distance_function,
            DistanceType::NearestCellIndex => nearest_cell_index_function,
            DistanceType::NearestCellIndexOffset => nearest_cell_index_offset_function,
            DistanceType::NearestCellPosition => nearest_cell_position_function,
        }
        // function
    }

    pub fn value_type(&self) -> DistanceValueType {
        match self {
            DistanceType::EuclideanDistance => U16,
            DistanceType::EuclideanDistanceSquared => U32,
            DistanceType::ChebyshevDistance => U16,
            DistanceType::RectilinearDistance => U16,
            DistanceType::CartesianDistance => TupleU16,
            DistanceType::NearestCellIndex => U32,
            DistanceType::NearestCellIndexOffset => I32,
            DistanceType::NearestCellPosition => TupleU16,
        }
    }

    pub fn dimensions(&self) -> u8 {
        match self {
            DistanceType::EuclideanDistance => 1,
            DistanceType::EuclideanDistanceSquared => 1,
            DistanceType::ChebyshevDistance => 1,
            DistanceType::RectilinearDistance=> 1,
            DistanceType::CartesianDistance => 2,
            DistanceType::NearestCellIndex => 1,
            DistanceType::NearestCellIndexOffset => 1,
            DistanceType::NearestCellPosition => 1,
        }
    }
}

// pub type CalculationFunction8 = fn(&Cell) -> u8;
// pub type CalculationFunction = fn(&Cell) -> u16;
pub type CalculationFunction<T> = fn(&Cell) -> T;

// TODO: implement capping to u8 / u16 as functions with inline tag
// currently capping is in export/image

// TODO: write tests for this
fn chebyshev_distance_function(cell: &Cell) -> u16 {
    if let Some((nearest_x, nearest_y)) = cell.nearest_cell_position {
        let dx = (cell.x as i16 - nearest_x as i16).abs() as u16;
        let dy = (cell.y as i16 - nearest_y as i16).abs() as u16;
        return min(dx, dy) as u16;
    }
    0
}

fn euclidean_distance_function(cell: &Cell) -> u16 {
    if let Some(distance_squared) = cell.distance_to_nearest_squared() {
        let distance = (distance_squared as f32).sqrt();
        return if distance > 65535.0f32 {
            0xffff
        } else {
            distance.round() as u16
        };
    }
    // TODO: We should think about the best behaviour of the None case here. For now, we just return 0.
    0
}

fn euclidean_distance_squared_function(cell: &Cell) -> u16 {
    if let Some(distance_squared) = cell.distance_to_nearest_squared() {
        return if distance_squared > 65535u32 {
            0xffff
        } else {
            distance_squared as u16
        };
    }
    // TODO: We should think about the best behaviour of the None case here. For now, we just return 0.
    0
}

fn rectilinear_distance_function(cell: &Cell) -> u16 {
    todo!()
}

fn cartesian_distance_function(cell: &Cell) -> u16 { // TODO: should be (u16, u16)
    todo!()
}

fn nearest_cell_index_function(cell: &Cell) -> u16 { // TODO: should be u32
    todo!()
}

fn nearest_cell_index_offset_function(cell: &Cell) -> u16 { // TODO: should be i32
    todo!()
}

fn nearest_cell_position_function(cell : &Cell) -> u16 { // TODO: should be (u16, u16)
    todo!()
}


