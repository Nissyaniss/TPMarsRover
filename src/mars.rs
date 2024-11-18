use crate::{cell::Cell, cellstate::CellState, point::Point, rover::Rover};

pub struct Mars {
	grid: Vec<Vec<Cell>>,
	height: u32,
	width: u32,
	rover: Rover,
}

impl Mars {
	pub fn new(width: u32, height: u32, rover: Rover) -> Mars {
		Self {
			width,
			height,
			grid: vec![
				vec![Cell::new(Point::new(width, height), CellState::Empty); width as usize];
				height as usize
			],
			rover,
		}
	}

	pub fn get_rover(&self) -> Rover {
		self.rover
	}

	pub fn get_height(&self) -> u32 {
		self.height
	}

	pub fn get_width(&self) -> u32 {
		self.width
	}
}
