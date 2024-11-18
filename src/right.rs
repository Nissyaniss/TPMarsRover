use crate::direction::Direction;
use crate::handler::{into_next, Handler};
use crate::mars::Mars;

#[derive(Default)]
pub struct Right {
	identifier: char,
	next: Option<Box<dyn Handler>>,
}

impl Right {
	pub fn new(next: impl Handler + 'static) -> Self {
		Self {
			identifier: 'r',
			next: into_next(next),
		}
	}
}

impl Handler for Right {
	fn handle(&mut self, mars: &mut Mars, mut commands: &str) {
		if commands.chars().nth(0).unwrap() == self.identifier {
			let rover_direction = mars.get_rover().get_orientation();
			let new_direction;
			match rover_direction {
				Direction::N => {
					new_direction = Direction::E;
				}
				Direction::E => {
					new_direction = Direction::S;
				}
				Direction::S => {
					new_direction = Direction::W;
				}
				Direction::W => {
					new_direction = Direction::N;
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
