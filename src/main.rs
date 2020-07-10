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

    // another test
    let g = DistanceGenerator::new()
        .input(r"assets\SDF_Test_Texture_2.png")
        .output(r"output/my_second_test.png")
        .strategy(GenerationStrategy::Naive); // maybe rename to process_strategy
    g.generate();

    // another test
    let g = DistanceGenerator::new()
        .input(r"assets\SDF_Test_RGBA.png")
        .output(r"output/my_third_test.png")
        .strategy(GenerationStrategy::Naive); // maybe rename to process_strategy
    g.generate();

    // another test
    let g = DistanceGenerator::new()
        .input(r"assets\rgba_512x512_heart.png")
        .output(r"output/heart.png")
        .strategy(GenerationStrategy::Naive); // maybe rename to process_strategy
    g.generate();

}
