extern crate curses;
use curses::{Event, Key};

fn main() {
	let mut screen = curses::screen::init().unwrap();

	screen.capabilities()
		.echo(false).unwrap()
		.keypad(true).unwrap()
		.raw(true).unwrap();

	let mut old = None;

	loop {
		let event = screen.input().event();

		screen
			.erase().unwrap()
			.cursor(0, 0).unwrap()
			.add().string(format!("{:?}", event)).unwrap();

		if let (Some(Event::Key(Key::Char('Z'))), Some(Event::Key(Key::Char('Z')))) = (event, old) {
			break;
		}

		old = event;
	}
}
