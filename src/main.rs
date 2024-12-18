mod backwards;
mod cell;
mod cellstate;
mod direction;
mod forward;
mod handler;
mod left;
mod mars;
mod point;
mod right;
mod rover;

use backwards::Backwards;
use direction::Direction;
use forward::Forward;
use handler::Handler;
use left::Left;
use mars::Mars;
use point::Point;
use right::Right;

use crate::rover::Rover;

fn main() {
	let rover_direction = Direction::N;
	let rover_position = Point::new(0, 0);
	let rover = Rover::new(rover_position, rover_direction);
	let mut commands = String::from("ffrff");

	let mut mars = Mars::new(100, 100, rover);
	let right = Right::default();
	let backward = Backwards::new(right);
	let left = Left::new(backward);
	let mut forward = Forward::new(left);

	println!(
		"x = {}, y = {}, commands = {} ",
		mars.rover.get_position().get_x(),
		mars.rover.get_position().get_y(),
		commands
	);
	while !commands.is_empty() {
		forward.execute(&mut mars, &mut commands);
		println!(
			"x = {}, y = {}, commands = {} ",
			mars.rover.get_position().get_x(),
			mars.rover.get_position().get_y(),
			commands
		);
	}
}
