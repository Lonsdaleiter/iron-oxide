use crate::import_objc_macros::*;
use crate::{handle, DeviceCreated, MTLDevice, Object, ObjectPointer};

/// Contains bound to itself the state of a compute pipeline configured by a device.
///
/// Will sent to its pointer only messages specified in the MTLComputePipelineState
/// protocol linked [here](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate?language=objc).
pub struct MTLComputePipelineState(ObjectPointer);
handle!(MTLComputePipelineState);

impl DeviceCreated for MTLComputePipelineState {
    unsafe fn get_device(&self) -> MTLDevice {
        MTLDevice::from_ptr({
            let k = ObjectPointer(msg_send![self.get_ptr(), device]);
            msg_send![k, retain]
        })
    }
}

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
