/*!
# Zesty - Console

---
*/

use crate::buffer::{
  BUFFER_HEIGHT,
  BUFFER_WIDTH,
  DEFAULT_WRITER,
};

/// Console structure
#[repr(C)]
pub struct Console;

impl Console {
  /// Write to buffer
  pub fn write(o: &'static str) {
    DEFAULT_WRITER.lock().write_string(o);
  }

  pub fn read_key() {}

  pub fn read_line() {}

  pub fn clear() {}
}
