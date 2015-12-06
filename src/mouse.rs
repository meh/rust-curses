use std::mem;

use curses;

bitflags! {
	flags Buttons: curses::mmask_t {
		const BUTTON_RELEASED = curses::BUTTON_RELEASED,
		const BUTTON_PRESSED  = curses::BUTTON_PRESSED,
		const BUTTON_CLICKED  = curses::BUTTON_CLICKED,
		const DOUBLE_CLICKED  = curses::DOUBLE_CLICKED,
		const TRIPLE_CLICKED  = curses::TRIPLE_CLICKED,

		const BUTTON1_RELEASED       = curses::BUTTON1_RELEASED,
		const BUTTON1_PRESSED        = curses::BUTTON1_PRESSED,
		const BUTTON1_CLICKED        = curses::BUTTON1_CLICKED,
		const BUTTON1_DOUBLE_CLICKED = curses::BUTTON1_DOUBLE_CLICKED,
		const BUTTON1_TRIPLE_CLICKED = curses::BUTTON1_TRIPLE_CLICKED,

		const BUTTON2_RELEASED       = curses::BUTTON2_RELEASED,
		const BUTTON2_PRESSED        = curses::BUTTON2_PRESSED,
		const BUTTON2_CLICKED        = curses::BUTTON2_CLICKED,
		const BUTTON2_DOUBLE_CLICKED = curses::BUTTON2_DOUBLE_CLICKED,
		const BUTTON2_TRIPLE_CLICKED = curses::BUTTON2_TRIPLE_CLICKED,

		const BUTTON3_RELEASED       = curses::BUTTON3_RELEASED,
		const BUTTON3_PRESSED        = curses::BUTTON3_PRESSED,
		const BUTTON3_CLICKED        = curses::BUTTON3_CLICKED,
		const BUTTON3_DOUBLE_CLICKED = curses::BUTTON3_DOUBLE_CLICKED,
		const BUTTON3_TRIPLE_CLICKED = curses::BUTTON3_TRIPLE_CLICKED,

		const BUTTON4_RELEASED       = curses::BUTTON4_RELEASED,
		const BUTTON4_PRESSED        = curses::BUTTON4_PRESSED,
		const BUTTON4_CLICKED        = curses::BUTTON4_CLICKED,
		const BUTTON4_DOUBLE_CLICKED = curses::BUTTON4_DOUBLE_CLICKED,
		const BUTTON4_TRIPLE_CLICKED = curses::BUTTON4_TRIPLE_CLICKED,

		const BUTTON5_RELEASED       = curses::BUTTON5_RELEASED,
		const BUTTON5_PRESSED        = curses::BUTTON5_PRESSED,
		const BUTTON5_CLICKED        = curses::BUTTON5_CLICKED,
		const BUTTON5_DOUBLE_CLICKED = curses::BUTTON5_DOUBLE_CLICKED,
		const BUTTON5_TRIPLE_CLICKED = curses::BUTTON5_TRIPLE_CLICKED,

		const BUTTON_CTRL  = curses::BUTTON_CTRL,
		const BUTTON_SHIFT = curses::BUTTON_SHIFT,
		const BUTTON_ALT   = curses::BUTTON_ALT,
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Mouse(curses::MEVENT);

impl Mouse {
	#[inline]
	pub unsafe fn new() -> Mouse {
		mem::zeroed()
	}

	#[inline]
	pub unsafe fn as_ptr(&self) -> *const curses::MEVENT {
		&self.0
	}

	#[inline]
	pub unsafe fn as_mut_ptr(&mut self) -> *mut curses::MEVENT {
		&mut self.0
	}
}

impl Mouse {
	#[inline]
	pub fn id(&self) -> i16 {
		self.0.id as i16
	}

	#[inline]
	pub fn x(&self) -> u32 {
		self.0.x as u32
	}

	#[inline]
	pub fn y(&self) -> u32 {
		self.0.y as u32
	}

	#[inline]
	pub fn z(&self) -> u32 {
		self.0.z as u32
	}

	pub fn buttons(&self) -> Buttons {
		Buttons::from_bits_truncate(self.0.bstate)
	}
}
