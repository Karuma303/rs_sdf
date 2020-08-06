use crate::data::DistanceField;
use crate::data::input::InputField;
use crate::processor::sweep::EightSideSweepProcessor;

pub mod sweep;

/// A SourceProcessor takes a SourceField and turns it into a DistanceField
/// (based on some internal algorithm to calculate the distances).
pub trait SourceProcessor {
    /// Generate a distance field for the source field.
    fn process(&self, field: &InputField) -> DistanceField;
}

pub struct Processor {
    pub processor : Box<dyn SourceProcessor>,
}

impl From<EightSideSweepProcessor> for Processor {
    fn from(proc: EightSideSweepProcessor) -> Self {
        Self {
            processor: Box::new(proc),
        }
    }
}
