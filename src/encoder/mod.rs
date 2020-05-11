mod compute;
mod pass;
mod render;
pub use compute::*;
pub use pass::*;
pub use render::*;

use crate::import_objc_macros::*;
use crate::Object;

pub trait MTLCommandEncoder: Object {
    unsafe fn end_encoding(self)
    where
        Self: std::marker::Sized,
    {
        msg_send![self.get_ptr(), endEncoding]
    }
}
