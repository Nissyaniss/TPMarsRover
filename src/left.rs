use crate::direction::Direction;
use crate::handler::{into_next, Handler};
use crate::mars::Mars;
pub struct Left {
	identifier: char,
	next: Option<Box<dyn Handler>>,
}

impl Left {
	pub fn new(next: impl Handler + 'static) -> Self {
		Self {
			identifier: 'l',
			next: into_next(next),
		}
	}
}

impl Handler for Left {
	fn handle(&mut self, mars: &mut Mars, commands: &mut String) {
		if !commands.is_empty() && commands.chars().nth(0).unwrap() == self.identifier {
			let rover_direction = mars.rover.get_orientation();
			let new_direction = match rover_direction {
				Direction::N => Direction::W,
				Direction::E => Direction::N,
				Direction::S => Direction::E,
				Direction::W => Direction::S,
			};
			mars.rover.set_orientation(new_direction);
			commands.remove(0);
		}
	}

	fn next(&mut self) -> &mut Option<Box<dyn Handler>> {
		&mut self.next
	}
}
