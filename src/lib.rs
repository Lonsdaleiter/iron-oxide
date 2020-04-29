use objc::Message;
use std::ops::Deref;

mod device;
pub use device::*;

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

pub trait Object {
    #[doc(hidden)]
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self;
    /// Returns the underlying pointer.
    fn get_ptr(&self) -> ObjectPointer;
}

pub type NSUInteger = u64;

#[macro_export]
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
