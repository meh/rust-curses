use curses;
use Error;
use super::Screen;

pub struct Suspended(Screen);

impl Suspended {
	#[inline]
	pub fn new(screen: Screen) -> Result<Self, Screen> {
		unsafe {
			match Error::check(curses::def_prog_mode()) {
				Ok(..) => {
					curses::endwin();

					Ok(Suspended(screen))
				}

				Err(..) => {
					Err(screen)
				}
			}
		}
	}

	#[inline]
	pub fn resume(self) -> Result<Screen, Self> {
		unsafe {
			match Error::check(curses::reset_prog_mode()) {
				Ok(..) => {
					curses::refresh();

					Ok(self.0)
				}

				Err(..) => {
					Err(self)
				}
			}
		}
	}
}
