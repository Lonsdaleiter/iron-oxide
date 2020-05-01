use crate::{NSUInteger, Object};
use std::fmt::{Display, Formatter};

/// Takes an implementor of `Object` and logs some information about it.
pub unsafe fn debug<T: Object>(obj: &T) {
    use crate::import_macros::*;

    let count: NSUInteger = msg_send![obj.get_ptr(), retainCount];

    let level = if count != 1 {log::Level::Warn} else {log::Level::Info};
    log::log!(level, "Retain count = {}", count);
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
