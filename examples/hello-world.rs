extern crate curses;
use curses::Color;

fn main() {
	let mut screen = curses::screen::init().unwrap();

	if screen.colors().available() && screen.colors().limit() >= 7 {
		screen.colors().define(1, (Color::Red,     Default::default())).unwrap();
		screen.colors().define(2, (Color::Green,   Default::default())).unwrap();
		screen.colors().define(3, (Color::Yellow,  Default::default())).unwrap();
		screen.colors().define(4, (Color::Blue,    Default::default())).unwrap();
		screen.colors().define(5, (Color::Magenta, Default::default())).unwrap();
		screen.colors().define(6, (Color::Cyan,    Default::default())).unwrap();
		screen.colors().define(7, (Color::White,   Default::default())).unwrap();
	}

	let desc = screen.description();
	let x    = (screen.columns() - desc.len()) / 2;
	let y    = (screen.rows() - 7) / 2;

	for c in 1 .. 8 {
		screen
			.colors().set(c).unwrap()
			.position(x, y + c).unwrap()
			.write(desc).unwrap();
	}

	screen.refresh().unwrap();
	screen.input().character();
}
