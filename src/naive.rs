use crate::source::SourceField;
use std::cmp::{max, min};

// creates an 8-bit resolution outer distance field
pub fn generate_outer_df(field: &SourceField) -> DistanceField<u8> {
    let mut buffer = init_buffer_for_outer_distances(&field);
    sweep(&mut buffer, field.width, field.height);
    get_df_from_buffer(&buffer, field.width, field.height)
}

// creates an 8-bit resolution inner distance field
pub fn generate_inner_df(field: &SourceField) -> DistanceField<u8> {
    let mut buffer = init_buffer_for_inner_distances(&field);
    sweep(&mut buffer, field.width, field.height);
    get_df_from_buffer(&buffer, field.width, field.height)
}

// creates an 8-bit resolution unsigned distance field (with inner and outer distances)
pub fn generate_combined_df(field: &SourceField) -> DistanceField<u8> {
    let inner_df = generate_inner_df(&field);
    let outer_df = generate_outer_df(&field);

    combine_distance_fields(&inner_df, &outer_df)
}

fn sweep(buffer: &mut Vec<u8>, field_width: u32, field_height: u32) {
    // Two pass sweep (down + up)
    sweep_down(buffer, field_width, field_height);
    sweep_up(buffer, field_width, field_height);
}

fn sweep_down(buffer: &mut Vec<u8>, field_width: u32, field_height: u32) {
    let buffer_width = field_width + 2;

    // outer loop (going down)
    let mut idx = buffer_width + 1; // start at pos (1/1)
    for _ in 0..field_height {
        //
        // ***
        // *O.  -->
        // ...
        for _ in 0..field_width {
            compare(buffer, idx, idx - 1); // left
            compare(buffer, idx, idx - buffer_width); // top
            compare(buffer, idx, idx - buffer_width - 1); // top left
            compare(buffer, idx, idx - buffer_width + 1); // top right
            idx = idx + 1;
        }
        //      ...
        // <--  .O*
        //      ...
        for _ in (0..field_width).rev() {
            idx = idx - 1;
            compare(buffer, idx, idx + 1); // right
        }
        idx = idx + buffer_width;
    }
}

fn sweep_up(buffer: &mut Vec<u8>, field_width: u32, field_height: u32) {
    let buffer_width = field_width + 2;

    // outer loop (going up)
    let mut idx = field_height * buffer_width + field_width;
    for _ in (0..field_height).rev() {
        //      ...
        // <--  .O*
        //      ***
        for _ in (0..field_width).rev() {
            compare(buffer, idx, idx + 1); // right
            compare(buffer, idx, idx + buffer_width); // bottom
            compare(buffer, idx, idx + buffer_width - 1); // bottom left
            compare(buffer, idx, idx + buffer_width + 1); // bottom right
            idx = idx - 1;
        }
        // ...
        // *O.  -->
        // ...
        for _ in 0..field_width {
            idx = idx + 1;
            compare(buffer, idx, idx - 1); // left
        }
        idx = idx - buffer_width;
    }
}

fn compare(buffer: &mut Vec<u8>, check_index: u32, other_index: u32) {
    // println!("{}/{}", check_index, other_index);
    let orig_distance = buffer[check_index as usize];
    let new_distance = buffer[other_index as usize].saturating_add(1);
    if new_distance < orig_distance {
        buffer[check_index as usize] = new_distance
    }
}

fn combine_distance_fields(inner_df: &DistanceField<u8>, outer_df: &DistanceField<u8>) -> DistanceField<u8> {
    if inner_df.data.len() != outer_df.data.len() {
        panic!("inner and outer distance fields must have same size!");
    }
    let len = inner_df.data.len();
    let mut data:Vec<u8> = vec![0; len];

    for index in 0..len {
        let inner = inner_df.data[index];
        let outer = outer_df.data[index];
        assert!(inner == 0 || outer == 0, "not null");
        data[index] = inner + outer;
        //data[index] = match (outer) {
        //    0 => (max(-(inner as i16 * 2), -128i16))as i8,
        //    _ => (min((outer as i16), 127i16)) as i8,
        //};
    }

    DistanceField {
        data,
        width: inner_df.width,
        height: inner_df.height,
    }
}

