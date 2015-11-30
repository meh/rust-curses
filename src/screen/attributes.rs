use std::ptr;
use libc::c_int;
use curses;

use {Error, Attributes as Attr};
use super::Screen;

pub struct Attributes<'a> {
	screen: &'a mut Screen,
}

impl<'a> Attributes<'a> {
	#[inline]
	pub unsafe fn wrap(screen: &mut Screen) -> Attributes {
		Attributes { screen: screen }
	}
}

impl<'a> Attributes<'a> {
	#[inline]
	pub fn on(self, attr: Attr) -> Result<&'a mut Screen, Error> {
		unsafe {
			try!(Error::check(curses::attron(attr.bits() as c_int)));
		}

		Ok(self.screen)
	}

	#[inline]
	pub fn off(self, attr: Attr) -> Result<&'a mut Screen, Error> {
		unsafe {
			try!(Error::check(curses::attroff(attr.bits() as c_int)));
		}

		Ok(self.screen)
	}

	#[inline]
	pub fn set(self, attr: Attr) -> Result<&'a mut Screen, Error> {
		unsafe {
			try!(Error::check(curses::attrset(attr.bits() as c_int)));
		}

		Ok(self.screen)
	}

	#[inline]
	pub fn clear(self) -> Result<&'a mut Screen, Error> {
		unsafe {
			try!(Error::check(curses::standend()));
		}

		Ok(self.screen)
	}

	#[inline]
	pub fn supported(&self) -> Attr {
		unsafe {
			Attr::from_bits_truncate(curses::termattrs())
		}
	}

	#[inline]
	pub fn current(&self) -> Result<Attr, Error> {
		unsafe {
			let mut attr = 0;
			let mut pair = 0;

			Error::check(curses::attr_get(&mut attr, &mut pair, ptr::null()))
				.map(|_| Attr::from_bits_truncate(attr))
		}
	}

	#[inline]
	pub fn change(&mut self, attr: Attr, len: Option<usize>) -> Result<(), Error> {
		unsafe {
			if let Some(n) = len {
				Error::check(curses::chgat(n as c_int, attr.bits(), 0, ptr::null()))
			}
			else {
				Error::check(curses::chgat(-1, attr.bits(), 0, ptr::null()))
			}
		}
	}
}
