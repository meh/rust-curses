use libc::{c_short, c_int};
use curses;

pub enum Color {
	Code(u16),
	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
	White,
	Default,
}

impl Default for Color {
	#[inline]
	fn default() -> Color {
		Color::Default
	}
}

impl From<c_short> for Color {
	#[inline]
	fn from(value: c_short) -> Color {
		match value {
			curses::COLOR_BLACK   => Color::Black,
			curses::COLOR_RED     => Color::Red,
			curses::COLOR_GREEN   => Color::Green,
			curses::COLOR_YELLOW  => Color::Yellow,
			curses::COLOR_BLUE    => Color::Blue,
			curses::COLOR_MAGENTA => Color::Magenta,
			curses::COLOR_CYAN    => Color::Cyan,
			curses::COLOR_WHITE   => Color::White,

			v if v < 0 => Color::Default,
			v          => Color::Code(v as u16),
		}
	}
}

impl Into<c_short> for Color {
	#[inline]
	fn into(self) -> c_short {
		match self {
			Color::Code(value) => value as c_short,
			Color::Default     => -1,
			Color::Black       => curses::COLOR_BLACK,
			Color::Red         => curses::COLOR_RED,
			Color::Green       => curses::COLOR_GREEN,
			Color::Yellow      => curses::COLOR_YELLOW,
			Color::Blue        => curses::COLOR_BLUE,
			Color::Magenta     => curses::COLOR_MAGENTA,
			Color::Cyan        => curses::COLOR_CYAN,
			Color::White       => curses::COLOR_WHITE,
		}
	}
}

impl Into<c_int> for Color {
	#[inline]
	fn into(self) -> c_int {
		let value: c_short = self.into();
		value as c_int
	}
}
