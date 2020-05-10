use crate::{handle, Object, ObjectPointer};

pub struct MTLRenderPassDescriptor(ObjectPointer);
handle!(MTLRenderPassDescriptor);

// TODO add my behavior

impl Object for MTLRenderPassDescriptor {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLRenderPassDescriptor(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}
