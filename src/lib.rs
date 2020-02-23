#![crate_name = "zstd"]
#![warn(missing_docs)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(clippy::needless_return)]
#![no_std]
/*!
# Crate zstd
*A lightweight alternative to the Rust standard library*

---
*/

pub use prelude::*;

// External crates
pub extern crate collections;
pub extern crate eco;

// Public modules
pub mod any;
pub mod console;
pub mod vga;

// Private modules
mod panic;
mod prelude;
