use crate::source::SourceField;

// creates an 8-bit resolution distance field (only with outer values)
pub fn generate_df(field: &SourceField) -> DistanceField<u8> {
    let size = (field.width * field.height) as usize;

    DistanceField {
        data: vec![0; size],
        width: field.width,
        height: field.height,
    }
}

// creates an 8-bit resolution outer distance field
pub fn generate_outer_df(field: &SourceField) -> DistanceField<u8> {
    let mut buffer = init_buffer_for_outer_distances(&field);
    sweep(&mut buffer);
    get_df_from_buffer(&buffer, field.width, field.height)
}

// creates an 8-bit resolution inner distance field
pub fn generate_inner_df(field: &SourceField) -> DistanceField<u8> {
    let mut buffer = init_buffer_for_inner_distances(&field);
    sweep(&mut buffer);
    get_df_from_buffer(&buffer, field.width, field.height)
}

// creates an 8-bit resolution signed distance field (with inner and outer distances)
pub fn generate_signed_df(field: &SourceField) -> DistanceField<i8> {
    let inner_df = generate_inner_df(&field);
    let outer_df = generate_outer_df(&field);

    // TODO: invert inner field
    // TODO: add the two fields together
    // TODO: clamp results (clamp, clamp_balanced)
}

fn sweep(buffer: &mut Vec<u8>) {

    // TODO: implement !

    // phase 1: down

    // left to right

    // right to left

    // phase 2: up

    // left to right

    // right to left
}

fn get_df_from_buffer(buffer: &Vec<u8>, width: u32, height: u32) -> DistanceField<u8> {
    if buffer.len() != ((width + 2) * (height +2)) as usize {
       panic!("incorrect buffer size");
    }

    let distance_vec = vec![0; (width * height) as usize];

    // TODO: copy values from buffer vector to distance vector

    [0..height].iter().for_each(|y| {
        [0..width].iter().for_each(|x| {

        });
    });

    DistanceField {
        data: distance_vec,
        width: field.width,
        height: field.height,
    }
}

// background cells populated with the maximum distance value
// foreground cells have zero distance values
fn init_buffer_for_outer_distances(source: &SourceField) -> Vec<u8> {
    init_buffer(source, 0, u8::MAX)
}

// background cells have zero distance values
// foreground cells populated with the maximum distance value
fn init_buffer_for_inner_distances(source: &SourceField) -> Vec<u8> {
    init_buffer(source, u8::MAX, 0)
}

fn init_buffer(source: &SourceField, set_value: u8, unset_value: u8) -> Vec<u8> {
    let mut buf = vec![unset_value; ((source.width + 2) * (source.height + 2)) as usize];

    let d = &source.data;
    let w = source.width;
    let h = source.height;

    for y in 0..h {
        for x in 0..w {
            if d[(x + (y * h)) as usize] == true {
                buf[(x + 1 + (y + 1) * (w + 2)) as usize] = set_value
            };
        };
    };
    buf
}

// creates an 8-bit resolution signed distance field (with inner and outer values)
pub fn generate_sdf(source: &SourceField) -> DistanceField<i8> {

    // source field has a vector of boolean in his data property
    // true at a give positions means that the field is occupied, false means that the field is empty

    let size = (source.width * source.height) as usize;

    let mut outer_buffer = init_buffer_for_outer_distances(source);
    let mut inner_buffer = init_buffer_for_inner_distances(source);

    calculate_distances(&mut outer_buffer);
    calculate_distances(&mut inner_buffer);

    // TODO: add the two fields together

    // TODO: return the real sdf here and not this dummy DistanceField
    DistanceField {
        width: source.width,
        height: source.height,
        data: vec![0; size],
    }

    // step 1 - detect outer distances


    // step 2 - detect inner distances

    // swipe top/left

    // swipe right
}

pub fn calculate_distances(buffer: &mut Vec<u8>) {
    // TODO: implement

    // swipe top/left

    // swipe right
}

