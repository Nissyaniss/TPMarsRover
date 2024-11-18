#[derive(Clone, Copy)]
pub struct Point {
	x: u32,
	y: u32,
}

impl Point {
	pub fn new(x: u32, y: u32) -> Point {
		Self { x, y }
	}

	pub fn get_x(&self) -> u32 {
		self.x
	}

	pub fn get_y(&self) -> u32 {
		self.y
	}
}
