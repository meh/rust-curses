use curses;

use {Error, Event, Key};
use super::Window;

pub struct Input<'a> {
	window: &'a mut Window<'a>,
}

impl<'a> Input<'a> {
	#[inline]
	pub unsafe fn wrap<'b>(window: &'b mut Window<'b>) -> Input<'b> {
		Input { window: window }
	}
}

impl<'a> Input<'a> {
	#[inline]
	pub fn event(&mut self) -> Option<Event> {
		unsafe {
			let mut character = 0;
			let     value     = some!(Error::check(curses::wget_wch(self.window.as_mut_ptr(),
				&mut character)));

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
