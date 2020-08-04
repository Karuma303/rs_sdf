use crate::data::{Cell, CellLayer, DistanceField};
use crate::processor::SourceProcessor;
use crate::data::input::InputField;

pub struct EightSideSweepProcessor;

impl SourceProcessor for EightSideSweepProcessor {
    fn process(&self, field: &InputField) -> DistanceField {
        let mut buffer = self.init_buffer_for_distance_field(&field);
        self.sweep_buffer(&mut buffer, field.width, field.height);
        self.get_distance_field_from_buffer(&buffer, field.width, field.height)
    }
}

impl EightSideSweepProcessor {
    /// Initialize an inner buffer with cells to calculate the distance field.
    fn init_buffer_for_distance_field(&self, source: &InputField) -> Vec<Cell> {
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
    fn sweep_buffer(&self, buffer: &mut Vec<Cell>, field_width: u32, field_height: u32) {
        // Two pass sweep (down + up)
        self.sweep_buffer_down(buffer, field_width, field_height);
        self.sweep_buffer_up(buffer, field_width, field_height);
    }

    /// Down sweep (pass #1)
    fn sweep_buffer_down(&self, buffer: &mut Vec<Cell>, field_width: u32, field_height: u32) {
        let w = field_width as usize;
        let h = field_height as usize;

        // first row

        // sweep to the right (left)
        //
        //      *O- -->
        //
        for index in 1..w {
            self.compare_cells(buffer, index, index - 1); // left
        };

        // sweep to the left (right)
        //
        // <--  -O*
        //
        for index in (0..w - 1).rev() {
            self.compare_cells(buffer, index, index + 1); // right
        };

        let mut idx = w;

        // other rows
        for _ in 1..h {

            // sweep to the right

            // first cell (top, top-right)
            self.compare_cells(buffer, idx, idx - w); // top
            self.compare_cells(buffer, idx, idx - w + 1); // top right

            idx += 1;

            // row cells (except first and last)
            for _ in 1..w - 1 {
                self.compare_cells(buffer, idx, idx - 1); // left
                self.compare_cells(buffer, idx, idx - w); // top
                self.compare_cells(buffer, idx, idx - w - 1); // top left
                self.compare_cells(buffer, idx, idx - w + 1); // top right
                idx += 1;
            }

            // last cell (left, top-left, top)
            self.compare_cells(buffer, idx, idx - 1); // left
            self.compare_cells(buffer, idx, idx - w); // top
            self.compare_cells(buffer, idx, idx - w - 1); // top left

            // sweep to the left
            for _ in 0..w - 1 {
                idx -= 1;
                self.compare_cells(buffer, idx, idx + 1); // right
            };
            idx += w;
        }
    }

    /// Up sweep (pass #2)
    fn sweep_buffer_up(&self, buffer: &mut Vec<Cell>, field_width: u32, field_height: u32) {
        let w = field_width as usize;
        let h = field_height as usize;

        let mut idx = w * h - 1;

        // last row
        // sweep to the left (right)
        for _ in 1..w {
            idx -= 1;
            self.compare_cells(buffer, idx, idx + 1); // right
        }
        // sweep to the right (left)
        for _ in 1..w {
            self.compare_cells(buffer, idx, idx - 1); // left
            idx += 1;
        }

        // other rows
        for _ in 1..h {
            idx -= w;

            // sweep to the left
            // first element (bottom, bottom-left)
            self.compare_cells(buffer, idx, idx + w); // bottom
            self.compare_cells(buffer, idx, idx + w - 1); // bottom left

            for _ in 1..w - 1 {
                // other elements (except first and last)
                idx -= 1;

                //      ...
                // <--  .O*
                //      ***
                self.compare_cells(buffer, idx, idx + 1); // right
                self.compare_cells(buffer, idx, idx + w); // bottom
                self.compare_cells(buffer, idx, idx + w + 1); // bottom right
                self.compare_cells(buffer, idx, idx + w - 1); // bottom left
            }

            // last element (right, bottom-right, bottom)
            idx -= 1;
            self.compare_cells(buffer, idx, idx + 1); // right
            self.compare_cells(buffer, idx, idx + w); // bottom
            self.compare_cells(buffer, idx, idx + w + 1); // bottom right

            // sweep to the right (left)
            // ...
            // *O.  -->
            // ...
            for _ in 1..w {
                idx += 1;
                self.compare_cells(buffer, idx, idx - 1);
            }
        }
    }

    fn compare_cells(
        &self,
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
    fn get_distance_field_from_buffer(&self, buffer: &[Cell], width: u32, height: u32) -> DistanceField {
        /*
        print!("None: ");
        buffer.iter().enumerate().for_each(|(index, cell)| {
            if cell.nearest_cell_position.is_none() {
                print!(" {} ", index);
            }
        });
        println!();
         */

        DistanceField {
            width,
            height,
            data: Vec::from(buffer),
        }
    }
}