fn get_df_from_buffer(buffer: &Vec<u8>, width: u32, height: u32) -> DistanceField<u8> {
    let source_w = (width + 2) as usize;
    let source_h = (height + 2) as usize;

    if buffer.len() != source_w * source_h {
        panic!("incorrect buffer size");
    }

    let target_w = width as usize;
    let target_h = height as usize;

    let mut distance_vec = vec![0; target_w * target_h];

    for y in 0..target_h {
        for x in 0..target_w {
            distance_vec[y * target_w + x] = buffer[(y + 1) * source_w + x + 1];
        }
    }

    DistanceField {
        data: distance_vec,
        width,
        height,
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

// new structure - we should use that !
pub struct NearestEdge<T> {
    pub x: T,
    pub y: T,
    pub distance_squared: T,
}

impl NearestEdge<i32> {
    pub fn new(x: i32, y: i32) -> Self {
        NearestEdge { x, y, distance_squared: x * y }
    }
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

#[cfg(test)]
mod tests {
    use crate::source::SourceField;
    use crate::naive::{init_buffer, init_buffer_for_outer_distances, init_buffer_for_inner_distances, get_df_from_buffer, generate_outer_df, generate_inner_df};

    // helper method to get an empty source field
    fn get_source_0_0() -> SourceField {
        SourceField::new(&[], 0, 0)
    }

    // helper method to get an empty 1x1 source field
    fn get_source_1_1_empty() -> SourceField {
        SourceField::new(&[
            0,
        ], 1, 1)
    }

    // helper method to get an filled 1x1 source field
    fn get_source_1_1_filled() -> SourceField {
        SourceField::new(&[
            255,
        ], 1, 1)
    }

    // helper method to get an 2x2 source field width a checkered pattern
    fn get_source_2_2_checker() -> SourceField {
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
        let b_1x1_empty = init_buffer(&get_source_1_1_empty(), 0, 0);
        assert_eq!(b_1x1_empty.len(), 3 * 3);

        let b_1x1_filled = init_buffer(&get_source_1_1_filled(), 0, 0);
        assert_eq!(b_1x1_filled.len(), 3 * 3);

        let b_2x2 = init_buffer(&get_source_2_2_checker(), 0, 0);
        assert_eq!(b_2x2.len(), 4 * 4);
    }

    #[test]
    fn get_filled_buffer_for_outer_distance() {
        let b = init_buffer_for_outer_distances(&get_source_2_2_checker());
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
        let b = init_buffer_for_inner_distances(&get_source_2_2_checker());
        let m = u8::MAX;
        assert_eq!(b, [
            0, 0, 0, 0,
            0, m, 0, 0,
            0, 0, m, 0,
            0, 0, 0, 0,
        ]);
    }

    #[test]
    fn gets_correct_distance_field_size_from_oversize_buffer() {
        let b_filled = init_buffer_for_outer_distances(&get_source_1_1_filled());
        let df_filled = get_df_from_buffer(&b_filled, 1, 1);
        assert_eq!(df_filled.data.len(), 1);
        assert_eq!(df_filled.data[0], 0);

        let b_empty = init_buffer_for_outer_distances(&get_source_1_1_empty());
        let df_filled = get_df_from_buffer(&b_empty, 1, 1);
        assert_eq!(df_filled.data.len(), 1);
        assert_eq!(df_filled.data[0], u8::MAX);
    }

    #[test]
    fn generates_outer_distance_field() {
        let df_checker = generate_outer_df(&get_source_2_2_checker());
        assert_eq!(df_checker.data, vec![0, 1, 1, 0]);

        let df_empty = generate_outer_df(&get_source_1_1_empty());
        assert_eq!(df_empty.data, vec![u8::MAX]);

        let df_empty_big = generate_outer_df(&get_source_3_3_empty());
        assert_eq!(df_empty_big.data, vec![u8::MAX, u8::MAX, u8::MAX,
                                           u8::MAX, u8::MAX, u8::MAX,
                                           u8::MAX, u8::MAX, u8::MAX]);

        let df_filled = generate_outer_df(&get_source_1_1_filled());
        assert_eq!(df_filled.data, vec![0]);

        let df_filled_big = generate_outer_df(&get_source_3_3_filled());
        assert_eq!(df_filled_big.data, vec![0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn generates_inner_distance_field() {
        let df_checker = generate_inner_df(&get_source_2_2_checker());
        assert_eq!(df_checker.data, vec![1, 0, 0, 1]);

        let df_empty = generate_inner_df(&get_source_1_1_empty());
        assert_eq!(df_empty.data, vec![0]);

        let df_empty_big = generate_inner_df(&get_source_3_3_empty());
        assert_eq!(df_empty_big.data, vec![0, 0, 0, 0, 0, 0, 0, 0, 0]);

        let df_filled = generate_inner_df(&get_source_1_1_filled());
        assert_eq!(df_filled.data, vec![1]);

        let df_filled_big = generate_inner_df(&get_source_3_3_filled());
        assert_eq!(df_filled_big.data, vec![1, 1, 1, 1, 2, 1, 1, 1, 1]);
    }

// TODO: generate signed distance field
// TODO: check for max ranges and clamping
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