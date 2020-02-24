#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature="const_fn", feature(const_fn))]
#![cfg_attr(feature="const_fn", feature(const_in_array_repeat_expressions))]
#![cfg_attr(feature="inline_asm", feature(asm))]
#![cfg_attr(feature="abi_x86_interrupt", feature(abi_x86_interrupt))]
#![cfg_attr(feature="deny-warnings", deny(warnings))]
#![cfg_attr(feature="deny-warnings", deny(missing_docs))]
#![cfg_attr(not(feature="deny-warnings"), warn(missing_docs))]
#![deny(missing_debug_implementations)]
/*!
# Crate zstd
*A lightweight alternative to the Rust standard library*

---
*/

/// Global exports
/// Crate prelude
pub use prelude::*;

/// Public modules
pub mod any;
pub mod colour;
pub mod console;
pub mod buffer;

/// Private modules
mod panic;
mod prelude;
