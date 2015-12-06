use libc::c_int;
use curses;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Error {
	SystemError,
	BadArgument,
	Posted,
	Connected,
	BadState,
	NoRoom,
	NotPosted,
	UnknownCommand,
	NoMatch,
	NotSelectable,
	NotConnected,
	RequestDenied,
	InvalidField,
	Current,
}

impl Error {
	#[inline]
	pub fn check(value: c_int) -> Result<c_int, Error> {
		if value < 0 {
			Err(Error::from(value))
		}
		else {
			Ok(value)
		}
	}
}

impl From<c_int> for Error {
	#[inline]
	fn from(value: c_int) -> Error {
		match value {
			curses::E_SYSTEM_ERROR    => Error::SystemError,
			curses::E_BAD_ARGUMENT    => Error::BadArgument,
			curses::E_POSTED          => Error::Posted,
			curses::E_CONNECTED       => Error::Connected,
			curses::E_BAD_STATE       => Error::BadState,
			curses::E_NO_ROOM         => Error::NoRoom,
			curses::E_NOT_POSTED      => Error::NotPosted,
			curses::E_UNKNOWN_COMMAND => Error::UnknownCommand,
			curses::E_NO_MATCH        => Error::NoMatch,
			curses::E_NOT_SELECTABLE  => Error::NotSelectable,
			curses::E_NOT_CONNECTED   => Error::NotConnected,
			curses::E_REQUEST_DENIED  => Error::RequestDenied,
			curses::E_INVALID_FIELD   => Error::InvalidField,
			curses::E_CURRENT         => Error::Current,

			_ => panic!("unknown error"),
		}
	}
}
