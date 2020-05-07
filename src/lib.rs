//! `iron-oxide` provides unsafe [Metal](https://developer.apple.com/documentation/metal?language=objc)
//! bindings for Rust.
//!
//! # Metal Functionality
//!
//! Not all of Metal's functionality is added. The pointer underlying a MTL(something) can
//! be accessed with `get_ptr`, and messages can be sent to it with `objc`'s `msg_send!`, if
//! necessary functionality isn't yet implemented. This is very unsafe.
//!
//! # Metal Docs
//!
//! It is the responsibility of the user to not use methods or functions which do not exist in
//! OS versions below what they support. macOS, iOS, tvOS, and watchOS only! They can be checked
//! in the Metal docs corresponding with a given method or function.
//!
//! # Examples
//!
//! See the examples directory for examples.
//!
//! # License
//!
//! Licensed under the MIT license.

use log::Level;
use objc::Message;
use std::ops::Deref;

mod commandbuffer;
mod commandqueue;
mod depthstencil;
mod device;
mod drawable;
mod library;
mod misc;
mod pipeline;
mod resource;
mod sampler;
mod surface;
pub use commandbuffer::*;
pub use commandqueue::*;
pub use depthstencil::*;
pub use device::*;
pub use drawable::*;
pub use library::*;
pub use misc::*;
pub use pipeline::*;
pub use resource::*;
pub use sampler::*;
pub use surface::*;

pub mod import_objc_macros {
    pub use objc::{class, msg_send, sel, sel_impl};
}

// TODO turn this into an NSError and give it behavior
pub enum MetalError<'a, T> {
    None(T),
    Warn(T, &'a str),
    Error(&'a str),
}

impl<'a, T> MetalError<'a, T> {
    pub fn unwrap(self) -> T {
        match self {
            MetalError::None(obj) => obj,
            MetalError::Warn(obj, msg) => {
                log::log!(Level::Warn, "{}", msg);
                obj
            }
            MetalError::Error(msg) => panic!("{}", msg),
        }
    }
}

#[repr(C)]
#[doc(hidden)]
pub struct ObjectPointerMarker {
    // ignore me
    _member: u8,
}
unsafe impl Message for ObjectPointerMarker {}

#[derive(Copy, Clone)]
#[repr(C)]
/// A messageable pointer to a (presumed) Objective C object.
pub struct ObjectPointer(*mut ObjectPointerMarker);
impl Deref for ObjectPointer {
    type Target = ObjectPointerMarker;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}
unsafe impl Message for ObjectPointer {}

pub trait Array<T: Object>: Object {
    unsafe fn set_object_at_indexed_subscript(&self, index: NSUInteger, obj: &T) {
        use crate::import_objc_macros::*;
        msg_send![self.get_ptr(), setObject:obj.get_ptr() atIndexedSubscript:index]
    }
}

/// Represents an Objective C object.
///
/// # Requirements
///
/// There *must* be for an implementation of Object an implementation of Drop using
/// the `handle!` macro. See `handle!` for more information and an example.
/// ```
pub trait Object: Drop {
    /// Constructs an object from the provided pointer.
    ///
    /// The pointer provided *must* be a valid pointer to an Objective C object which can
    /// accept the messages which the used implementation of Object will send.
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized;
    /// Returns the underlying pointer of the object.
    ///
    /// The returned pointer *must* be a valid pointer to an Objective C object.
    fn get_ptr(&self) -> ObjectPointer;
}

/// Aliased exclusively so that, should the watchOS target be added to Rust, NSInteger can
/// conform to watchOS' 32-bit architecture.
pub type NSInteger = i64;
/// Aliased exclusively so that, should the watchOS target be added to Rust, NSUInteger can
/// conform to watchOS' 32-bit architecture.
pub type NSUInteger = u64;

#[macro_export]
/// Provides an implementation of `Drop` which implements lifetime-based releasing
/// on the implemented type's pointer to an Objective C object.
///
/// This implementation of `Drop` decrements the reference count of the object which the
/// `get_ptr` method returns. This ensures that the object to which the implementor points
/// lives only for the lifetime of the implementor.
///
/// # Requirements
///
/// The ident passed into `handle!` must be the correct local name of a struct or enum which
/// implements `Object`.
///
/// # Example
///
/// ```
/// use iron_oxide::{ObjectPointer, handle, Object};
///
/// struct Wrapper(ObjectPointer);
/// handle!(Wrapper);
///
/// impl Object for Wrapper {
///     unsafe fn from_ptr(ptr: ObjectPointer) -> Self where
///         Self: Sized {
///         Wrapper(ptr)
///     }
///
///     fn get_ptr(&self) -> ObjectPointer {
///         self.0
///     }
///
/// }
/// ```
macro_rules! handle {
    ($name:ident) => {
        impl Drop for $name {
            fn drop(&mut self) {
                use crate::import_objc_macros::*;
                unsafe {
                    let _: () = msg_send![self.get_ptr(), release];
                }
            }
        }
    };
}
