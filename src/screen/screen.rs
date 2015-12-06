use std::ffi::CStr;
use std::str::from_utf8_unchecked;

use libc::c_int;
use curses;

use {Error, Result};
use super::{Colors, Attributes, Capabilities};
use super::{Input, Clear, Line, Add};

pub struct Screen {
	pub window: *mut curses::WINDOW,
}

unsafe impl Send for Screen { }

impl Screen {
	#[inline]
	pub unsafe fn wrap(ptr: *mut curses::WINDOW) -> Screen {
		Screen {
			window: ptr,
		}
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
	pub fn columns(&self) -> u32 {
		curses::COLS as u32
	}

	#[inline]
	pub fn rows(&self) -> u32 {
		curses::LINES as u32
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
	pub fn capabilities(&mut self) -> Capabilities {
		unsafe {
			Capabilities::wrap(self)
		}
	}

	#[inline]
	pub fn refresh(&mut self) -> Result<()> {
		unsafe {
			try!(Error::check(curses::refresh()));
		}

		Ok(())
	}

	#[inline]
	pub fn update(&mut self) -> Result<()> {
		unsafe {
			try!(Error::check(curses::doupdate()));
		}

		Ok(())
	}

	#[inline]
	pub fn erase(&mut self) -> Result<()> {
		unsafe {
			try!(Error::check(curses::erase()));
		}

		Ok(())
	}

	#[inline]
	pub fn clear(&mut self, what: Clear) -> Result<()> {
		unsafe {
			match what {
				Clear::All => {
					try!(Error::check(curses::clear()));
				}

				Clear::Bottom => {
					try!(Error::check(curses::clrtobot()));
				}

				Clear::Line => {
					try!(Error::check(curses::clrtoeol()));
				}
			}
		}

		Ok(())
	}

	#[inline]
	pub fn beep(&mut self) -> Result<()> {
		unsafe {
			try!(Error::check(curses::beep()));
		}

		Ok(())
	}

	#[inline]
	pub fn flash(&mut self) -> Result<()> {
		unsafe {
			try!(Error::check(curses::flash()));
		}

		Ok(())
	}

	#[inline]
	pub fn cursor(&mut self, x: u32, y: u32) -> Result<&mut Self> {
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
