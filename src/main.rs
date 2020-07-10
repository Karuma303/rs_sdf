use crate::generator::{DistanceGenerator, GenerationStrategy};

mod input;
mod source;
mod generator;
mod naive;
mod output;

fn main() {

    // generate some SDFs here based on input images
    generate_sdf(r"assets\example_1_rgba_512x512.png", r"output/example_1_512x512.png");
    generate_sdf(r"assets\example_2_rgba_512x512.png", r"output/example_2_512x512.png");
    generate_sdf(r"assets\example_3_rgba_512x512.png", r"output/example_3_512x512.png");
    generate_sdf(r"assets\example_4_rgba_512x512.png", r"output/example_4_512x512.png");
}

fn generate_sdf(source_image_path: &str, destination_image_path: &str) {
    let g = DistanceGenerator::new()
        .input(&source_image_path)
        .output(&destination_image_path)
        .strategy(GenerationStrategy::Naive); // maybe rename to process_strategy

    let result = g.generate();

    match result {
        Ok(_) => {
            println!("generated sdf {} for input image {}", destination_image_path, source_image_path);
        }
        Err(err) => {
            println!("error generating sdf: {}", err);
        }
    }
}
