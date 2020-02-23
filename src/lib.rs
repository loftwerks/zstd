#![crate_name = "zstd"]
#![warn(missing_docs)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(clippy::needless_return)]
#![no_std]
/*!
# Zesty - Core

---
*/

pub use prelude::*;

// External crates
pub extern crate collections;
pub extern crate eco;

// Public modules
pub mod any;
pub mod kernel;
pub mod console;

// Internal modules
pub(crate) mod vga_buffer;

// Private modules
mod panic;
mod prelude;
