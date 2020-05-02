use crate::{ObjectPointer, handle, Object, DeviceCreated, MTLDevice};
use crate::import_objc_macros::*;

/// A resource which stores data.
///
/// Will send to its pointer only the messages specified in the MTLBuffer protocol
/// linked [here](https://developer.apple.com/documentation/metal/mtlbuffer?language=objc).
pub struct MTLBuffer(ObjectPointer);
handle!(MTLBuffer);

impl MTLBuffer {
    //
}

impl DeviceCreated for MTLBuffer {
    unsafe fn get_device(&self) -> MTLDevice {
        MTLDevice(msg_send![self.get_ptr(), device])
    }
}

impl Object for MTLBuffer {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self where
        Self: Sized {
        MTLBuffer(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}
