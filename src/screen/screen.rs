use std::ffi::{CStr, CString};
use std::str::from_utf8_unchecked;
use std::time::Duration;

use libc::c_int;
use curses;

use {Window, Error};
use super::{Input, Colors, Attributes, Capabilities, Clear};

pub struct Screen {
	window: Window,
}

impl Screen {
	#[inline]
	pub unsafe fn new() -> Result<Screen, Error> {
		let window = curses::initscr();

		try!(Error::check(curses::start_color()));
		try!(Error::check(curses::use_default_colors()));

		Ok(Screen { window: Window::wrap(window) })
	}
}

impl Screen {
	#[inline]
	pub fn window(&self) -> &Window {
		&self.window
	}

	#[inline]
	pub fn window_mut(&mut self) -> &mut Window {
		&mut self.window
	}

	#[inline]
	pub fn description(&self) -> &'static str {
		unsafe {
			from_utf8_unchecked(CStr::from_ptr(curses::longname()).to_bytes())
		}
	}

	#[inline]
	pub fn baudrate(&self) -> usize {
		unsafe {
			curses::baudrate() as usize
		}
	}

	#[inline]
	pub fn columns(&self) -> usize {
		curses::COLS as usize
	}

	#[inline]
	pub fn rows(&self) -> usize {
		curses::LINES as usize
	}

	#[inline]
	pub fn input(&mut self) -> Input {
		unsafe {
			Input::wrap(self)
		}
	}

	#[inline]
	pub fn colors(&mut self) -> Colors {
		unsafe {
			Colors::wrap(self)
		}
	}

	#[inline]
	pub fn attributes(&mut self) -> Attributes {
		unsafe {
			Attributes::wrap(self)
		}
	}

	#[inline]
	pub fn capabilities(&self) -> Capabilities {
		unsafe {
			Capabilities::wrap(self)
		}
	}

	#[inline]
	pub fn raw(&mut self, value: bool) -> Result<&mut Self, Error> {
		unsafe {
			if value {
				Error::check(curses::raw()).map(|_| self)
			}
			else {
				Error::check(curses::noraw()).map(|_| self)
			}
		}
	}

	#[inline]
	pub fn buffered(&mut self, value: bool) -> Result<&mut Self, Error> {
		unsafe {
			if value {
				Error::check(curses::nocbreak()).map(|_| self)
			}
			else {
				Error::check(curses::cbreak()).map(|_| self)
			}
		}
	}

	#[inline]
	pub fn halfdelay(&mut self, value: Option<Duration>) -> Result<&mut Self, Error> {
		unsafe {
			if let Some(duration) = value {
				let seconds = (duration.as_secs() * 1_000) as u64;
				let nanos   = (duration.subsec_nanos() / 1_000_000) as u64;

				Error::check(curses::halfdelay((seconds + nanos / 100) as c_int)).map(|_| self)
			}
			else {
				Error::check(curses::nocbreak()).map(|_| self)
			}
		}
	}

	#[inline]
	pub fn echo(&mut self, value: bool) -> Result<&mut Self, Error> {
		unsafe {
			if value {
				Error::check(curses::echo()).map(|_| self)
			}
			else {
				Error::check(curses::noecho()).map(|_| self)
			}
		}
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
			Error::check(curses::intrflush(self.window.as_mut_ptr(), value)).map(|_| self)
		}
	}

	#[inline]
	pub fn meta(&mut self, value: bool) -> Result<&mut Self, Error> {
		unsafe {
			Error::check(curses::meta(self.window.as_mut_ptr(), value)).map(|_| self)
		}
	}

	#[inline]
	pub fn refresh(&mut self) -> Result<(), Error> {
		unsafe {
			Error::check(curses::refresh())
		}
	}

	#[inline]
	pub fn update(&mut self) -> Result<(), Error> {
		unsafe {
			Error::check(curses::doupdate())
		}
	}

	#[inline]
	pub fn erase(&mut self) -> Result<(), Error> {
		unsafe {
			Error::check(curses::erase())
		}
	}

	#[inline]
	pub fn clear(&mut self, what: Clear) -> Result<(), Error> {
		unsafe {
			match what {
				Clear::All    => Error::check(curses::clear()),
				Clear::Bottom => Error::check(curses::clrtobot()),
				Clear::Line   => Error::check(curses::clrtoeol()),
			}
		}
	}

	#[inline]
	pub fn beep(&mut self) -> Result<(), Error> {
		unsafe {
			Error::check(curses::beep())
		}
	}

	#[inline]
	pub fn flash(&mut self) -> Result<(), Error> {
		unsafe {
			Error::check(curses::flash())
		}
	}

	#[inline]
	pub fn position(&mut self, x: usize, y: usize) -> Result<&mut Self, Error> {
		unsafe {
			Error::check(curses::move_(y as c_int, x as c_int)).map(|_| self)
		}
	}
	
	#[inline]
	pub fn write<S: AsRef<str>>(&mut self, string: S) -> Result<&mut Self, Error> {
		unsafe {
			let string = CString::new(string.as_ref()).unwrap();

			Error::check(curses::addstr(string.as_ptr())).map(|_| self)
		}
	}
}

impl Drop for Screen {
	#[inline]
	fn drop(&mut self) {
		unsafe {
			curses::endwin();
		}
	}
}
