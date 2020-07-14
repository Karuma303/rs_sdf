use std::cmp::{max, min};

use crate::df::{Cell, CellLayer, DistanceField};
use crate::input::FieldInput;
use crate::source::SourceField;

/// Generate a distance field for the source field.
pub fn generate_df(field: &SourceField) -> DistanceField {
    let mut buffer = init_buffer_for_distance_field(&field);
    sweep_buffer(&mut buffer, field.width, field.height);
    get_distance_field_from_buffer(&buffer, field.width, field.height)
}

/// Initialize an inner buffer with cells to calculate the distance field.
fn init_buffer_for_distance_field(source: &SourceField) -> Vec<Cell> {
    source.data
        .chunks(source.width as usize)
        .enumerate()
        .map(|(row_index, row_data)| {
            row_data.iter().enumerate().map(move |(col_index, value)| {
                Cell {
                    x: col_index as u16,
                    y: row_index as u16,
                    layer: match value {
                        true => CellLayer::Foreground,
                        false => CellLayer::Background,
                    },
                    nearest_cell_position: None,
                }
            })
        })
        .flatten()
        .collect()
}

/// 2-pass sweep over the inner buffer to calculate the distances.
fn sweep_buffer(buffer: &mut Vec<Cell>, field_width: u32, field_height: u32) {
    // Two pass sweep (down + up)
    sweep_buffer_down(buffer, field_width, field_height);
    sweep_buffer_up(buffer, field_width, field_height);
}

/// Down sweep (pass #1)
fn sweep_buffer_down(buffer: &mut Vec<Cell>, field_width: u32, field_height: u32) {
    let w = field_width as usize;
    let h = field_height as usize;

    let mut idx: usize = w + 1;

    // down

        // first row sweep left / sweep right

        // others:
        // special treatment for first
        // sweep row
        // special treatment for last
        // sweep to right skips the last cell but goes to the first

    // up

        // skip the last row but go until the first

    for _ in 1..(h - 1) {
        //
        // ***
        // *O.  -->
        // ...
        for _ in 1..(w - 1) {
            // let target_cell = &mut buffer[idx];
            compare_cells(buffer, idx, idx - 1); // left
            compare_cells(buffer, idx, idx - w); // top
            compare_cells(buffer, idx, idx - w - 1); // top left
            compare_cells(buffer, idx, idx - w + 1); // top right
            idx = idx + 1;
        }
        //      ...
        // <--  .O*
        //      ...
        for _ in (1..(w - 1)).rev() {
            idx = idx - 1;
            compare_cells(buffer, idx, idx + 1); // right
        }
        idx = idx + w;
    }
}

/// Up sweep (pass #2)
fn sweep_buffer_up(buffer: &mut Vec<Cell>, field_width: u32, field_height: u32) {
    let w = field_width as usize;
    let h = field_height as usize;

    let mut idx: usize = w * (h - 1) - 2;

    for _ in (1..(h - 1)).rev() {
        //      ...
        // <--  .O*
        //      ***
        for _ in (1..(w - 1)).rev() {
            // let mut target_cell = &mut buffer[idx];
            compare_cells(buffer, idx, idx + 1); // right
            compare_cells(buffer, idx, idx + w); // bottom
            compare_cells(buffer, idx, idx + w - 1); // bottom left
            compare_cells(buffer, idx, idx + w + 1); // bottom right
            idx = idx - 1;
        }
        // ...
        // *O.  -->
        // ...
        for _ in 0..w {
            idx = idx + 1;
            compare_cells(buffer, idx, idx - 1); // left
        }
        idx = idx - w;
    }
}

fn compare_cells(
    buffer: &mut Vec<Cell>,
    target_index: usize,
    source_index: usize) {
    let mut nearest_pos: Option<(u16, u16)> = None; // TODO: continue here...

    {
        let target_cell = &buffer[target_index];
        let source_cell = &buffer[source_index];


        if target_cell.layer != source_cell.layer {

            // the cells have a different layer, so we are on the boundary between
            // foreground and background. In that case, we will set the source
            // cell as the nearest cell in our target cell

            nearest_pos = Some((source_cell.x, source_cell.y));
            // target_cell.set_nearest_cell_position(source_cell.x, source_cell.y);

            // TODO: check, if we can and should use:
            // a) relative x/a offset here
            // b) relative index offset here
            // c) absolute index here
        } else {

            // the cells are on the same layer (foreground or background),
            // so we should maybe check their distances

            match target_cell.distance_to_nearest_squared() {
                None => {

                    // our target cell does not have a nearest cell yet,
                    // so we should take the nearest cell of the source
                    // as a reference.

                    // does the source have a nearest cell?

                    if let Some((x, y)) = source_cell.get_nearest_cell_position() {

                        // yes it has, so we set this as the targets nearest cell position
                        // target_cell.set_nearest_cell_position(x, y);
                        nearest_pos = Some((x, y));
                    }
                }

                Some(existing_target_distance) => {

                    // our target already has a distance

                    // let's check, if the source also has a nearest cell
                    if let Some((source_nearest_cell_x, source_nearest_cell_y)) = source_cell.get_nearest_cell_position() {

                        // yes it has, so let's calculate the distance from our target cell
                        // to the nearest cell of the source
                        let distance_to_sources_nearest_cell =
                            Cell::get_distance_squared(
                                &target_cell.x,
                                &target_cell.y,
                                &source_nearest_cell_x,
                                &source_nearest_cell_y); //

                        if distance_to_sources_nearest_cell < existing_target_distance {
                            // set the new target
                            // target_cell.set_nearest_cell_position(source_nearest_cell_x, source_nearest_cell_y)
                            nearest_pos = Some((source_nearest_cell_x, source_nearest_cell_y));
                        }
                    }
                    // otherwise we do nothing
                }
            }
        }
    }
    if let Some((x, y)) = nearest_pos {
        let target_cell = &mut buffer[target_index];
        target_cell.set_nearest_cell_position(x, y);
    }
}

