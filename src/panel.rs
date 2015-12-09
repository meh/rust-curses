use std::ops::{Deref, DerefMut};

use libc::c_int;
use curses;
use {Error, Result, Window};

pub struct Panel<'a> {
	ptr:    *mut curses::PANEL,
	window: Window<'a>,
}

impl<'a> Panel<'a> {
	#[inline]
	pub unsafe fn wrap<'b>(ptr: *mut curses::PANEL) -> Panel<'b> {
		Panel { ptr: ptr, window: Window::wrap(curses::panel_window(ptr)) }
	}

	#[inline]
	pub unsafe fn as_ptr(&self) -> *const curses::PANEL {
		self.ptr as *const _
	}

	#[inline]
	pub unsafe fn as_mut_ptr(&mut self) -> *mut curses::PANEL {
		self.ptr
	}
}

impl<'a> Panel<'a> {
	#[inline]
	pub fn show(&mut self) -> Result<&mut Self> {
		unsafe {
			try!(Error::check(curses::show_panel(self.as_mut_ptr())));
		}

		Ok(self)
	}

	#[inline]
	pub fn hide(&mut self) -> Result<&mut Self> {
		unsafe {
			try!(Error::check(curses::hide_panel(self.as_mut_ptr())));
		}

		Ok(self)
	}

	#[inline]
	pub fn is_visible(&self) -> bool {
		unsafe {
			curses::panel_hidden(self.as_ptr()) == 0
		}
	}

	#[inline]
	pub fn top(&mut self) -> Result<&mut Self> {
		unsafe {
			try!(Error::check(curses::top_panel(self.as_mut_ptr())));
		}

		Ok(self)
	}

	#[inline]
	pub fn bottom(&mut self) -> Result<&mut Self> {
		unsafe {
			try!(Error::check(curses::bottom_panel(self.as_mut_ptr())));
		}

		Ok(self)
	}

	#[inline]
	pub fn position(&mut self, x: u32, y: u32) -> Result<&mut Self> {
		unsafe {
			try!(Error::check(curses::move_panel(self.as_mut_ptr(), y as c_int, x as c_int)));
		}

		Ok(self)
	}
}

impl<'a> Deref for Panel<'a> {
	type Target = Window<'a>;

	fn deref(&self) -> &Self::Target {
		&self.window
	}
}

impl<'a> DerefMut for Panel<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.window
	}
}
