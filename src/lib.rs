//! `iron-oxide` provides unsafe [Metal](https://developer.apple.com/documentation/metal?language=objc) bindings for Rust.

use objc::Message;
use std::ops::Deref;

mod device;
mod util;
pub use device::*;
pub use util::*;

mod import_macros {
    pub use objc::{class, msg_send, sel, sel_impl};
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjectPointer(pub *mut objc::runtime::Object);
impl Deref for ObjectPointer {
    type Target = objc::runtime::Object;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl Message for ObjectPointer {}

/// Represents an Objective C object.
///
/// # Requirements
///
/// There *must* be for an implementation of Object an implementation of Clone and Drop using
/// the [handle!] macro. See [handle!] for more information about what these implementations do.
pub trait Object: Clone + Drop {
    /// Constructs an object from the provided pointer.
    ///
    /// # Safety
    ///
    /// The pointer provided *must* be a valid pointer to an Objective C object which can
    /// accept the messages which the used implementation of Object will send.
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self where Self: Sized;
    /// Returns the underlying pointer.
    ///
    /// # Requirements
    ///
    /// Must be a valid pointer to an Objective C object. If it is not, the [handle!] implementation
    /// will cause a crash on drop or clone.
    fn get_ptr(&self) -> ObjectPointer;
}

/// Aliased exclusively so that, should the watchOS target be added to Rust, NSUInteger can handle
/// watchOS' 32-bit architecture.
pub type NSUInteger = u64;

#[macro_export]
/// Provides an implementation of `Clone` and `Drop` for a struct implementing `Object`.
///
/// The `Clone` implementation creates a new of the struct with the same pointer and an
/// increments the object's reference count. The `Drop` implementation decrements the object's
/// reference count.
///
/// These implementations together serve to make the struct on which this is called be a reference
/// to the underlying object. When the last struct with a reference to this object is dropped,
/// the object is garbage collected.
macro_rules! handle {
    ($name:ident) => {
        impl Clone for $name {
            fn clone(&self) -> $name {
                use crate::import_macros::*;
                unsafe { $name::from_ptr(msg_send![self.get_ptr(), retain]) }
            }
        }
        impl Drop for $name {
            fn drop(&mut self) {
                use crate::import_macros::*;
                unsafe {
                    let _: () = msg_send![self.get_ptr(), release];
                }
            }
        }
    };
}
