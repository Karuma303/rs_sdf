use crate::data::Cell;
use crate::distance::OneDimensionalDistanceCalculation;

/// The euclidean distance to the nearest cell.
/// The distance is a single, unsigned value.
pub struct EuclideanDistance;

impl OneDimensionalDistanceCalculation<u8> for EuclideanDistance {
	fn calculate(&self, cell: &Cell) -> u8 {
		if let Some(distance_squared) = cell.distance_to_nearest_squared() {
			let distance = (distance_squared as f32).sqrt();
			if distance > 255.0f32 {
				0xff
			} else {
				distance.round() as u8
			}
		} else {
			0
		}
	}
}

impl OneDimensionalDistanceCalculation<u16> for EuclideanDistance {
	fn calculate(&self, cell: &Cell) -> u16 {
		if let Some(distance_squared) = cell.distance_to_nearest_squared() {
			let distance = (distance_squared as f32).sqrt();
			if distance > 65535.0f32 {
				0xffff
			} else {
				distance.round() as u16
			}
		} else {
			0
		}
	}
}

impl OneDimensionalDistanceCalculation<u32> for EuclideanDistance {
	fn calculate(&self, _cell: &Cell) -> u32 {
		unimplemented!()
	}
}

/// The squared euclidean distance to the nearest cell.
/// The distance is a single, unsigned value.
pub struct EuclideanDistanceSquared;

impl OneDimensionalDistanceCalculation<u8> for EuclideanDistanceSquared {
	fn calculate(&self, _cell: &Cell) -> u8 {
		unimplemented!()
	}
}

impl OneDimensionalDistanceCalculation<u16> for EuclideanDistanceSquared {
	fn calculate(&self, cell: &Cell) -> u16 {
		if let Some(distance_squared) = cell.distance_to_nearest_squared() {
			if distance_squared > 65535u32 {
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
	fn calculate(&self, _cell: &Cell) -> u32 {
		unimplemented!()
	}
}
