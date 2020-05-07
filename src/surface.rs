use crate::{ObjectPointer, handle, Object, NSUInteger};

pub struct MTLDrawable(ObjectPointer);
handle!(MTLDrawable);

impl MTLDrawable {
    pub unsafe fn get_id(&self) -> NSUInteger {
        msg_send![self.get_ptr(), drawableID]
    }
    pub unsafe fn present(&self) {
        msg_send![self.get_ptr(), present]
    }
    pub unsafe fn present_after_minimum_duration(&self, duration: f64) {
        msg_send![self.get_ptr(), presentAfterMinimumDuration:duration]
    }
    pub unsafe fn present_at_time(&self, time: f64) {
        msg_send![self.get_ptr(), presentAtTime:time]
    }
    pub unsafe fn get_presented_time(&self) -> f64 {
        msg_send![self.get_ptr(), presentedTime]
    }
}

impl Object for MTLDrawable {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self where
        Self: Sized {
        MTLDrawable(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}
