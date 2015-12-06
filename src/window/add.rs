use libc::c_int;
use curses;

use {Error, Result, Character};
use super::Window;

pub struct Add<'a> {
	window: &'a mut Window<'a>,
}

impl<'a> Add<'a> {
	#[inline]
	pub unsafe fn wrap<'b>(window: &'b mut Window<'b>) -> Add<'b> {
		Add { window: window }
	}
}

impl<'a> Add<'a> {
	#[inline]
	pub fn string<S: AsRef<str>>(self, string: S) -> Result<&'a mut Window<'a>> {
		unsafe {
			let string = string.as_ref();
			let bytes  = string.as_bytes();

			try!(Error::check(curses::waddnstr(self.window.as_mut_ptr(),
				bytes.as_ptr() as *const _, bytes.len() as c_int)));
		}

		Ok(self.window)
	}

	pub fn character<C: Into<Character>>(self, character: C) -> Result<&'a mut Window<'a>> {
		unsafe {
			let character: Character = character.into();

			try!(Error::check(curses::wadd_wch(self.window.as_mut_ptr(),
				character.as_ptr())));
		}

		Ok(self.window)
	}
}
