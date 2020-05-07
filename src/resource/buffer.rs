use crate::import_objc_macros::*;
use crate::{
    handle, DeviceCreated, MTLResource, NSRange, NSUInteger, NSUIntegerRange, Object, ObjectPointer,
};
use std::os::raw::c_void;

pub struct MTLBuffer(ObjectPointer);
handle!(MTLBuffer);

impl MTLBuffer {
    pub unsafe fn get_contents(&self) -> *mut c_void {
        msg_send![self.get_ptr(), contents]
    }
    pub unsafe fn did_modify_range(&self, range: NSUIntegerRange) {
        let range = NSRange {
            location: range.start,
            length: range.end,
        };
        msg_send![self.get_ptr(), didModifyRange: range]
    }
    pub unsafe fn get_length(&self) -> NSUInteger {
        msg_send![self.get_ptr(), length]
    }
}

impl MTLResource for MTLBuffer {}

impl DeviceCreated for MTLBuffer {}

impl Object for MTLBuffer {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLBuffer(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}
