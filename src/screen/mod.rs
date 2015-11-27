mod screen;
pub use self::screen::Screen;

mod input;
pub use self::input::Input;

mod colors;
pub use self::colors::Colors;

mod attributes;
pub use self::attributes::Attributes;

mod capabilities;
pub use self::capabilities::Capabilities;

mod clear;
pub use self::clear::Clear;

use Error;

#[inline]
pub fn init() -> Result<Screen, Error> {
	unsafe {
		Screen::new()
	}
}
