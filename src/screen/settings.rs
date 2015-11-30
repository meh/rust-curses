use std::time::Duration;

use libc::c_int;
use curses;

use Error;
use super::Screen;

pub enum Scroll {
	None,
	Hardware,
	Replace,
}

pub struct Settings<'a> {
	screen: &'a mut Screen,
}

impl<'a> Settings<'a> {
	#[inline]
	pub unsafe fn wrap(screen: &mut Screen) -> Settings {
		Settings { screen: screen }
	}
}

impl<'a> Settings<'a> {
	#[inline]
	pub fn raw(&mut self, value: bool) -> Result<&mut Self, Error> {
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
	pub fn buffered(&mut self, value: bool) -> Result<&mut Self, Error> {
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
	pub fn halfdelay(&mut self, value: Option<Duration>) -> Result<&mut Self, Error> {
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
	pub fn echo(&mut self, value: bool) -> Result<&mut Self, Error> {
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
	pub fn qiflush(&mut self, value: bool) -> Result<&mut Self, Error> {
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
	pub fn timeout(&mut self, value: Option<Duration>) -> Result<&mut Self, Error> {
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
	pub fn flush(&mut self, value: bool) -> Result<&mut Self, Error> {
		unsafe {
			try!(Error::check(curses::intrflush(self.screen.window_mut().as_mut_ptr(), value)));
		}

		Ok(self)
	}

	#[inline]
	pub fn meta(&mut self, value: bool) -> Result<&mut Self, Error> {
		unsafe {
			try!(Error::check(curses::meta(self.screen.window_mut().as_mut_ptr(), value)));
		}

		Ok(self)
	}

	#[inline]
	pub fn newline(&mut self, value: bool) -> Result<&mut Self, Error> {
		unsafe {
			if value {
				try!(Error::check(curses::nl()));
			}
			else {
				try!(Error::check(curses::nonl()))
			}
		}

		Ok(self)
	}

	#[inline]
	pub fn scroll(&mut self, value: Scroll) -> Result<&mut Self, Error> {
		unsafe {
			match value {
				Scroll::None => {
					try!(Error::check(curses::scrollok(self.screen.window_mut().as_mut_ptr(), false)));
				}

				Scroll::Hardware => {
					try!(Error::check(curses::scrollok(self.screen.window_mut().as_mut_ptr(), true)));
					try!(Error::check(curses::idlok(self.screen.window_mut().as_mut_ptr(), true)));
				}

				Scroll::Replace => {
					try!(Error::check(curses::scrollok(self.screen.window_mut().as_mut_ptr(), true)));
					try!(Error::check(curses::idlok(self.screen.window_mut().as_mut_ptr(), false)));
				}
			}
		}

		Ok(self)
	}
}
