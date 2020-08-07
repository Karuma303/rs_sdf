use crate::data::transformation::TransformationResult;

/// Output writer for a distance transformations.
/// Implementors are responsible to write the given result to some output (e.g. to a file).
pub trait OutputWriter {
    /// Write the transformation result to the output.
    fn write(&self, output: TransformationResult);
}