#![crate_name = "zcore"]
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

pub mod any;
pub mod collections;
pub mod eco;
pub mod kernel;

pub(crate) mod console;
pub(crate) mod vga_buffer;

mod panic;
mod prelude;
