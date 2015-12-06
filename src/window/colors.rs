use std::ptr;
use libc::{c_short, c_int};
use curses;

use {Error, Result};
use super::Window;

pub struct Colors<'a> {
	window: &'a mut Window<'a>,
}

impl<'a> Colors<'a> {
	#[inline]
	pub unsafe fn wrap<'b>(window: &'b mut Window<'b>) -> Colors<'b> {
		Colors { window: window }
	}
}

impl<'a> Colors<'a> {
	#[inline]
	pub fn set(self, pair: usize) -> Result<&'a mut Window<'a>> {
		unsafe {
			try!(Error::check(curses::wcolor_set(self.window.as_mut_ptr(),
				pair as c_short, ptr::null())));
		}

		Ok(self.window)
	}

	#[inline]
	pub fn current(&self) -> Result<usize> {
		unsafe {
			let mut attr = 0;
			let mut pair = 0;

			try!(Error::check(curses::wattr_get(self.window.as_ptr(),
				&mut attr, &mut pair, ptr::null())));

			Ok(pair as usize)
		}
	}

	#[inline]
	pub fn change(&mut self, pair: usize, len: Option<usize>) -> Result<()> {
		unsafe {
			if let Some(n) = len {
				try!(Error::check(curses::wchgat(self.window.as_mut_ptr(),
					n as c_int, 0, pair as c_short, ptr::null())));
			}
			else {
				try!(Error::check(curses::wchgat(self.window.as_mut_ptr(),
					-1, 0, pair as c_short, ptr::null())));
			}

			Ok(())
		}
	}
}
