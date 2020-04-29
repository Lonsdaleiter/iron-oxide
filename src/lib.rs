use objc::Message;

mod import_macros {
    pub use objc::{sel, sel_impl, msg_send, class};
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjectPointer(pub *mut objc::runtime::Object);
impl Deref for ObjectPointer {
    type Target = objc::runtime::Object;

    fn deref(&self) -> &Self::Target {
        unsafe {&*self.0}
    }
}
unsafe impl Message for ObjectPointer {}

pub trait Object {
    #[doc(hidden)]
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self;
    /// Returns a pointer to an Objective-C object.
    ///
    /// Messages specified by the
    fn get_ptr(&self) -> ObjectPointer;
}

macro_rules! handle {
    ($name:ident) => {
        impl Clone for $name {
            fn clone(&self) -> $name {
                use crate::import_macros::*;
                unsafe {
                    $name::from_ptr(msg_send![self.get_ptr(), retain])
                }
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

mod device;
pub use device::*;
use std::ops::Deref;
