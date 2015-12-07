use curses;

use {Error, Result};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Style {
	Invisible,
	Normal,
	Standout,
}

impl Default for Style {
	fn default() -> Self {
		Style::Normal
	}
}

pub fn set(style: Style) -> Result<()> {
	unsafe {
		try!(Error::check(curses::curs_set(match style {
			Style::Invisible => 0,
			Style::Normal    => 1,
			Style::Standout  => 2,
		})));
	}

	Ok(())
}
