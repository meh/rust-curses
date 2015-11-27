use curses;

use super::Screen;

#[allow(dead_code)]
pub struct Input<'a> {
	screen: &'a mut Screen,
}

impl<'a> Input<'a> {
	#[inline]
	pub unsafe fn wrap(screen: &mut Screen) -> Input {
		Input { screen: screen }
	}
}

impl<'a> Input<'a> {
	#[inline]
	pub fn character(&mut self) -> i32 {
		unsafe {
			curses::getch()
		}
	}
}
