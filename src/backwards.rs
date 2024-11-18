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
	fn handle(&mut self, mars: &mut Mars, commands: &mut String) {
		if !commands.is_empty() && commands.chars().nth(0).unwrap() == self.identifier {
			let rover_posistion = mars.rover.get_position();
			let rover_direction = mars.rover.get_orientation();
			let new_position = match rover_direction {
				Direction::N => {
					if rover_posistion.get_y() == 0 {
						Point::new(rover_posistion.get_x(), mars.get_height())
					} else {
						Point::new(rover_posistion.get_x(), rover_posistion.get_y() - 1)
					}
				}
				Direction::E => {
					if rover_posistion.get_x() == 0 {
						Point::new(mars.get_width(), rover_posistion.get_y())
					} else {
						Point::new(rover_posistion.get_x() - 1, rover_posistion.get_y())
					}
				}
				Direction::S => {
					if rover_posistion.get_y() == mars.get_height() {
						Point::new(rover_posistion.get_x(), 0)
					} else {
						Point::new(rover_posistion.get_x(), rover_posistion.get_y() + 1)
					}
				}
				Direction::W => {
					if rover_posistion.get_x() == mars.get_width() {
						Point::new(0, rover_posistion.get_y())
					} else {
						Point::new(rover_posistion.get_x() + 1, rover_posistion.get_y())
					}
				}
			};
			mars.rover.set_position(new_position);
			commands.remove(0);
		}
	}

	fn next(&mut self) -> &mut Option<Box<dyn Handler>> {
		&mut self.next
	}
}
