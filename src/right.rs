use crate::direction::Direction;
use crate::handler::{into_next, Handler};
use crate::mars::Mars;

#[derive(Default)]
pub struct Right {
	next: Option<Box<dyn Handler>>,
}

impl Handler for Right {
	fn handle(&mut self, mars: &mut Mars, commands: &mut String) {
		if !commands.is_empty() && commands.chars().nth(0).unwrap() == 'r' {
			let rover_direction = mars.rover.get_orientation();
			let new_direction = match rover_direction {
				Direction::N => Direction::E,
				Direction::E => Direction::S,
				Direction::S => Direction::W,
				Direction::W => Direction::N,
			};
			mars.rover.set_orientation(new_direction);
			commands.remove(0);
		}
	}

	fn next(&mut self) -> &mut Option<Box<dyn Handler>> {
		&mut self.next
	}
}
