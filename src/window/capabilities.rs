use std::time::Duration;
use libc::c_int;
use curses;

use {Error, Result, Scroll};
use super::Window;

pub struct Capabilities<'a> {
	window: &'a mut Window<'a>,
}

impl<'a> Capabilities<'a> {
	#[inline]
	pub unsafe fn wrap<'b>(window: &'b mut Window<'b>) -> Capabilities<'b> {
		Capabilities { window: window }
	}
}

impl<'a> Capabilities<'a> {
	#[inline]
	pub fn sync(&mut self, value: bool) -> Result<&mut Self> {
		unsafe {
			try!(Error::check(curses::syncok(self.window.as_mut_ptr(), value)));
		}

		Ok(self)
	}

	#[inline]
	pub fn keypad(&mut self, value: bool) -> Result<&mut Self> {
		unsafe {
			try!(Error::check(curses::keypad(self.window.as_mut_ptr(), value)));
		}

		Ok(self)
	}

	#[inline]
	pub fn scroll(&mut self, value: Scroll) -> Result<&mut Self> {
		unsafe {
			match value {
				Scroll::None => {
					try!(Error::check(curses::scrollok(self.window.as_mut_ptr(), false)));
				}

				Scroll::Hardware => {
					try!(Error::check(curses::scrollok(self.window.as_mut_ptr(), true)));
					try!(Error::check(curses::idlok(self.window.as_mut_ptr(), true)));
				}

				Scroll::Replace => {
					try!(Error::check(curses::scrollok(self.window.as_mut_ptr(), true)));
					try!(Error::check(curses::idlok(self.window.as_mut_ptr(), false)));
				}
			}
		}

		Ok(self)
	}

	#[inline]
	pub fn timeout(&mut self, value: Option<Duration>) -> Result<&mut Self> {
		unsafe {
			if let Some(duration) = value {
				let seconds = (duration.as_secs() * 1_000) as u64;
				let nanos   = (duration.subsec_nanos() / 1_000_000) as u64;

				curses::wtimeout(self.window.as_mut_ptr(), (seconds + nanos) as c_int);
			}
			else {
				curses::wtimeout(self.window.as_mut_ptr(), -1);
			}

			Ok(self)
		}
	}
}
