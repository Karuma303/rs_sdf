use crate::source::SourceField;
use std::ops::{SubAssign, Sub};

#[derive(Debug, Clone)]
/// A single cell of a distance field
pub struct Cell {
    /// The layer (foreground, background) this cell belongs to.
    pub layer: CellLayer,

    /// The horizontal position of the cell in the field.
    x: u16,

    /// The vertical position of the cell in the field.
    y: u16,

    /// The position of the nearest cell from the other layer.
    nearest_cell_position: Option<(u16, u16)>,
}

#[derive(Debug, Clone)]
/// The layer definition for the cells.
pub enum CellLayer {
    /// The foreground layer. This is the layer where cells are regarded as being set.
    /// In an input image, this is for example the layer where the pixels are opaque.
    Foreground,

    /// The background layer. This is the layer where cells are regarded as not being set.
    /// In an input image, this is for example the layer where the pixels are fully transparent.
    Background,
}

impl Cell {
    pub fn new(layer: CellLayer, x: u16, y: u16) -> Self {
        Self {
            x,
            y,
            layer,
            nearest_cell_position: None,
        }
    }

    /// The absolute squared distance to the nearest cell with the opposite layer type.
    /// This is `None`, if no nearest cell was detected (yet).
    pub fn distance_to_nearest_squared(&self) -> Option<u32> {
        if let Some((nearest_x, nearest_y)) = &self.nearest_cell_position {
            // TODO: we should check all the casts here
            // TODO: use appropriate rust functions here
            let horiz_dist = i32::from(self.x) - i32::from(nearest_x.clone());
            let vert_dist = i32::from(self.y) - i32::from(nearest_y.clone());
            Some(horiz_dist.pow(2) as u32 + vert_dist.pow(2) as u32)
        } else {
            None
        }
    }

    /// Set the position (x,y) of the nearest cell with the opposite layer type.
    fn set_nearest_cell_position(&mut self, x: u16, y: u16) {
        self.nearest_cell_position = Some((x, y));
    }
}

/// A two-dimensional distance field with cells.
pub struct DistanceField {
    pub data: Vec<Cell>,
    pub width: u32,
    pub height: u32,
}

impl DistanceField {
    /// Initialize a DistanceField based on the given SourceField.
    fn new(source: &SourceField) -> Self {
        let width = source.width;
        let height = source.height;

        let cells = source.data
            .as_slice()
            .chunks(width as usize)
            .into_iter()
            .enumerate()
            .map(
            |(y, row_values)| {
                let row_vector = row_values
                    .iter()
                    .enumerate()
                    .map(
                        move|(x, &value)| {
                            Cell {
                                x: x.clone() as u16,
                                y: y.clone() as u16,
                                nearest_cell_position: None,
                                layer: if value { CellLayer::Foreground } else { CellLayer::Background },
                            }
                        });
                row_vector
            })
            .flatten()
            .collect();

        DistanceField {
            data: cells,
            width,
            height,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::source::SourceField;

    #[test]
    fn distance_field_initializes_correctly() {
        let sf: SourceField = SourceField {
            width: 2,
            height: 2,
            data: Vec::from([true, false, false, true]),
        };
        assert_eq!(sf.width, 2);
        assert_eq!(sf.height, 2);
        assert_eq!(sf.data[0], true);
        assert_eq!(sf.data[1], false);
        assert_eq!(sf.data[2], false);
        assert_eq!(sf.data[3], true);
        // TODO: add more checks here!
    }
}