use crate::{mars::Mars, rover::Rover};

pub trait Handler {
	fn execute(&mut self, rover: &mut Mars, commands: &str) {
		self.handle(rover, commands);

		if let Some(next) = &mut self.next() {
			next.execute(rover, commands);
		}
	}

	fn handle(&mut self, rover: &mut Mars, commands: &str);
	fn next(&mut self) -> &mut Option<Box<dyn Handler>>;
}

pub fn into_next(handler: impl Handler + Sized + 'static) -> Option<Box<dyn Handler>> {
	Some(Box::new(handler))
}
