mod render;
pub use render::*;

use crate::{Object, DeviceCreated};
use crate::import_objc_macros::*;

pub trait MTLCommandEncoder: Object + DeviceCreated {
    unsafe fn end_encoding(&self) {
        msg_send![self.get_ptr(), endEncoding]
    }
}
