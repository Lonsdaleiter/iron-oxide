use crate::import_objc_macros::*;
use crate::{handle, DeviceCreated, Object, ObjectPointer};

pub struct MTLCommandBuffer(ObjectPointer);
handle!(MTLCommandBuffer);

impl MTLCommandBuffer {
    pub unsafe fn enqueue(&self) {
        msg_send![self.get_ptr(), enqueue]
    }
    pub unsafe fn commit(&self) {
        msg_send![self.get_ptr(), commit]
    }
    pub unsafe fn wait_until_scheduled(&self) {
        msg_send![self.get_ptr(), waitUntilScheduled]
    }
    pub unsafe fn wait_until_completed(&self) {
        msg_send![self.get_ptr(), waitUntilCompleted]
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