pub fn get_combined_buffer(outer_buffer: &Vec<u8>, inner_buffer: &Vec<u8>) -> Vec<u8> {
    // TODO: implement
    vec![0, 0]
}

pub struct DistanceField<T> {
    pub data: Vec<T>,
    pub width: u32,
    pub height: u32,
    // TODO: add more metadata here maybe...
    // largest (outer/inner) distance
    // isSigned (true/false)

//    pub fn get_min_value() -> T;
//    pub fn get_max_value() -> T;
}

impl DistanceField<u8> {
    fn new(source: &SourceField) -> Self {
        let width = source.width;
        let height = source.height;
        DistanceField {
            data: vec![0_u8; (width * height) as usize],
            width,
            height,
        }
    }

    fn init_for_outer_distance() {}

    fn init_for_inner_distance() {}
}

trait InitDistanceField {}

impl InitDistanceField for DistanceField<i8> {
//    fn init_for_outer_distance(source : &SourceField) {
//        // TODO:
//    }
}

impl DistanceField<f32> {
//    fn init(source: &SourceField) {
    // TODO:
//    }
}

#[cfg(test)]
mod tests {
    use crate::source::SourceField;
    use crate::naive::{init_buffer, init_buffer_for_outer_distances, init_buffer_for_inner_distances, generate_sdf};

    // helper method to get an empty source field
    fn get_source_0_0() -> SourceField {
        SourceField::new(&[], 0, 0)
    }

    // helper method to get an 1x1 source field width a checkered pattern
    fn get_source_1_1_checker() -> SourceField {
        SourceField::new(&[255, 0, 0, 255], 2, 2)
    }

    // helper method to get an empty 3x3 source field
    fn get_source_3_3_empty() -> SourceField {
        SourceField::new(&[
            0, 0, 0,
            0, 0, 0,
            0, 0, 0,
        ], 3, 3)
    }

    // helper method to get a filled 3x3 source field
    fn get_source_3_3_filled() -> SourceField {
        SourceField::new(&[
            255, 255, 255,
            255, 255, 255,
            255, 255, 255,
        ], 3, 3)
    }

    #[test]
    fn generates_buffer_with_additional_border() {
        let b_empty = init_buffer(&get_source_0_0(), 0, 0);
        assert_eq!(b_empty.len(), 2 * 2);

        let b_2x2 = init_buffer(&get_source_1_1_checker(), 0, 0);
        assert_eq!(b_2x2.len(), 4 * 4);
    }

    #[test]
    fn get_filled_buffer_for_outer_distance() {
        let b = init_buffer_for_outer_distances(&get_source_1_1_checker());
        let m = u8::MAX;
        assert_eq!(b, [
            m, m, m, m,
            m, 0, m, m,
            m, m, 0, m,
            m, m, m, m,
        ]);
    }

    #[test]
    fn get_filled_buffer_for_inner_distance() {
        let b = init_buffer_for_inner_distances(&get_source_1_1_checker());
        let m = u8::MAX;
        assert_eq!(b, [
            0, 0, 0, 0,
            0, m, 0, 0,
            0, 0, m, 0,
            0, 0, 0, 0,
        ]);
    }


    #[test]
    fn generates_distance_field_u8_3x3() {
//        let b = vec![0, 0, 0, 0, 1, 0, 0, 0, 0];
        let s = SourceField::new(&b, 3, 3);
        let df = generate_sdf(get_source_3_3_empty());
        assert!(df.data == vec![2, 1, 2, 1, 0, 1, 2, 1, 2]);
    }

    /*
    #[test]
    fn generates_signed_distance_field_i8_3x3() {
        let b = vec![0, 0, 0, 0, 1, 0, 0, 0, 0];
        let s = SourceField::new(&b,3,3);
        let df = generate_sdf(&s);
        assert!(df.data == vec![2, 1, 2, 1, -1, 1, 2, 1, 2]);
    }
     */
}