/// Module for image-based export (PNG)
pub mod image;

#[derive(PartialEq, Clone)]
pub enum BitDepth {
	Eight,
	Sixten,
	ThirtyTwo,
}