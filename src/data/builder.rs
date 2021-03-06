use crate::data::DistanceField;
use crate::input::DistanceInput;
use crate::processor::{Processor};
use crate::input::image::PngInput;

pub struct DistanceFieldBuilder {
	input: Box<dyn DistanceInput>,
}

impl DistanceFieldBuilder {
	pub fn new(input: Box<dyn DistanceInput>) -> Self {
		DistanceFieldBuilder {
			input
		}
	}

	pub fn build(self, processor: Processor) -> DistanceField {
		let input_field = self.input.source_field().unwrap(); // dangerous !
		processor.processor.process(&input_field)
	}
}

// TODO. this must be DistanceInput I guess and not PngInput
impl From<PngInput> for DistanceFieldBuilder {
	fn from(input: PngInput) -> Self {
		Self::new(Box::new(input))
	}
}
