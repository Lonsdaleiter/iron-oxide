use crate::import_objc_macros::*;
use crate::{handle, DeviceCreated, Object, ObjectPointer, MTLCommandBuffer};

/// A queue which organizes command buffers to be executed by a GPU.
///
/// Will send to its pointer only the messages specified in the MTLCommandQueue protocol linked
/// [here](https://developer.apple.com/documentation/metal/mtlcommandqueue?language=objc).
pub struct MTLCommandQueue(ObjectPointer);
handle!(MTLCommandQueue);

impl MTLCommandQueue {
    /// Creates a new command buffer using either the
    /// [commandBuffer](https://developer.apple.com/documentation/metal/mtlcommandqueue/1508686-commandbuffer?language=objc)
    /// or [commandBufferWithUnretainedReferences](https://developer.apple.com/documentation/metal/mtlcommandqueue/1508684-commandbufferwithunretainedrefer?language=objc)
    /// methods.
    pub unsafe fn new_command_buffer(&self, retain_references: bool) -> MTLCommandBuffer {
        MTLCommandBuffer::from_ptr(match retain_references{
            true => msg_send![self.get_ptr(), commandBuffer],
            false => msg_send![self.get_ptr(), commandBufferWithUnretainedReferences],
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