// new
fn get_distance_field_from_buffer(buffer: &Vec<Cell>, width: u32, height: u32) -> DistanceField {
    buffer.iter().enumerate().for_each(|(index, cell)| {
        if cell.nearest_cell_position.is_none() {
            println!("None at index {}", index);
        }
    });

    DistanceField {
        width,
        height,
        data: buffer.clone(),
    }
}


// @deprecated
// creates an 8-bit resolution outer distance field
/*
pub fn generate_outer_df(field: &SourceField) -> DistanceField {
    let mut buffer = init_buffer_for_outer_distances(&field);
    sweep(&mut buffer, field.width, field.height);
    get_df_from_buffer(&buffer, field.width, field.height)
}
*/


// @deprecated
// creates an 8-bit resolution inner distance field
/*
pub fn generate_inner_df(field: &SourceField) -> DistanceField {
    let mut buffer = init_buffer_for_inner_distances(&field);
    sweep(&mut buffer, field.width, field.height);
    get_df_from_buffer(&buffer, field.width, field.height)
}
 */

// @deprecated
// creates an 8-bit resolution unsigned distance field (with inner and outer distances)
/*
pub fn generate_combined_df(field: &SourceField) -> DistanceField {
    let inner_df = generate_inner_df(&field);
    let outer_df = generate_outer_df(&field);

    combine_distance_fields(&inner_df, &outer_df)
}
 */

// @deprecated
/*
fn sweep(buffer: &mut Vec<u8>, field_width: u32, field_height: u32) {
    // Two pass sweep (down + up)
    sweep_down(buffer, field_width, field_height);
    sweep_up(buffer, field_width, field_height);
}
 */

// @deprecated
/*
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
 */

// @deprecated
/*
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
 */

// deprecated
/*
fn compare(buffer: &mut Vec<u8>, check_index: u32, other_index: u32) {
    // println!("{}/{}", check_index, other_index);
    let orig_distance = buffer[check_index as usize];
    let new_distance = buffer[other_index as usize].saturating_add(1);
    if new_distance < orig_distance {
        buffer[check_index as usize] = new_distance
    }
}
 */

// deprecated?
/*
fn combine_distance_fields(inner_df: &DistanceField, outer_df: &DistanceField) -> DistanceField {
    if inner_df.data.len() != outer_df.data.len() {
        panic!("inner and outer distance fields must have same size!");
    }
    let len = inner_df.data.len();
    let mut data: Vec<u8> = vec![0; len];

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
 */

// deprecated
/*
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
 */

// deprecated
// background cells populated with the maximum distance value
// foreground cells have zero distance values
/*
fn init_buffer_for_outer_distances(source: &SourceField) -> Vec<u8> {
    init_buffer(source, 0, u8::MAX)
}
 */

// deprecated
// background cells have zero distance values
// foreground cells populated with the maximum distance value
/*
fn init_buffer_for_inner_distances(source: &SourceField) -> Vec<u8> {
    init_buffer(source, u8::MAX, 0)
}
 */

// deprecated
/*
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
*/

#[cfg(test)]
mod tests {
    use crate::source::SourceField;

// use crate::naive::{init_buffer, init_buffer_for_outer_distances, init_buffer_for_inner_distances, get_df_from_buffer, generate_outer_df, generate_inner_df};

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

    /* TODO: reactivate this test !
    #[test]
    fn generates_buffer_with_additional_border() {
        let b_1x1_empty = init_buffer(&get_source_1_1_empty(), 0, 0);
        assert_eq!(b_1x1_empty.len(), 3 * 3);

        let b_1x1_filled = init_buffer(&get_source_1_1_filled(), 0, 0);
        assert_eq!(b_1x1_filled.len(), 3 * 3);

        let b_2x2 = init_buffer(&get_source_2_2_checker(), 0, 0);
        assert_eq!(b_2x2.len(), 4 * 4);
    }
     */

    /* TODO: reactivate this test
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
     */

    /* TODO: reactivate this test
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
     */

    /* TODO: reactivate this test
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
     */

    /* TODO: reactivate this test
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
     */

    /* TODO: reactivate this test
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
     */

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