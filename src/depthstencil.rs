use crate::import_objc_macros::*;
use crate::{handle, DeviceCreated, MTLCompareFunction, Object, ObjectPointer};

#[repr(u64)]
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

pub struct MTLStencilDescriptor(ObjectPointer);
handle!(MTLStencilDescriptor);

impl MTLStencilDescriptor {
    pub unsafe fn new() -> MTLStencilDescriptor {
        MTLStencilDescriptor::from_ptr(msg_send![class!(MTLStencilDescriptor), new])
    }
    pub unsafe fn set_stencil_fail_operation(&self, operation: MTLStencilOperation) {
        msg_send![self.get_ptr(), setStencilFailureOperation: operation]
    }
    pub unsafe fn set_depth_fail_operation(&self, operation: MTLStencilOperation) {
        msg_send![self.get_ptr(), setDepthFailureOperation: operation]
    }
    pub unsafe fn set_depth_stencil_pass_operation(&self, operation: MTLStencilOperation) {
        msg_send![self.get_ptr(), setDepthStencilPassOperation: operation]
    }
    pub unsafe fn set_read_mask(&self, mask: u32) {
        msg_send![self.get_ptr(), setReadMask: mask]
    }
    pub unsafe fn set_write_mask(&self, mask: u32) {
        msg_send![self.get_ptr(), setWriteMask: mask]
    }
}

impl Object for MTLStencilDescriptor {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLStencilDescriptor(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}

pub struct MTLDepthStencilDescriptor(ObjectPointer);
handle!(MTLDepthStencilDescriptor);

impl MTLDepthStencilDescriptor {
    pub unsafe fn new() -> MTLDepthStencilDescriptor {
        MTLDepthStencilDescriptor::from_ptr(msg_send![class!(MTLDepthStencilDescriptor), new])
    }
    pub unsafe fn set_depth_compare_function(&self, function: MTLCompareFunction) {
        msg_send![self.get_ptr(), setDepthCompareFunction: function]
    }
    pub unsafe fn set_depth_write_enabled(&self, enabled: bool) {
        msg_send![self.get_ptr(), setDepthWriteEnabled: enabled]
    }
    pub unsafe fn set_back_face_stencil(&self, stencil: &MTLStencilDescriptor) {
        msg_send![self.get_ptr(), setBackFaceStencil:stencil.get_ptr()]
    }
    pub unsafe fn set_front_face_stencil(&self, stencil: &MTLStencilDescriptor) {
        msg_send![self.get_ptr(), setFrontFaceStencil:stencil.get_ptr()]
    }
}

impl Object for MTLDepthStencilDescriptor {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLDepthStencilDescriptor(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}

pub struct MTLDepthStencilState(ObjectPointer);
handle!(MTLDepthStencilState);

impl Object for MTLDepthStencilState {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLDepthStencilState(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}

impl DeviceCreated for MTLDepthStencilState {}
