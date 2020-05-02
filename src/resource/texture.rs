use crate::import_objc_macros::*;
use crate::{handle, DeviceCreated, MTLDevice, Object, ObjectPointer};

/// A resource which stores data.
///
/// Will send to its pointer only the messages specified in the MTLTexture protocol
/// linked [here](https://developer.apple.com/documentation/metal/mtltexture?language=objc).
pub struct MTLTexture(ObjectPointer);
handle!(MTLTexture);

impl MTLTexture {
    //
}

impl DeviceCreated for MTLTexture {
    unsafe fn get_device(&self) -> MTLDevice {
        MTLDevice::from_ptr(msg_send![self.get_ptr(), device])
    }
}

impl Object for MTLTexture {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLTexture(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}
