use libc::{c_int, wchar_t};
use curses;

use {Key, Mouse};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Event {
	Key(Key),
	Mouse(Mouse),
	Resize,
}

impl Event {
	pub fn fetch(value: c_int, character: wchar_t) -> Option<Event> {
		unsafe {
			if value != curses::KEY_CODE_YES {
				return Some(Event::Key(Key::character(character)));
			}

			match character {
				curses::KEY_MOUSE => {
					let mut mouse = Mouse::new();
					
					if curses::getmouse(mouse.as_mut_ptr()) == curses::E_OK {
						Some(Event::Mouse(mouse))
					}
					else {
						None
					}
				}

				curses::KEY_RESIZE =>
					Some(Event::Resize),

				_ =>
					Some(Event::Key(Key::code(character)))
			}
		}
	}
}
