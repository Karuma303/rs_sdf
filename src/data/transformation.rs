use crate::distance::{DistanceLayer, DistanceType};
use crate::distance::DistanceType::EuclideanDistance;
use crate::data::DistanceField;

pub struct DistanceTransformation {
    // TODO: we need some reference here to the distance field

    filter: DistanceLayer,
    scale: Option<f32>,
    distance_type: DistanceType,
}

impl From<DistanceField> for DistanceTransformation {
    fn from(df: DistanceField) -> Self {

        // TODO: we must give the df here to the transformation
        DistanceTransformation::new()
    }
}

impl DistanceTransformation {
    fn new() -> Self {
        Self {
            filter: DistanceLayer::Combined,
            scale: None,
            distance_type: EuclideanDistance,
        }
    }

    pub fn filter(&mut self, layer: DistanceLayer) {
        self.filter = layer;
    }

    pub fn scale(&mut self, scale: f32) {
        self.scale = Some(scale);
    }

    pub fn distance_type(&mut self, distance_type: DistanceType) {
        self.distance_type = distance_type;
    }

    pub fn transform(&self) -> TransformationResult {
        unimplemented!();
        TransformationResult {num_channels : 2} // TODO implement
    }
}


pub struct TransformationResult {
    num_channels : u8, // TODO: this should be removed
    // we should have a vector with different output lanes here later
}