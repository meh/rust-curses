use std::char;
use std::time::Duration;
use libc::c_int;
use curses;

use {Error, Result, Scroll};
use super::Screen;

pub struct Capabilities<'a> {
	screen: &'a mut Screen,
}

impl<'a> Capabilities<'a> {
	#[inline]
	pub unsafe fn wrap(screen: &mut Screen) -> Capabilities {
		Capabilities { screen: screen }
	}
}

impl<'a> Capabilities<'a> {
	#[inline]
	pub fn mouse(&self) -> bool {
		unsafe {
			curses::has_mouse()
		}
	}

	#[inline]
	pub fn insert_delete(&self) -> bool {
		unsafe {
			curses::has_ic()
		}
	}

	#[inline]
	pub fn simulates_insert_delete(&self) -> bool {
		unsafe {
			curses::has_il()
		}
	}

	#[inline]
	pub fn kill(&self) -> char {
		unsafe {
			char::from_u32(curses::killchar() as u32).unwrap()
		}
	}

	#[inline]
	pub fn erase(&self) -> char {
		unsafe {
			char::from_u32(curses::erasechar() as u32).unwrap()
		}
	}

	#[inline]
	pub fn raw(&mut self, value: bool) -> Result<&mut Self> {
		unsafe {
			if value {
				try!(Error::check(curses::raw()));
			}
			else {
				try!(Error::check(curses::noraw()));
			}
		}

		Ok(self)
	}

	#[inline]
	pub fn buffered(&mut self, value: bool) -> Result<&mut Self> {
		unsafe {
			if value {
				try!(Error::check(curses::nocbreak()));
			}
			else {
				try!(Error::check(curses::cbreak()));
			}
		}

		Ok(self)
	}

	#[inline]
	pub fn halfdelay(&mut self, value: Option<Duration>) -> Result<&mut Self> {
		unsafe {
			if let Some(duration) = value {
				let seconds = (duration.as_secs() * 1_000) as u64;
				let nanos   = (duration.subsec_nanos() / 1_000_000) as u64;

				try!(Error::check(curses::halfdelay((seconds + nanos / 100) as c_int)));
			}
			else {
				try!(Error::check(curses::nocbreak()));
			}
		}

		Ok(self)
	}

	#[inline]
	pub fn echo(&mut self, value: bool) -> Result<&mut Self> {
		unsafe {
			if value {
				try!(Error::check(curses::echo()));
			}
			else {
				try!(Error::check(curses::noecho()));
			}
		}

		Ok(self)
	}

	#[inline]
	pub fn qiflush(&mut self, value: bool) -> Result<&mut Self> {
		unsafe {
			if value {
				curses::qiflush();
			}
			else {
				curses::noqiflush();
			}

			Ok(self)
		}
	}

	#[inline]
	pub fn timeout(&mut self, value: Option<Duration>) -> Result<&mut Self> {
		unsafe {
			if let Some(duration) = value {
				let seconds = (duration.as_secs() * 1_000) as u64;
				let nanos   = (duration.subsec_nanos() / 1_000_000) as u64;

				curses::timeout((seconds + nanos) as c_int);
			}
			else {
				curses::timeout(-1);
			}

			Ok(self)
		}
	}

	#[inline]
	pub fn flush(&mut self, value: bool) -> Result<&mut Self> {
		unsafe {
			try!(Error::check(curses::intrflush(self.screen.window, value)));
		}

		Ok(self)
	}

	#[inline]
	pub fn meta(&mut self, value: bool) -> Result<&mut Self> {
		unsafe {
			try!(Error::check(curses::meta(self.screen.window, value)));
		}

		Ok(self)
	}

	#[inline]
	pub fn keypad(&mut self, value: bool) -> Result<&mut Self> {
		unsafe {
			try!(Error::check(curses::keypad(self.screen.window, value)));
		}

		Ok(self)
	}

	#[inline]
	pub fn newline(&mut self, value: bool) -> Result<&mut Self> {
		unsafe {
			if value {
				try!(Error::check(curses::nl()));
			}
			else {
				try!(Error::check(curses::nonl()));
			}
		}

		Ok(self)
	}

	#[inline]
	pub fn scroll(&mut self, value: Scroll) -> Result<&mut Self> {
		unsafe {
			match value {
				Scroll::None => {
					try!(Error::check(curses::scrollok(self.screen.window, false)));
				}

				Scroll::Hardware => {
					try!(Error::check(curses::scrollok(self.screen.window, true)));
					try!(Error::check(curses::idlok(self.screen.window, true)));
				}

				Scroll::Replace => {
					try!(Error::check(curses::scrollok(self.screen.window, true)));
					try!(Error::check(curses::idlok(self.screen.window, false)));
				}
			}
		}

		Ok(self)
	}
}
