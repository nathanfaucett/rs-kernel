#![feature(lang_items)]
#![no_std]


#[macro_use]
extern crate errno;

#[macro_use]
extern crate memory_area;


mod error;
mod kernel;


pub use error::{Error, ErrorCode};
pub use kernel::Kernel;
