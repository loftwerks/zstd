/*!
# Module zcore::any

---
*/

///////////////////////////////////////////////////////////////////////////////
// TypeID and its methods
///////////////////////////////////////////////////////////////////////////////

use core::intrinsics;

/// A `TypeId` represents a globally unique identifier for a type.
///
/// Each `TypeId` is an opaque object which does not allow inspection of what's
/// inside but does allow basic operations such as cloning, comparison,
/// printing, and showing.
///
/// A `TypeId` is currently only available for types which ascribe to `'static`,
/// but this limitation may be removed in the future.
///
/// While `TypeId` implements `Hash`, `PartialOrd`, and `Ord`, it is worth
/// noting that the hashes and ordering will vary between Rust releases. Beware
/// of relying on them inside of your code!
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct TypeId {
  t: u64,
}

impl TypeId {
  /// Returns the `TypeId` of the type this generic function has been
/// instantiated with.
///
/// # Examples
///
/// ```
/// use std::any::{Any, TypeId};
///
/// fn is_string<T: ?Sized + Any>(_s: &T) -> bool {
///     TypeId::of::<String>() == TypeId::of::<T>()
/// }
///
/// assert_eq!(is_string(&0), false);
/// assert_eq!(is_string(&"cookie monster".to_string()), true);
/// ```
  #[cfg_attr(bootstrap, rustc_const_unstable(feature = "const_type_id"))]
  #[cfg_attr(not(bootstrap), rustc_const_unstable(feature = "const_type_id", issue = "41875"))]
  pub const fn of<T: ?Sized + 'static>() -> TypeId {
    TypeId {
      #[cfg(bootstrap)]
      t: unsafe { intrinsics::type_id::<T>() },
    }
  }
}
