use crate::generator::{DistanceGenerator, GenerationStrategy};

mod input;
mod source;
mod generator;
mod naive;
mod output;

fn main() {

    // generate some SDFs here based on input images
    generate_sdf(r"assets\SDF_Test_Texture_RGBA.png", r"output/my_first_test.png");
    generate_sdf(r"assets\SDF_Test_Texture_2.png", r"output/my_second_test.png");
    generate_sdf(r"assets\SDF_Test_RGBA.png", r"output/my_third_test.png");
    generate_sdf(r"assets\rgba_512x512_heart.png", r"output/heart.png");
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
