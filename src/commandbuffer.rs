use crate::import_objc_macros::*;
use crate::{handle, DeviceCreated, Object, ObjectPointer};

pub struct MTLCommandBuffer(ObjectPointer);
handle!(MTLCommandBuffer);

impl MTLCommandBuffer {
    pub unsafe fn enqueue(&self) {
        msg_send![self.get_ptr(), enqueue]
    }
}

impl Object for MTLCommandBuffer {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLCommandBuffer(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}

impl DeviceCreated for MTLCommandBuffer {}
