use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use libc::c_int;
use curses;

use {Window, Error};
use super::{Colors, Attributes, Capabilities, Settings};
use super::{Input, Clear, Line, Add};

pub struct Screen {
	window: Window,
}

impl Screen {
	#[inline]
	pub unsafe fn wrap(ptr: *mut curses::WINDOW) -> Screen {
		Screen { window: Window::wrap(ptr) }
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
	pub fn line(&mut self) -> Line {
		unsafe {
			Line::wrap(self)
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
	pub fn settings(&mut self) -> Settings {
		unsafe {
			Settings::wrap(self)
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
			try!(Error::check(curses::move_(y as c_int, x as c_int)));
		}

		Ok(self)
	}

	#[inline]
	pub fn input(&mut self) -> Input {
		unsafe {
			Input::wrap(self)
		}
	}

	#[inline]
	pub fn add(&mut self) -> Add {
		unsafe {
			Add::wrap(self)
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
