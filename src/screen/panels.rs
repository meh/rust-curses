use std::ptr;
use libc::c_int;
use curses;

use {Error, Result, Panel};
use super::Screen;

pub struct Panels<'a> {
	screen: &'a mut Screen,
}

impl<'a> Panels<'a> {
	#[inline]
	pub unsafe fn wrap(screen: &mut Screen) -> Panels {
		Panels { screen: screen }
	}
}

impl<'a> Panels<'a> {
	#[inline]
	pub fn open<S: AsRef<str>>(&mut self, name: S, (width, height): (u32, u32), (x, y): (u32, u32)) -> Result<Panel> {
		if self.screen.panels.contains_key(name.as_ref()) {
			return Err(Error::BadArgument);
		}

		unsafe {
			let window = curses::newwin(height as c_int, width as c_int, y as c_int, x as c_int);
			let panel  = curses::new_panel(window);

			self.screen.panels.insert(name.as_ref().to_owned(), panel as *mut _);

			Ok(Panel::wrap(panel))
		}
	}

	#[inline]
	pub fn get<S: AsRef<str>>(&mut self, name: S) -> Result<Panel> {
		if !self.screen.panels.contains_key(name.as_ref()) {
			return Err(Error::BadArgument);
		}

		unsafe {
			let &panel = self.screen.panels.get(name.as_ref()).unwrap();

			Ok(Panel::wrap(panel as *mut _))
		}
	}

	#[inline]
	pub fn top(&mut self) -> Result<Panel> {
		unsafe {
			let panel = curses::panel_below(ptr::null());

			if panel.is_null() {
				return Err(Error::NoMatch);
			}

			Ok(Panel::wrap(panel))
		}
	}

	#[inline]
	pub fn bottom(&mut self) -> Result<Panel> {
		unsafe {
			let panel = curses::panel_above(ptr::null());

			if panel.is_null() {
				return Err(Error::NoMatch);
			}

			Ok(Panel::wrap(panel))
		}
	}

	#[inline]
	pub fn above<S: AsRef<str>>(&mut self, name: S) -> Result<Panel> {
		if !self.screen.panels.contains_key(name.as_ref()) {
			return Err(Error::BadArgument);
		}

		unsafe {
			let &panel = self.screen.panels.get(name.as_ref()).unwrap();
			let panel  = curses::panel_above(panel as *const _);

			if panel.is_null() {
				return Err(Error::NoMatch);
			}

			Ok(Panel::wrap(panel as *mut _))
		}
	}

	#[inline]
	pub fn below<S: AsRef<str>>(&mut self, name: S) -> Result<Panel> {
		if !self.screen.panels.contains_key(name.as_ref()) {
			return Err(Error::BadArgument);
		}

		unsafe {
			let &panel = self.screen.panels.get(name.as_ref()).unwrap();
			let panel  = curses::panel_below(panel as *const _);

			if panel.is_null() {
				return Err(Error::NoMatch);
			}

			Ok(Panel::wrap(panel as *mut _))
		}
	}

	#[inline]
	pub fn update(&mut self) -> Result<()> {
		unsafe {
			curses::update_panels();
		}

		Ok(())
	}

	#[inline]
	pub fn close<S: AsRef<str>>(&mut self, name: S) -> Result<()> {
		if !self.screen.panels.contains_key(name.as_ref()) {
			return Err(Error::BadArgument);
		}

		unsafe {
			let panel  = self.screen.panels.remove(name.as_ref()).unwrap();
			let window = curses::panel_window(panel as *const _);

			try!(Error::check(curses::del_panel(panel as *mut _)));
			try!(Error::check(curses::delwin(window)));
		}

		Ok(())
	}
}
