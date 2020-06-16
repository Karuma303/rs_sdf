use crate::generator::{DistanceGenerator, GenerationStrategy};

mod input;
mod source;
mod generator;
mod naive;
mod output;

fn main() {
    let g = DistanceGenerator::new()
        .input(r"assets/SDF_Test_Texture_RGBA.png")
        .output(r"output/my_first_test.sdf")
        .strategy(GenerationStrategy::Naive); // maybe rename to process_strategy

    let result = g.generate();

    println!("generated: {}", result.is_ok());

    /*
        let d = Decoder::new(File::open(r"assets/SDF_Test_Texture_2.png").unwrap());
        let (info, mut reader) = d.read_info().unwrap();
        // Allocate the output buffer.
        let mut buffer = vec![0; info.buffer_size()];
        // Read the next frame. Currently this function should only called once.
        // The default options
        reader.next_frame(&mut buffer).unwrap();

     */
}

fn calculate_sdf() {
    // we should test and maybe microbenchmark at least two known apporches here:
    // 1) brute force O(n²)
    // 2) the old EightPointSeqEuclideanDistTrans O(n)
}

fn encode_image() {}

// Notes from old C# repository:

// edges detecten und markieren

// next: make it signed
// next: vector field

// vectoren zeichnen
// später: brute force circle methode auch mal ausprobieren : https://github.com/chriscummings100/signeddistancefields/blob/master/Assets/SignedDistanceFields/SignedDistanceFieldGenerator.cs


// https://github.com/chriscummings100/signeddistancefields
// https://shaderfun.com/
// https://shaderfun.com/2018/03/23/signed-distance-fields-part-1-unsigned-distance-fields/