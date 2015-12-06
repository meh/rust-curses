use std::marker::PhantomData;

use libc::c_int;
use curses;

use {Error, Result};
use super::{Colors, Attributes, Capabilities};
use super::{Input, Add};

pub struct Window<'a> {
	ptr: *mut curses::WINDOW,

	_marker: PhantomData<&'a mut ()>,
}

impl<'a> Window<'a> {
	#[inline]
	pub unsafe fn wrap<'b>(ptr: *mut curses::WINDOW) -> Window<'b> {
		Window {
			ptr: ptr,

			_marker: PhantomData,
		}
	}

	#[inline]
	pub unsafe fn as_ptr(&self) -> *const curses::WINDOW {
		self.ptr as *const _
	}

	#[inline]
	pub unsafe fn as_mut_ptr(&mut self) -> *mut curses::WINDOW {
		self.ptr
	}
}

impl<'a> Window<'a> {
	#[inline]
	pub fn colors(&'a mut self) -> Colors {
		unsafe {
			Colors::wrap(self)
		}
	}

	#[inline]
	pub fn attributes(&'a mut self) -> Attributes {
		unsafe {
			Attributes::wrap(self)
		}
	}

	#[inline]
	pub fn capabilities(&'a mut self) -> Capabilities {
		unsafe {
			Capabilities::wrap(self)
		}
	}

	#[inline]
	pub fn position(&mut self, x: u32, y: u32) -> Result<()> {
		unsafe {
			try!(Error::check(curses::mvwin(self.as_mut_ptr(), y as c_int, x as c_int)));
		}

		Ok(())
	}

	#[inline]
	pub fn sync(&mut self) -> Result<()> {
		unsafe {
			try!(Error::check(curses::wsyncup(self.as_mut_ptr())));
		}

		Ok(())
	}

	#[inline]
	pub fn refresh(&mut self) -> Result<()> {
		unsafe {
			try!(Error::check(curses::wrefresh(self.as_mut_ptr())));
		}

		Ok(())
	}

	#[inline]
	pub fn cursor(&mut self, x: u32, y: u32) -> Result<&mut Self> {
		unsafe {
			try!(Error::check(curses::wmove(self.as_mut_ptr(), y as c_int, x as c_int)));
		}

		Ok(self)
	}

	#[inline]
	pub fn input(&'a mut self) -> Input {
		unsafe {
			Input::wrap(self)
		}
	}

	#[inline]
	pub fn add(&'a mut self) -> Add {
		unsafe {
			Add::wrap(self)
		}
	}
}
