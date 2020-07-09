use crate::generator::{DistanceGenerator, GenerationStrategy};

mod input;
mod source;
mod generator;
mod naive;
mod output;

fn main() {
    let g = DistanceGenerator::new()
        .input(r"assets\SDF_Test_Texture_RGBA.png")
        .output(r"output/my_first_test.png")
        .strategy(GenerationStrategy::Naive); // maybe rename to process_strategy

    let result = g.generate();

    println!("generated: {}", result.is_ok());
}
