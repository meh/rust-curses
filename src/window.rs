use curses;
use Error;

pub struct Window {
	ptr: *mut curses::WINDOW,
}

impl Window {
	#[inline]
	pub unsafe fn wrap(ptr: *mut curses::WINDOW) -> Window {
		Window {
			ptr: ptr
		}
	}

	#[inline]
	pub unsafe fn as_ptr(&self) -> *const curses::WINDOW {
		self.ptr as *const _
	}

	#[inline]
	pub unsafe fn as_mut_ptr(&mut self) -> *mut curses::WINDOW {
		self.ptr
	}
}

impl Window {
	#[inline]
	pub fn keypad(&mut self, value: bool) -> Result<&mut Self, Error> {
		unsafe {
			Error::check(curses::keypad(self.as_mut_ptr(), value)).map(|_| self)
		}
	}
}

impl Drop for Window {
	#[inline]
	fn drop(&mut self) {
		unsafe {
			curses::delwin(self.ptr);
		}
	}
}
