use crate::source::SourceField;

/// A single cell of a distance field
pub struct Cell {
    /// The layer (foreground, background) this cell belongs to.
    layer: CellLayer,

    /// The horizontal position of the cell in the field.
    x: u16,

    /// The vertical position of the cell in the field.
    y: u16,

    /// The position of the nearest cell from the other layer.
    nearest_cell_position: Option<(u16, u16)>,
}

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
    fn distance_to_nearest(&self) -> Option<u32> {
        if let Some((nearest_x, nearest_y)) = &self.nearest_cell_position {
            let horiz_dist_squared = (&self.x - nearest_x).pow(2);
            let vert_dist_squared = (&self.y - nearest_y).pow(2);
            Some((horiz_dist_squared + vert_dist_squared).sqtr())
        } else {
            None
        }
    }

    fn set_nearest_cell_position(&mut self, x: u16, y: u16) {
        self.nearest_cell = Some((x, y));
    }
}

/// A two-dimensional distance field with cells.
pub struct DistanceField {
    pub data: Vec<Cell>,
    pub width: u32,
    pub height: u32,
}

impl DistanceField {
    fn new(source: &SourceField) -> Self {
        let width = source.width;
        let height = source.height;

        let cells: Vec<Cell> = Vec::with_capacity((width * height) as usize);

        // TODO: init the cell array here !

        DistanceField {
            data: cells,
            width,
            height,
        }
    }
}

// RC

#[cfg(test)]
mod tests {
    #[test]
    fn foo() {
        assert_eq!(true, true);
    }
}