use crate::source::SourceField;

#[derive(Debug, Clone)]
/// A single cell of a distance field
pub struct Cell {
    /// The layer (foreground, background) this cell belongs to.
    pub layer: CellLayer,

    /// The horizontal position of the cell in the field.
    pub x: u16,

    /// The vertical position of the cell in the field.
    pub y: u16,

    /// The position of the nearest cell from the other layer.
    pub nearest_cell_position: Option<(u16, u16)>,
}

#[derive(Debug, Clone, PartialEq)]
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
//            let horiz_dist = i32::from(self.x) - i32::from(nearest_x.clone());
//            let vert_dist = i32::from(self.y) - i32::from(nearest_y.clone());
            // Some(horiz_dist.pow(2) as u32 + vert_dist.pow(2) as u32)
            Some(Self::get_distance_squared(&self.x, &self.y, &nearest_x, &nearest_y))
        } else {
            None
        }
    }

    pub fn get_nearest_cell_position(&self) -> Option<(u16, u16)> {
        self.nearest_cell_position
    }

    /// Set the position (x,y) of the nearest cell with the opposite layer type.
    pub fn set_nearest_cell_position(&mut self, x: u16, y: u16) {
        self.nearest_cell_position = Some((x, y));
    }

    pub fn get_distance_squared(first_x: &u16, first_y: &u16, second_x: &u16, second_y: &u16) -> u32 {
        // TODO: we should check all the casts here
        // TODO: maybe use appropriate rust functions here
        let horiz_dist = i32::from(first_x.clone()) - i32::from(second_x.clone());
        let vert_dist = i32::from(first_y.clone()) - i32::from(second_y.clone());
        horiz_dist.pow(2) as u32 + vert_dist.pow(2) as u32
    }
}

/// A two-dimensional distance field with cells.
pub struct DistanceField {
    pub data: Vec<Cell>,
    pub width: u32,
    pub height: u32,
}

impl DistanceField {
    // TODO: it is rather stupid to make a filtered distance field. The filter should be moved to the output stage.
    pub fn filter_inner(source: &Self) -> Self {
        let cells = source.data.iter().map(|cell| {
            match cell.layer {
                CellLayer::Foreground => cell.clone(),
                CellLayer::Background => Cell {
                    x: cell.x,
                    y: cell.y,
                    layer: CellLayer::Background,
                    nearest_cell_position: Some((cell.x, cell.y)),
                }
            }
        }).collect();
        DistanceField {
            width: source.width,
            height: source.height,
            data: cells,
        }
    }

    // TODO: it is rather stupid to make a filtered distance field. The filter should be moved to the output stage.
    pub fn filter_outer(source: &Self) -> Self {
        let cells = source.data.iter().map(|cell| {
            match cell.layer {
                CellLayer::Background => cell.clone(),
                CellLayer::Foreground => Cell {
                    x: cell.x,
                    y: cell.y,
                    layer: CellLayer::Foreground,
                    nearest_cell_position: Some((cell.x, cell.y)),
                }
            }
        }).collect();
        DistanceField {
            width: source.width,
            height: source.height,
            data: cells,
        }
    }

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
                            move |(x, &value)| {
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