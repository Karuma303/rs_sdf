use crate::data::input::InputField;

pub mod input;
pub mod builder;
pub mod transformation;
pub mod output;
pub mod serialize;

#[derive(Debug, Clone)]
/// A single cell of a distance field
pub struct Cell {
	/// The layer (foreground, background) this cell belongs to.
	pub layer: CellLayer,

	/// The index of the cell in the field when it is seen as a one dimensional list.
	pub index: u32,

	/// The horizontal position of the cell in the field.
	pub x: u16,

	/// The vertical position of the cell in the field.
	pub y: u16,

	/// The position of the nearest cell from the other layer.
	pub nearest_cell_position: Option<CellPosition>,
}

#[derive(Debug, Clone, Copy)]
pub struct CellPosition {
	pub x: u16,
	pub y: u16,
	pub index: u32,
}

impl CellPosition {
	pub fn get_coordinates(&self) -> (u16, u16) {
		(self.x, self.y)
	}
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
	pub fn new(layer: CellLayer, index: u32, x: u16, y: u16) -> Self {
		Self {
			index,
			x,
			y,
			layer,
			nearest_cell_position: None,
		}
	}

	/// The absolute squared distance to the nearest cell with the opposite layer type.
	/// This is `None`, if no nearest cell was detected (yet).
	pub fn distance_to_nearest_squared(&self) -> Option<u64> {
		if let Some(nearest) = &self.nearest_cell_position {
			Some(Self::get_distance_squared(&self.x, &self.y, &nearest.x, &nearest.y))
		} else {
			None
		}
	}

	pub fn get_nearest_cell_position(&self) -> &Option<CellPosition> {
		&self.nearest_cell_position
	}

	/// Set the position (x,y, index) of the nearest cell with the opposite layer type.
	pub fn set_nearest_cell_position(&mut self, pos: CellPosition) {
		self.nearest_cell_position = Some(pos);
	}

	pub fn get_distance_squared(first_x: &u16, first_y: &u16, second_x: &u16, second_y: &u16) -> u64 {
		// TODO: we should check all the casts here
		// TODO: maybe use appropriate rust functions here
		let horiz_dist = i32::from(*first_x) - i32::from(*second_x);
		let vert_dist = i32::from(*first_y) - i32::from(*second_y);
		horiz_dist.pow(2) as u64 + vert_dist.pow(2) as u64
	}
}

/// A two-dimensional distance field with cells.
/// The maximum for the width and the height of the field is 2^16 (65.536) units,
/// so the maximum size of the whole field is 2^32 (4.294.967.296) units.
pub struct DistanceField {
	pub data: Vec<Cell>,
	pub width: u16,
	pub height: u16,
}

impl DistanceField {
	// TODO: it is rather stupid to make a filtered distance field. The filter should be moved to the export stage.
	pub fn filter_inner(source: &Self) -> Self {
		let cells = source.data.iter().map(|cell| {
			match cell.layer {
				CellLayer::Foreground => cell.clone(),
				CellLayer::Background => Cell {
					index: cell.index,
					x: cell.x,
					y: cell.y,
					layer: CellLayer::Background,
					nearest_cell_position: None,
				}
			}
		}).collect();
		DistanceField {
			width: source.width,
			height: source.height,
			data: cells,
		}
	}

	// TODO: it is rather stupid to make a filtered distance field. The filter should be moved to the export stage.
	pub fn filter_outer(source: &Self) -> Self {
		let cells = source.data.iter().map(|cell| {
			match cell.layer {
				CellLayer::Background => cell.clone(),
				CellLayer::Foreground => Cell {
					index: cell.index,
					x: cell.x,
					y: cell.y,
					layer: CellLayer::Foreground,
					nearest_cell_position: None,
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
	pub fn new(source: &InputField) -> Self {
		let cells = source.data
			.chunks(source.width as usize)
			.enumerate()
			.map(
				|(y, row_values)| {
					row_values
						.iter()
						.enumerate()
						.map(
							move |(x, &value)| {
								Cell {
									index: (x + y * source.height as usize) as u32,
									x: x as u16,
									y: y as u16,
									nearest_cell_position: None,
									layer: if value { CellLayer::Foreground } else { CellLayer::Background },
								}
							})
				})
			.flatten()
			.collect();

		DistanceField {
			data: cells,
			width: source.width,
			height: source.height,
		}
	}
}
