use crate::NSUInteger;
use std::fmt::{Display, Formatter};

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
