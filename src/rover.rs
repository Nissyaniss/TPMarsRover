use crate::direction::Direction;
use crate::point::Point;

#[derive(Clone, Copy)]
pub struct Rover {
	position: Point,
	orientation: Direction,
}

impl Rover {
	pub fn new(position: Point, orientation: Direction) -> Self {
		Self {
			position,
			orientation,
		}
	}

	pub fn get_position(&self) -> Point {
		self.position
	}

	pub fn get_orientation(&self) -> Direction {
		self.orientation
	}

	pub fn set_position(&mut self, position: Point) {
		self.position = position;
	}

	pub fn set_orientation(&mut self, orientation: Direction) {
		self.orientation = orientation;
	}
}
