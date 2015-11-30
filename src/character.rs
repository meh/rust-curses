use std::mem;
use std::char;

use unicode_segmentation::UnicodeSegmentation;

use libc::{wchar_t};
use curses;

use {Attributes, Width};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Character(curses::cchar_t);

impl Character {
	#[inline]
	pub unsafe fn as_ptr(&self) -> *const curses::cchar_t {
		&self.0
	}
}

impl Character {
	#[inline]
	pub fn attributes(&self) -> Attributes {
		Attributes::from_bits_truncate(self.0.attr)
	}

	#[inline]
	pub fn set_attributes(&mut self, value: Attributes) {
		self.0.attr = value.bits();
	}
}

impl Width for Character {
	fn width(&self) -> Option<usize> {
		let mut result = 0;

		for ch in &self.0.chars[..] {
			match char::from_u32(*ch as u32).unwrap().width() {
				Some(width) => {
					result += width;
				}

				None => {
					if result > 0 {
						break;
					}
					else {
						return None;
					}
				}
			}
		}

		Some(result)
	}
}

impl From<char> for Character {
	#[inline]
	fn from(value: char) -> Character {
		unsafe {
			let mut ch: curses::cchar_t = mem::zeroed();
			ch.chars[0] = value as wchar_t;

			Character(ch)
		}
	}
}

impl<'a> From<&'a str> for Character {
	#[inline]
	fn from(value: &'a str) -> Character {
		unsafe {
			let mut ch: curses::cchar_t = mem::zeroed();
			for (i, code) in value.graphemes(true).next().unwrap().chars().enumerate() {
				ch.chars[i] = code as wchar_t;
			}

			Character(ch)
		}
	}
}