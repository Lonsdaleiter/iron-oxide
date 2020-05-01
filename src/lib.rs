//! `iron-oxide` provides unsafe [Metal](https://developer.apple.com/documentation/metal?language=objc)
//! bindings for Rust.
//!
//! Not all of Metal's functionality is added. The pointer underlying a MTL(something) can
//! be accessed with `get_ptr`, and messages can be sent to it with `objc`'s `msg_send!`, if
//! necessary functionality isn't yet implemented. This is very unsafe.
//!
//! It is the responsibility of the user to not use methods or functions which do not exist in
//! OS versions below what they support (specified in the linked Metal docs).

use log::Level;
use objc::Message;
use std::ops::Deref;

mod commandqueue;
mod device;
mod library;
mod util;
pub use commandqueue::*;
pub use device::*;
pub use library::*;
pub use util::*;

pub mod import_macros {
    pub use objc::{class, msg_send, sel, sel_impl};
}

/// Represents either an error, a `T`, or a warning and a `T`.
///
/// When there is a Metal function or method which takes a pointer to an error,
/// the bound Rust function or method will return an `Error`.
pub enum Error<'a, T> {
    /// The operation succeeded.
    None(T),
    /// The operation succeeded but a warning was produced.
    Warn(T, &'a str),
    /// The operation failed.
    Error(&'a str),
}

impl<'a, T> Error<'a, T> {
    pub fn unwrap(self) -> T {
        match self {
            Error::None(obj) => obj,
            Error::Warn(obj, msg) => {
                log::log!(Level::Warn, "{}", msg);
                obj
            }
            Error::Error(msg) => panic!("{}", msg),
        }
    }
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
/// the `handle!` macro. See `handle!` for more information about what these implementations do.
pub trait Object: Clone + Drop {
    /// Constructs an object from the provided pointer.
    ///
    /// The pointer provided *must* be a valid pointer to an Objective C object which can
    /// accept the messages which the used implementation of Object will send.
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized;
    /// Returns the underlying pointer.
    ///
    /// The returned pointer *must* be a valid pointer to an Objective C object.
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
