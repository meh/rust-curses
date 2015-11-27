extern crate libc;
#[macro_use] extern crate bitflags;
extern crate curses_sys as curses;

pub use curses as sys;

mod error;
pub use error::Error;

pub mod color;
pub use color::Color;

pub mod attribute;
pub use attribute::Attributes;

pub mod screen;
pub use screen::Screen;

mod window;
pub use window::Window;
