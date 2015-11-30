use unicode_width::UnicodeWidthChar;
use unicode_width::UnicodeWidthStr;

pub trait Width {
	fn width(&self) -> Option<usize>;
}

impl Width for str {
	fn width(&self) -> Option<usize> {
		match UnicodeWidthStr::width(self) {
			0 => None,
			n => Some(n),
		}
	}
}

impl Width for char {
	fn width(&self) -> Option<usize> {
		UnicodeWidthChar::width(*self)
	}
}
