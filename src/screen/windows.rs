use libc::c_int;
use curses;

use {Error, Result, Window};
use super::Screen;

pub struct Windows<'a> {
	screen: &'a mut Screen,
}

impl<'a> Windows<'a> {
	#[inline]
	pub unsafe fn wrap(screen: &mut Screen) -> Windows {
		Windows { screen: screen }
	}
}

impl<'a> Windows<'a> {
	#[inline]
	pub fn open<S: AsRef<str>>(&mut self, name: S, (width, height): (u32, u32), (x, y): (u32, u32)) -> Result<Window> {
		if self.screen.windows.contains_key(name.as_ref()) {
			return Err(Error::BadArgument);
		}

		unsafe {
			let window = curses::newwin(height as c_int, width as c_int, y as c_int, x as c_int);
			self.screen.windows.insert(name.as_ref().to_owned(), window as *mut _);

			Ok(Window::wrap(window))
		}
	}

	#[inline]
	pub fn get<S: AsRef<str>>(&mut self, name: S) -> Result<Window> {
		if self.screen.windows.contains_key(name.as_ref()) {
			return Err(Error::BadArgument);
		}

		unsafe {
			Ok(Window::wrap(*self.screen.windows.get(name.as_ref()).unwrap()))
		}
	}

	#[inline]
	pub fn close<S: AsRef<str>>(&mut self, name: S) -> Result<()> {
		if !self.screen.windows.contains_key(name.as_ref()) {
			return Err(Error::BadArgument);
		}

		unsafe {
			try!(Error::check(curses::delwin(
				self.screen.windows.remove(name.as_ref()).unwrap() as *mut _)));
		}

		Ok(())
	}
}
