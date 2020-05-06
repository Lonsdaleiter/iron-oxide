use crate::import_objc_macros::*;
use crate::{handle, DeviceCreated, Object, ObjectPointer};

/// Contains bound to itself the state of a compute pipeline configured by a device.
///
/// Will sent to its pointer only messages specified in the MTLComputePipelineState
/// protocol linked [here](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate?language=objc).
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
