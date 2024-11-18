use crate::direction::Direction;
use crate::handler::{into_next, Handler};
use crate::mars::Mars;
use crate::point::Point;

pub struct Backwards {
	identifier: char,
	next: Option<Box<dyn Handler>>,
}

impl Backwards {
	pub fn new(next: impl Handler + 'static) -> Self {
		Self {
			identifier: 'b',
			next: into_next(next),
		}
	}
}

impl Handler for Backwards {
	fn handle(&mut self, mars: &mut Mars, mut commands: &str) {
		if commands.chars().nth(0).unwrap() == self.identifier {
			let rover_posistion = mars.get_rover().get_position();
			let rover_direction = mars.get_rover().get_orientation();
			let new_position;
			match rover_direction {
				Direction::N => {
					if rover_posistion.get_y() == 0 {
						new_position = Point::new(rover_posistion.get_x(), mars.get_height());
					} else {
						new_position =
							Point::new(rover_posistion.get_x(), rover_posistion.get_y() - 1);
					}
				}
				Direction::E => {
					if rover_posistion.get_x() == 0 {
						new_position = Point::new(mars.get_width(), rover_posistion.get_y());
					} else {
						new_position =
							Point::new(rover_posistion.get_x() - 1, rover_posistion.get_y());
					}
				}
				Direction::S => {
					if rover_posistion.get_y() == mars.get_height() {
						new_position = Point::new(rover_posistion.get_x(), 0);
					} else {
						new_position =
							Point::new(rover_posistion.get_x(), rover_posistion.get_y() + 1);
					}
				}
				Direction::W => {
					if rover_posistion.get_x() == mars.get_width() {
						new_position = Point::new(0, rover_posistion.get_y());
					} else {
						new_position =
							Point::new(rover_posistion.get_x() + 1, rover_posistion.get_y());
					}
				}
			}
			mars.get_rover().set_position(new_position);
		}
		commands = &commands[1..];
	}

	fn next(&mut self) -> &mut Option<Box<dyn Handler>> {
		&mut self.next
	}
}
