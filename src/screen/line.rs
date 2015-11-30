use libc::c_int;
use curses;

use Error;
use super::Screen;

#[allow(dead_code)]
pub struct Line<'a> {
	screen: &'a mut Screen,
}

impl<'a> Line<'a> {
	#[inline]
	pub unsafe fn wrap(screen: &mut Screen) -> Line {
		Line { screen: screen }
	}
}

impl<'a> Line<'a> {
	#[inline]
	pub fn delete(&mut self) -> Result<(), Error> {
		unsafe {
			Error::check(curses::deleteln())
		}
	}

	#[inline]
	pub fn insert(&mut self, amount: usize) -> Result<(), Error> {
		unsafe {
			Error::check(curses::insdelln(amount as c_int))
		}
	}

	#[inline]
	pub fn replace(&mut self) -> Result<(), Error> {
		unsafe {
			Error::check(curses::insertln())
		}
	}
}
