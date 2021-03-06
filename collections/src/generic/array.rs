/*!
# Module zstd::collections::generic::array

---
*/

use core::fmt;
use core::fmt::{Formatter, Error};
use alloc::boxed::Box;

/// # Struct Array<T>
///
/// ```
/// use collections::generic::Array;
/// pub fn array() -> Array<char> {
///   let x = Array::<char>::new(['h', 'e', 'l', 'l', 'o']);
///
///   return *x;
/// }
/// ```
pub struct Array<T> {
  arr: [T],
}

impl<T> Array<T> {
}

impl<T> fmt::Debug for Array<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
  }
}

mod specialized {
  /*!
  # Module zstd::collections::generic::array::specialized

  ---
  */
}
