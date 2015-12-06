use curses;

use {Error, Event, Key};
use super::Screen;

#[allow(dead_code)]
pub struct Input<'a> {
	screen: &'a mut Screen,
}

impl<'a> Input<'a> {
	#[inline]
	pub unsafe fn wrap(screen: &mut Screen) -> Input {
		Input { screen: screen }
	}
}

impl<'a> Input<'a> {
	#[inline]
	pub fn event(&mut self) -> Option<Event> {
		unsafe {
			let mut character = 0;
			let     value     = some!(Error::check(curses::get_wch(&mut character)));

			Event::fetch(value, character)
		}
	}

	#[inline]
	pub fn character(&mut self) -> Option<char> {
		match self.event() {
			Some(Event::Key(Key::Char(ch))) =>
				Some(ch),

			Some(..) =>
				self.character(),

			None =>
				None
		}
	}
}
