use crate::import_objc_macros::*;
use crate::{handle, DeviceCreated, Object, ObjectPointer};

pub struct MTLComputePipelineState(ObjectPointer);
handle!(MTLComputePipelineState);

impl DeviceCreated for MTLComputePipelineState {}

impl Object for MTLComputePipelineState {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLComputePipelineState(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}
