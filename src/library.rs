use crate::import_macros::*;
use crate::{DeviceCreated, MTLDevice, Object, ObjectPointer, handle};

/// A collection of shader functions.
/// Will send to its pointer only the messages specified in the MTLLibrary protocol
/// linked [here](https://developer.apple.com/documentation/metal/mtllibrary?language=objc).
pub struct MTLLibrary(ObjectPointer);
handle!(MTLLibrary);

impl MTLLibrary {}

impl Object for MTLLibrary {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLLibrary(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}

impl DeviceCreated for MTLLibrary {
    unsafe fn get_device(&self) -> MTLDevice {
        MTLDevice::from_ptr({
            let k = ObjectPointer(msg_send![self.get_ptr(), device]);
            msg_send![k, retain]
        })
    }
}
