use crate::{cellstate::CellState, point::Point};

#[derive(Clone, Copy)]
pub struct Cell {
	position: Point,
	state: CellState,
}

impl Cell {
	pub fn new(position: Point, state: CellState) -> Self {
		Self { position, state }
	}
}
