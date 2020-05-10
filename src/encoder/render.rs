use crate::{handle, Object, ObjectPointer, MTLCommandEncoder};

pub struct MTLRenderCommandEncoder(ObjectPointer);
handle!(MTLRenderCommandEncoder);

impl MTLRenderCommandEncoder {
    //
}

impl MTLCommandEncoder for MTLRenderCommandEncoder {}

impl Object for MTLRenderCommandEncoder {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLRenderCommandEncoder(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}
