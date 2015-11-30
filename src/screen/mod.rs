mod screen;
pub use self::screen::Screen;

mod settings;
pub use self::settings::{Settings, Scroll};

mod input;
pub use self::input::Input;

mod add;
pub use self::add::Add;

mod line;
pub use self::line::Line;

mod colors;
pub use self::colors::Colors;

mod attributes;
pub use self::attributes::Attributes;

mod capabilities;
pub use self::capabilities::Capabilities;

mod clear;
pub use self::clear::Clear;

use Error;
use curses;

#[inline]
pub fn init() -> Result<Screen, Error> {
	use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

	static INITIALIZED: AtomicBool = ATOMIC_BOOL_INIT;

	if INITIALIZED.load(Ordering::Relaxed) {
		return Err(Error::SystemError);
	}

	INITIALIZED.store(true, Ordering::Relaxed);

	unsafe {
		locale::setlocale(locale::LC_ALL, b"\0".as_ptr() as *const _);

		let window = curses::initscr();

		try!(Error::check(curses::start_color()));
		try!(Error::check(curses::use_default_colors()));

		Ok(Screen::wrap(window))
	}
}

mod locale {
	use libc::{c_int, c_char};
	pub const LC_ALL: c_int = 6;

	extern "C" {
		pub fn setlocale(category: c_int, locale: *const c_char) -> *mut c_char;
	}
}
