use crate::data::Cell;
use crate::distance::OneDimensionalDistanceCalculation;
use crate::utils::{f32_to_u8_clamped, f32_to_u16_clamped};

/// The euclidean distance to the nearest cell.
/// The distance is a single, unsigned value.
pub struct EuclideanDistance;

impl EuclideanDistance {

	// This is the default calculation for this distance type with maximum precision
	pub fn calculate(cell: &Cell) -> f64 {
		if let Some(distance_squared) = cell.distance_to_nearest_squared() {
			(distance_squared as f64).sqrt()
		} else {
			0f64
		}
	}
}

impl OneDimensionalDistanceCalculation<u8> for EuclideanDistance {
	fn calculate_legacy(cell: &Cell) -> u8 {
		if let Some(distance_squared) = cell.distance_to_nearest_squared() {
			let distance = (distance_squared as f32).sqrt();
			f32_to_u8_clamped(distance)
		} else {
			0
		}
	}
}

impl OneDimensionalDistanceCalculation<u16> for EuclideanDistance {
	fn calculate_legacy(cell: &Cell) -> u16 {
		if let Some(distance_squared) = cell.distance_to_nearest_squared() {
			let distance = (distance_squared as f32).sqrt();
			f32_to_u16_clamped(distance)
		} else {
			0
		}
	}
}

impl OneDimensionalDistanceCalculation<u32> for EuclideanDistance {
	fn calculate_legacy(cell: &Cell) -> u32 {
		if let Some(distance_squared) = cell.distance_to_nearest_squared() {
			let distance = (distance_squared as f32).sqrt();
			distance as u32
		} else {
			0
		}
	}
}

/// The squared euclidean distance to the nearest cell.
/// The distance is a single, unsigned value.
pub struct EuclideanDistanceSquared;

impl EuclideanDistanceSquared {
	// This is the default calculation for this distance type with maximum precision
	pub fn calculate_u64(cell: &Cell) -> u64 {
		if let Some(distance_squared) = cell.distance_to_nearest_squared() {
			distance_squared
		} else {
			0
		}
	}
}

impl OneDimensionalDistanceCalculation<u8> for EuclideanDistanceSquared {
	fn calculate_legacy(_cell: &Cell) -> u8 {
		unimplemented!()
	}
}

impl OneDimensionalDistanceCalculation<u16> for EuclideanDistanceSquared {
	fn calculate_legacy(cell: &Cell) -> u16 {
		if let Some(distance_squared) = cell.distance_to_nearest_squared() {
			if distance_squared > 65535u64 {
				0xffff
			} else {
				distance_squared as u16
			}
		} else {
			// TODO: We should think about the best behaviour of the None case here. For now, we just return 0.
			0
		}
	}
}

impl OneDimensionalDistanceCalculation<u32> for EuclideanDistanceSquared {
	fn calculate_legacy(_cell: &Cell) -> u32 {
		unimplemented!()
	}
}
