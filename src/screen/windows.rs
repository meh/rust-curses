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
		if !self.screen.windows.contains_key(name.as_ref()) {
			return Err(Error::BadArgument);
		}

		unsafe {
			Ok(Window::wrap(*self.screen.windows.get(name.as_ref()).unwrap()))
		}
	}

	#[inline]
	pub fn overlay<S: AsRef<str>, D: AsRef<str>>(&mut self, src: S, dst: D) -> Result<()> {
		if !self.screen.windows.contains_key(src.as_ref()) || !self.screen.windows.contains_key(dst.as_ref()) {
			return Err(Error::BadArgument);
		}

		unsafe {
			try!(Error::check(curses::overlay(
				*self.screen.windows.get(src.as_ref()).unwrap(),
				*self.screen.windows.get(dst.as_ref()).unwrap())));
		}

		Ok(())
	}

	#[inline]
	pub fn overwrite<S: AsRef<str>, D: AsRef<str>>(&mut self, src: S, dst: D) -> Result<()> {
		if !self.screen.windows.contains_key(src.as_ref()) || !self.screen.windows.contains_key(dst.as_ref()) {
			return Err(Error::BadArgument);
		}

		unsafe {
			try!(Error::check(curses::overwrite(
				*self.screen.windows.get(src.as_ref()).unwrap(),
				*self.screen.windows.get(dst.as_ref()).unwrap())));
		}

		Ok(())
	}

	pub fn copy<S: AsRef<str>, D: AsRef<str>>(&mut self, src: S, dst: D,
	                                                     (src_min_col, src_min_row): (u32, u32),
	                                                     (dst_min_col, dst_min_row): (u32, u32),
	                                                     (dst_max_col, dst_max_row): (u32, u32),
	                                                     overlay: bool) -> Result<()> {
		if !self.screen.windows.contains_key(src.as_ref()) || !self.screen.windows.contains_key(dst.as_ref()) {
			return Err(Error::BadArgument);
		}

		unsafe {
			try!(Error::check(curses::copywin(
				*self.screen.windows.get(src.as_ref()).unwrap(),
				*self.screen.windows.get(dst.as_ref()).unwrap(),
				src_min_row as c_int, src_min_col as c_int,
				dst_min_row as c_int, dst_min_col as c_int,
				dst_max_row as c_int, dst_max_col as c_int,
				if overlay { 1 } else { 0 })));
		}

		Ok(())
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
