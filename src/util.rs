use crate::{NSUInteger, Object, ObjectPointer};
use std::fmt::{Display, Formatter};

/// Takes an implementor of `Object` and logs some basic information about it:
/// - `retainCount`
/// - `description`
///
/// Assumes that the implementation of `get_ptr` given by `T` unconditionally
/// returns a pointer to a valid Objective-C object inheriting from `NSObject`.
pub unsafe fn debug<T: Object>(obj: &T) {
    use crate::import_objc_macros::*;

    let count: NSUInteger = msg_send![obj.get_ptr(), retainCount];
    let description = ObjectPointer(msg_send![obj.get_ptr(), description]);
    let description = {
        let bytes: *const u8 = msg_send![description, UTF8String];
        let len: NSUInteger = msg_send![description, length];
        let bytes = std::slice::from_raw_parts(bytes, len as usize);
        std::str::from_utf8(bytes).unwrap()
    };

    let level = if count != 1 {
        log::Level::Warn
    } else {
        log::Level::Info
    };
    log::log!(level, "Retain count = {}", count);
    log::log!(log::Level::Info, "Description = {}", description);
}

#[repr(C)]
pub struct MTLSize {
    pub width: NSUInteger,
    pub height: NSUInteger,
    pub depth: NSUInteger,
}

impl Display for MTLSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("W: {}, H: {}, D: {}", self.width, self.height, self.depth).as_str())
    }
}

#[repr(C)]
pub struct MTLSamplePosition {
    pub x: f32,
    pub y: f32,
}

impl Display for MTLSamplePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("X: {}, Y: {}", self.x, self.y).as_str())
    }
}
