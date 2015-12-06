use curses;

use {Character};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Symbol {
	UpperLeftCorner,
	LowerLeftCorner,
	UpperRightCorner,
	LowerRightCorner,

	LeftTee,
	RightTee,
	DownTee,
	UpTee,

	HorizontalLine,
	VerticalLine,

	Plus,
	PlusMinus,
	LessEqual,
	GreaterEqual,
	NotEqual,

	Degree,
	Pi,
	Sterling,

	Diamond,
	CheckerBoard,
	Bullet,
	Board,
	Lantern,
	Block,
}

impl Into<Character> for Symbol {
	fn into(self) -> Character {
		unsafe {
			Character::from(curses::WACS(match self {
				Symbol::UpperLeftCorner  => curses::ULCORNER,
				Symbol::LowerLeftCorner  => curses::LLCORNER,
				Symbol::UpperRightCorner => curses::URCORNER,
				Symbol::LowerRightCorner => curses::ULCORNER,

				Symbol::LeftTee  => curses::LTEE,
				Symbol::RightTee => curses::RTEE,
				Symbol::DownTee  => curses::BTEE,
				Symbol::UpTee    => curses::TTEE,

				Symbol::HorizontalLine => curses::HLINE,
				Symbol::VerticalLine   => curses::VLINE,

				Symbol::Plus         => curses::PLUS,
				Symbol::PlusMinus    => curses::PLMINUS,
				Symbol::LessEqual    => curses::LEQUAL,
				Symbol::GreaterEqual => curses::GEQUAL,
				Symbol::NotEqual     => curses::NEQUAL,

				Symbol::Degree   => curses::DEGREE,
				Symbol::Pi       => curses::PI,
				Symbol::Sterling => curses::STERLING,

				Symbol::Diamond      => curses::DIAMOND,
				Symbol::CheckerBoard => curses::CKBOARD,
				Symbol::Bullet       => curses::BULLET,
				Symbol::Board        => curses::BOARD,
				Symbol::Lantern      => curses::LANTERN,
				Symbol::Block        => curses::BLOCK,
			}))
		}
	}
}
