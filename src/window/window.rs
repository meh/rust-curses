use std::marker::PhantomData;

use libc::c_int;
use curses;

use {Error, Result, Character, Border};
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
			curses::wsyncup(self.as_mut_ptr());
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

	#[inline]
	pub fn border(&mut self, desc: Border) -> Result<&mut Self> {
		unsafe {
			let ls = desc.left.unwrap_or(Character::empty());
			let rs = desc.right.unwrap_or(Character::empty());
			let ts = desc.top.unwrap_or(Character::empty());
			let bs = desc.bottom.unwrap_or(Character::empty());
			let tl = desc.top_left.unwrap_or(Character::empty());
			let tr = desc.top_right.unwrap_or(Character::empty());
			let bl = desc.bottom_left.unwrap_or(Character::empty());
			let br = desc.bottom_right.unwrap_or(Character::empty());

			try!(Error::check(curses::wborder_set(self.as_mut_ptr(),
				cchar!(ls), cchar!(rs),
				cchar!(ts), cchar!(bs),
				cchar!(tl), cchar!(tr),
				cchar!(bl), cchar!(br))));
		}

		Ok(self)
	}
}
