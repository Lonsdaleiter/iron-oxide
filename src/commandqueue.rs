use crate::import_macros::*;
use crate::{handle, DeviceCreated, MTLDevice, Object, ObjectPointer};

/// A queue which organizes command buffers to be executed by a GPU.
///
/// Will send to its pointer only the messages specified in the MTLCommandQueue protocol linked
/// [here](https://developer.apple.com/documentation/metal/mtlcommandqueue?language=objc).
pub struct MTLCommandQueue(ObjectPointer);
handle!(MTLCommandQueue);

impl Object for MTLCommandQueue {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLCommandQueue(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}

impl DeviceCreated for MTLCommandQueue {
    unsafe fn get_device(&self) -> MTLDevice {
        MTLDevice::from_ptr({
            let k = ObjectPointer(msg_send![self.get_ptr(), device]);
            msg_send![k, retain]
        })
    }
}
