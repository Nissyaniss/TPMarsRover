use crate::mars::Mars;

pub trait Handler {
	fn execute(&mut self, mars: &mut Mars, commands: &mut String) {
		self.handle(mars, commands);

		if let Some(next) = &mut self.next() {
			next.execute(mars, commands);
		}
	}

	fn handle(&mut self, mars: &mut Mars, commands: &mut String);
	fn next(&mut self) -> &mut Option<Box<dyn Handler>>;
}

pub fn into_next(handler: impl Handler + Sized + 'static) -> Option<Box<dyn Handler>> {
	Some(Box::new(handler))
}
