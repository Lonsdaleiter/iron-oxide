use crate::{handle, MTLCommandEncoder, Object, ObjectPointer};

pub struct MTLComputeCommandEncoder(ObjectPointer);
handle!(MTLComputeCommandEncoder);

impl MTLComputeCommandEncoder {
    // TODO add my behavior
}

impl MTLCommandEncoder for MTLComputeCommandEncoder {}

impl Object for MTLComputeCommandEncoder {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLComputeCommandEncoder(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}
