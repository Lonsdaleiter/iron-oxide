mod pass;
mod render;
pub use pass::*;
pub use render::*;

use crate::import_objc_macros::*;
use crate::{DeviceCreated, Object};

pub trait MTLCommandEncoder: Object {
    unsafe fn end_encoding(&self) {
        msg_send![self.get_ptr(), endEncoding]
    }
}
