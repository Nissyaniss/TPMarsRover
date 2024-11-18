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
	fn handle(&mut self, mars: &mut Mars, mut commands: &str) {
		if commands.chars().nth(0).unwrap() == self.identifier {
			let rover_direction = mars.get_rover().get_orientation();
			let new_direction;
			match rover_direction {
				Direction::N => {
					new_direction = Direction::W;
				}
				Direction::E => {
					new_direction = Direction::N;
				}
				Direction::S => {
					new_direction = Direction::E;
				}
				Direction::W => {
					new_direction = Direction::S;
				}
			}
			mars.get_rover().set_orientation(new_direction);
		}
		commands = &commands[1..];
	}

	fn next(&mut self) -> &mut Option<Box<dyn Handler>> {
		&mut self.next
	}
}
