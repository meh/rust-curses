use libc::c_int;
use curses;

use {Error, Result, Character};
use super::Screen;

pub struct Add<'a> {
	screen: &'a mut Screen,
}

impl<'a> Add<'a> {
	#[inline]
	pub unsafe fn wrap(screen: &mut Screen) -> Add {
		Add { screen: screen }
	}
}

impl<'a> Add<'a> {
	#[inline]
	pub fn string<S: AsRef<str>>(self, string: S) -> Result<&'a mut Screen> {
		unsafe {
			let string = string.as_ref();
			let bytes  = string.as_bytes();

			try!(Error::check(curses::addnstr(bytes.as_ptr() as *const _, bytes.len() as c_int)));
		}

		Ok(self.screen)
	}

	pub fn character<C: Into<Character>>(self, character: C) -> Result<&'a mut Screen> {
		unsafe {
			let character: Character = character.into();

			try!(Error::check(curses::add_wch(character.as_ptr())));
		}

		Ok(self.screen)
	}
}
