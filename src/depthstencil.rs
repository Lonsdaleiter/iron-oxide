use crate::{ObjectPointer, handle, Object, MTLCompareFunction};
use crate::import_objc_macros::*;

#[repr(u64)]
/// The option performed on a stored stencil value when a test passes or fails.
///
/// Analogous to [this](https://developer.apple.com/documentation/metal/mtlstenciloperation?language=objc).
pub enum MTLStencilOperation {
    Keep = 0,
    Zero = 1,
    Replace = 2,
    IncrementClamp = 3,
    DecrementClamp = 4,
    Invert = 5,
    IncrementWrap = 6,
    DecrementWrap = 7,
}

/// Describes a stencil face.
///
/// Will send to its pointer only the messages specified in the MTLStencilDescriptor interface
/// linked [here](https://developer.apple.com/documentation/metal/mtlstencildescriptor?language=objc).
pub struct MTLStencilDescriptor(ObjectPointer);
handle!(MTLStencilDescriptor);

impl MTLStencilDescriptor {
    /// Sets the [stencilFailureOperation](https://developer.apple.com/documentation/metal/mtlstencildescriptor/1462471-stencilfailureoperation?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_stencil_fail_operation(&self, operation: MTLStencilOperation) {
        msg_send![self.get_ptr(), setStencilFailureOperation:operation]
    }
    /// Sets the [depthFailureOperation](https://developer.apple.com/documentation/metal/mtlstencildescriptor/1462500-depthfailureoperation?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_depth_fail_operation(&self, operation: MTLStencilOperation) {
        msg_send![self.get_ptr(), setDepthFailureOperation:operation]
    }
    /// Sets the [depthStencilPassOperation](https://developer.apple.com/documentation/metal/mtlstencildescriptor/1462486-depthstencilpassoperation?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_depth_stencil_pass_operation(&self, operation: MTLStencilOperation) {
        msg_send![self.get_ptr(), setDepthStencilPassOperation:operation]
    }
}

impl Object for MTLStencilDescriptor {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self where
        Self: Sized {
        MTLStencilDescriptor(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}

/// Describes a depth stencil state.
///
/// Will send to its pointer only the messages specified in the MTLDepthStencilDescriptor interface
/// linked [here](https://developer.apple.com/documentation/metal/mtldepthstencildescriptor?language=objc).
pub struct MTLDepthStencilDescriptor(ObjectPointer);
handle!(MTLDepthStencilDescriptor);

impl MTLDepthStencilDescriptor {
    /// Creates a new MTLDepthStencilDescriptor with standard allocation and initialization.
    pub unsafe fn new() -> MTLDepthStencilDescriptor {
        MTLDepthStencilDescriptor::from_ptr(msg_send![class!(MTLDepthStencilDescriptor), new])
    }
    /// Sets the [depthCompareFunction](https://developer.apple.com/documentation/metal/mtldepthstencildescriptor/1462463-depthcomparefunction?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_depth_compare_function(&self, function: MTLCompareFunction) {
        msg_send![self.get_ptr(), setDepthCompareFunction:function]
    }
    /// Sets the [depthWriteEnabled](https://developer.apple.com/documentation/metal/mtldepthstencildescriptor/1462501-depthwriteenabled?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_depth_write_enabled(&self, enabled: bool) {
        msg_send![self.get_ptr(), setDepthWriteEnabled:enabled]
    }
}

impl Object for MTLDepthStencilDescriptor {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self where
        Self: Sized {
        MTLDepthStencilDescriptor(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}
