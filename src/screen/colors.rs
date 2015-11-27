use std::ptr;
use libc::{c_short, c_int};
use curses;

use {Error, Color};
use super::Screen;

pub struct Colors<'a> {
	screen: &'a mut Screen,
}

impl<'a> Colors<'a> {
	#[inline]
	pub unsafe fn wrap(screen: &mut Screen) -> Colors {
		Colors { screen: screen }
	}
}

impl<'a> Colors<'a> {
	#[inline]
	pub fn default(&mut self, fg: Color, bg: Color) -> Result<(), Error> {
		unsafe {
			Error::check(curses::assume_default_colors(fg.into(), bg.into()))
		}
	}
	
	#[inline]
	pub fn available(&self) -> bool {
		unsafe {
			curses::has_colors()
		}
	}
	
	#[inline]
	pub fn redefinable(&self) -> bool {
		unsafe {
			curses::can_change_color()
		}
	}
	
	#[inline]
	pub fn limit(&self) -> usize {
		curses::COLOR_PAIRS as usize
	}
	
	#[inline]
	pub fn define(&mut self, index: usize, (fg, bg): (Color, Color)) -> Result<(), Error> {
		unsafe {
			Error::check(curses::init_pair(index as c_short, fg.into(), bg.into()))
		}
	}
	
	#[inline]
	pub fn definition(&self, index: usize) -> Result<(Color, Color), Error> {
		unsafe {
			let mut f = 0;
			let mut b = 0;
	
			Error::check(curses::pair_content(index as c_short, &mut f, &mut b))
				.map(|_| (Color::from(f), Color::from(b)))
		}
	}
	
	#[inline]
	pub fn redefine(&mut self, color: Color, (r, g, b): (u8, u8, u8)) -> Result<(), Error> {
		unsafe {
			Error::check(curses::init_color(color.into(), r as c_short, g as c_short, b as c_short))
		}
	}
	
	#[inline]
	pub fn redefinition(&self, color: Color) -> Result<(u8, u8, u8), Error> {
		unsafe {
			let mut r = 0;
			let mut g = 0;
			let mut b = 0;
	
			Error::check(curses::color_content(color.into(), &mut r, &mut g, &mut b))
				.map(|_| (r as u8, g as u8, b as u8))
		}
	}

	#[inline]
	pub fn set(self, pair: usize) -> Result<&'a mut Screen, Error> {
		unsafe {
			match Error::check(curses::color_set(pair as c_short, ptr::null())) {
				Ok(..) => Ok(self.screen),
				Err(e) => Err(e)
			}
		}
	}

	#[inline]
	pub fn current(&self) -> Result<usize, Error> {
		unsafe {
			let mut attr = 0;
			let mut pair = 0;

			Error::check(curses::attr_get(&mut attr, &mut pair, ptr::null()))
				.map(|_| pair as usize)
		}
	}

	#[inline]
	pub fn change(&mut self, pair: usize, len: Option<usize>) -> Result<(), Error> {
		unsafe {
			if let Some(n) = len {
				Error::check(curses::chgat(n as c_int, 0, pair as c_short, ptr::null()))
			}
			else {
				Error::check(curses::chgat(-1, 0, pair as c_short, ptr::null()))
			}
		}
	}
}
