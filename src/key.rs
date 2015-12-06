use std::char;

use libc::wchar_t;
use curses;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Key {
	Char(char),

	Code(i32),
	Break,
	Reset,
	Down,
	Up,
	Left,
	Right,
	Home,
	Backspace,
	F1,
	F2,
	F3,
	F4,
	F5,
	F6,
	F7,
	F8,
	F9,
	F10,
	F11,
	F12,
	Clear,
	Enter,
	Print,
	Beg,
	Cancel,
	Close,
	Command,
	Copy,
	Create,
	End,
	Exit,
	Find,
	Help,
	Mark,
	Message,
	Move,
	Next,
	Open,
	Options,
	Previous,
	Redo,
	Reference,
	Refresh,
	Replace,
	Restart,
	Resume,
	Save,
	Suspend,
	Undo,
}

impl Key {
	#[inline]
	pub fn character(value: wchar_t) -> Key {
		Key::Char(char::from_u32(value as u32).unwrap())
	}

	#[inline]
	pub fn code(value: wchar_t) -> Key {
		match value {
			curses::KEY_BREAK     => Key::Break,
			curses::KEY_RESET     => Key::Reset,
			curses::KEY_DOWN      => Key::Down,
			curses::KEY_UP        => Key::Up,
			curses::KEY_LEFT      => Key::Left,
			curses::KEY_RIGHT     => Key::Right,
			curses::KEY_HOME      => Key::Home,
			curses::KEY_BACKSPACE => Key::Backspace,
			curses::KEY_F1        => Key::F1,
			curses::KEY_F2        => Key::F2,
			curses::KEY_F3        => Key::F3,
			curses::KEY_F4        => Key::F4,
			curses::KEY_F5        => Key::F5,
			curses::KEY_F6        => Key::F6,
			curses::KEY_F7        => Key::F7,
			curses::KEY_F8        => Key::F8,
			curses::KEY_F9        => Key::F9,
			curses::KEY_F10       => Key::F10,
			curses::KEY_F11       => Key::F11,
			curses::KEY_F12       => Key::F12,
			curses::KEY_CLEAR     => Key::Clear,
			curses::KEY_ENTER     => Key::Enter,
			curses::KEY_PRINT     => Key::Print,
			curses::KEY_BEG       => Key::Beg,
			curses::KEY_CANCEL    => Key::Cancel,
			curses::KEY_CLOSE     => Key::Close,
			curses::KEY_COMMAND   => Key::Command,
			curses::KEY_COPY      => Key::Copy,
			curses::KEY_CREATE    => Key::Create,
			curses::KEY_END       => Key::End,
			curses::KEY_EXIT      => Key::Exit,
			curses::KEY_FIND      => Key::Find,
			curses::KEY_HELP      => Key::Help,
			curses::KEY_MARK      => Key::Mark,
			curses::KEY_MESSAGE   => Key::Message,
			curses::KEY_MOVE      => Key::Move,
			curses::KEY_NEXT      => Key::Next,
			curses::KEY_OPTIONS   => Key::Options,
			curses::KEY_PREVIOUS  => Key::Previous,
			curses::KEY_REDO      => Key::Redo,
			curses::KEY_REFERENCE => Key::Reference,
			curses::KEY_REFRESH   => Key::Refresh,
			curses::KEY_REPLACE   => Key::Replace,
			curses::KEY_RESTART   => Key::Restart,
			curses::KEY_RESUME    => Key::Resume,
			curses::KEY_SAVE      => Key::Save,
			curses::KEY_SUSPEND   => Key::Suspend,
			curses::KEY_UNDO      => Key::Undo,

			v => Key::Code(v as i32)
		}
	}
}
