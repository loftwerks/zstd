#![no_std]
#![cfg_attr(feature = "const_fn", feature(const_fn))]
#![cfg_attr(feature = "const_fn", feature(const_in_array_repeat_expressions))]
#![cfg_attr(feature = "inline_asm", feature(asm))]
#![cfg_attr(feature = "abi_x86_interrupt", feature(abi_x86_interrupt))]
#![cfg_attr(feature = "deny-warnings", deny(warnings))]
#![cfg_attr(feature = "deny-warnings", deny(missing_docs))]
#![cfg_attr(not(feature = "deny-warnings"), warn(missing_docs))]
#![deny(missing_debug_implementations)]
/*!
# Crate alloc

---
*/

extern crate collections;

#[cfg(not(test))]
pub mod borrow;
pub mod string;

#[cfg(test)]
mod borrow;
