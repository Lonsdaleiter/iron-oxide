use crate::import_objc_macros::*;
use crate::{handle, DeviceCreated, MTLCommandBuffer, Object, ObjectPointer};

pub struct MTLCommandQueue(ObjectPointer);
handle!(MTLCommandQueue);

impl MTLCommandQueue {
    pub unsafe fn new_command_buffer(&self, retain_references: bool) -> MTLCommandBuffer {
        MTLCommandBuffer::from_ptr({
            let pointer = ObjectPointer(match retain_references {
                true => msg_send![self.get_ptr(), commandBuffer],
                false => msg_send![self.get_ptr(), commandBufferWithUnretainedReferences],
            });
            msg_send![pointer, retain]
        })
    }
}

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

impl DeviceCreated for MTLCommandQueue {}
