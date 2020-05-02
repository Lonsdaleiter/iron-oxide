use crate::{ObjectPointer, handle, Object, DeviceCreated, MTLDevice, NSUIntegerRange, NSRange, NSUInteger};
use crate::import_objc_macros::*;
use std::os::raw::c_void;

/// A resource which stores data.
///
/// Will send to its pointer only the messages specified in the MTLBuffer protocol
/// linked [here](https://developer.apple.com/documentation/metal/mtlbuffer?language=objc).
pub struct MTLBuffer(ObjectPointer);
handle!(MTLBuffer);

impl MTLBuffer {
    /// Returns the address of the buffer's storage allocation via the
    /// [contents](https://developer.apple.com/documentation/metal/mtlbuffer/1515716-contents?language=objc)
    /// instance method.
    ///
    /// The returned pointer may be `null` is the buffer's storage allocation is in VRAM.
    pub unsafe fn get_contents(&self) -> *mut c_void {
        msg_send![self.get_ptr(), contents]
    }
    /// Informs the GPU that the CPU has modified a range of the buffer via the
    /// [didModifyRange](https://developer.apple.com/documentation/metal/mtlbuffer/1516121-didmodifyrange?language=objc)
    /// instance method.
    pub unsafe fn did_modify_range(&self, range: NSUIntegerRange) {
        let range = NSRange {
            location: range.start,
            length: range.end,
        };
        msg_send![self.get_ptr(), range]
    }
    /// Returns the instance property [length](https://developer.apple.com/documentation/metal/mtlbuffer/1515373-length?language=objc),
    /// representing the logical size of the buffer in bytes.
    pub unsafe fn get_length(&self) -> NSUInteger {
        msg_send![self.get_ptr(), length]
    }
}

impl DeviceCreated for MTLBuffer {
    unsafe fn get_device(&self) -> MTLDevice {
        MTLDevice(msg_send![self.get_ptr(), device])
    }
}

impl Object for MTLBuffer {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self where
        Self: Sized {
        MTLBuffer(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}
