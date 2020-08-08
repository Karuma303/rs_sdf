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

    // TODO: type argument must be set according to distance calculation method
    pub fn transform(&self) -> TransformationResult::<u32> {
        // unimplemented!();
        TransformationResult::<u32> {
            width: 100,
            height: 100,
            channels: vec![vec![0u32]],
        } // TODO implement
    }

    /* taken from former export mod:

            // inner / outer / combined ?
            // combined: add or sdf?
            // 8 bit / 16 bit
            match &self.configuration.channel_depth {
            ImageOutputChannelDepth::Eight => {
                let function = distance_type.calculation_function();
                df.data.iter().for_each(|cell: &Cell| {
                    // TODO: right now, we just add the inner distances and the outer distances
                    // We should add a feature to generate real 8-bit-signed distance field here!
                    // buffer.push(self.get_8_bit_distance(&cell));
                    buffer.push(u16_to_u8_clamped(function(&cell)));
                });
            }
            ImageOutputChannelDepth::Sixteen => {
                let function = distance_type.calculation_function();
                df.data.iter().for_each(|cell: &Cell| {
                    let distance = function(&cell);
                    buffer.push((distance >> 8) as u8);
                    buffer.push((distance & 0xFF) as u8);
                });
            }
            _ => {
                // TODO: we have to implement the 32 bit output (use for example for squared distance output)
                unimplemented!()
            }

     */
}


pub struct TransformationResult<T> {
    pub width: u32,
    pub height: u32,
    pub channels: Vec<Vec<T>>,
    // we should have a vector with different output lanes here later
}