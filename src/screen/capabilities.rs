use std::char;
use curses;

use super::Screen;

#[allow(dead_code)]
pub struct Capabilities<'a> {
	screen: &'a Screen,
}

impl<'a> Capabilities<'a> {
	#[inline]
	pub unsafe fn wrap(screen: &Screen) -> Capabilities {
		Capabilities { screen: screen }
	}
}

impl<'a> Capabilities<'a> {
	#[inline]
	pub fn insert_delete(&self) -> bool {
		unsafe {
			curses::has_ic()
		}
	}

	#[inline]
	pub fn simulates_insert_delete(&self) -> bool {
		unsafe {
			curses::has_il()
		}
	}

	#[inline]
	pub fn kill(&self) -> char {
		unsafe {
			char::from_u32(curses::killchar() as u32).unwrap()
		}
	}

	#[inline]
	pub fn erase(&self) -> char {
		unsafe {
			char::from_u32(curses::erasechar() as u32).unwrap()
		}
	}
}
