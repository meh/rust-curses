use std::ptr;
use libc::c_int;
use curses;

use {Error, Result, Attributes as Attr};
use super::Window;

pub struct Attributes<'a> {
	window: &'a mut Window<'a>,
}

impl<'a> Attributes<'a> {
	#[inline]
	pub unsafe fn wrap<'b>(window: &'b mut Window<'b>) -> Attributes<'b> {
		Attributes { window: window }
	}
}

impl<'a> Attributes<'a> {
	#[inline]
	pub fn on(self, attr: Attr) -> Result<&'a mut Window<'a>> {
		unsafe {
			try!(Error::check(curses::wattron(self.window.as_mut_ptr(),
				attr.bits() as c_int)));
		}

		Ok(self.window)
	}

	#[inline]
	pub fn off(self, attr: Attr) -> Result<&'a mut Window<'a>> {
		unsafe {
			try!(Error::check(curses::wattroff(self.window.as_mut_ptr(),
				attr.bits() as c_int)));
		}

		Ok(self.window)
	}

	#[inline]
	pub fn set(self, attr: Attr) -> Result<&'a mut Window<'a>> {
		unsafe {
			try!(Error::check(curses::wattrset(self.window.as_mut_ptr(),
				attr.bits() as c_int)));
		}

		Ok(self.window)
	}

	#[inline]
	pub fn clear(self) -> Result<&'a mut Window<'a>> {
		unsafe {
			try!(Error::check(curses::wstandend(self.window.as_mut_ptr())));
		}

		Ok(self.window)
	}

	#[inline]
	pub fn current(&self) -> Result<Attr> {
		unsafe {
			let mut attr = 0;
			let mut pair = 0;

			try!(Error::check(curses::wattr_get(self.window.as_ptr(),
				&mut attr, &mut pair, ptr::null())));

			Ok(Attr::from_bits_truncate(attr))
		}
	}

	#[inline]
	pub fn change(&mut self, attr: Attr, len: Option<usize>) -> Result<()> {
		unsafe {
			if let Some(n) = len {
				try!(Error::check(curses::wchgat(self.window.as_mut_ptr(),
					n as c_int, attr.bits(), 0, ptr::null())));
			}
			else {
				try!(Error::check(curses::wchgat(self.window.as_mut_ptr(),
					-1, attr.bits(), 0, ptr::null())));
			}
		}

		Ok(())
	}
}
