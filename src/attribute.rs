use libc::c_uint;
use curses;

bitflags! {
	flags Attributes: c_uint {
		const NORMAL            = curses::A_NORMAL,
		const COLOR             = curses::A_COLOR,
		const STANDOUT          = curses::A_STANDOUT,
		const UNDERLINE         = curses::A_UNDERLINE,
		const REVERSE           = curses::A_REVERSE,
		const BLINK             = curses::A_BLINK,
		const DIM               = curses::A_DIM,
		const BOLD              = curses::A_BOLD,
		const ALTERNATE_CHARSET = curses::A_ALTCHARSET,
		const INVISIBLE         = curses::A_INVIS,
		const PROTECTED         = curses::A_PROTECT,
		const HORIZONTAL        = curses::A_HORIZONTAL,
		const LEFT              = curses::A_LEFT,
		const LOW               = curses::A_LOW,
		const RIGHT             = curses::A_RIGHT,
		const TOP               = curses::A_TOP,
		const VERTICAL          = curses::A_VERTICAL,
		const ITALIC            = curses::A_ITALIC,
	}
}
