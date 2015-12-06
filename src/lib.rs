extern crate libc;
#[macro_use] extern crate bitflags;
extern crate curses_sys as curses;

extern crate unicode_segmentation;
extern crate unicode_width;

pub use curses as sys;

pub type Result<T> = ::std::result::Result<T, Error>;

#[macro_use]
mod util;

mod error;
pub use error::Error;

pub mod color;
pub use color::Color;

pub mod attribute;
pub use attribute::Attributes;

mod key;
pub use key::Key;

mod mouse;
pub use mouse::Mouse;

mod scroll;
pub use scroll::Scroll;

mod event;
pub use event::Event;

mod width;
pub use width::Width;

mod character;
pub use character::Character;

mod symbol;
pub use symbol::Symbol;

pub mod screen;
pub use screen::Screen;

pub mod window;
pub use window::Window;
