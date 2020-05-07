use crate::import_objc_macros::*;
use crate::{handle, DeviceCreated, Object, ObjectPointer};

/// Stores commands for the GPU to execute.
///
/// Will send to its pointer only the messages specified in the MTLCommandBuffer
/// protocol linked [here](https://developer.apple.com/documentation/metal/mtlcommandbuffer?language=objc).
pub struct MTLCommandBuffer(ObjectPointer);
handle!(MTLCommandBuffer);

impl MTLCommandBuffer {
    //
}

impl Object for MTLCommandBuffer {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self where
        Self: Sized {
        MTLCommandBuffer(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}

impl DeviceCreated for MTLCommandBuffer {}
