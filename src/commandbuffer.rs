use crate::import_objc_macros::*;
use crate::{handle, DeviceCreated, MTLDrawable, Object, ObjectPointer, NSError};

#[repr(u64)]
pub enum MTLCommandBufferStatus {
    NotEnqueued = 0,
    Enqueued = 1,
    Committed = 2,
    Scheduled = 3,
    Completed = 4,
    Error = 5,
}

pub struct MTLCommandBuffer(ObjectPointer);
handle!(MTLCommandBuffer);

impl MTLCommandBuffer {
    pub unsafe fn enqueue(&self) {
        msg_send![self.get_ptr(), enqueue]
    }
    pub unsafe fn commit(&self) {
        msg_send![self.get_ptr(), commit]
    }
    pub unsafe fn wait_until_scheduled(&self) {
        msg_send![self.get_ptr(), waitUntilScheduled]
    }
    pub unsafe fn wait_until_completed(&self) {
        msg_send![self.get_ptr(), waitUntilCompleted]
    }
    pub unsafe fn present_drawable<T: MTLDrawable>(&self, drawable: T) {
        msg_send![self.get_ptr(), presentDrawable:drawable.get_ptr()]
    }
    pub unsafe fn present_drawable_after_min_duration<T: MTLDrawable>(
        &self,
        drawable: T,
        duration: f64,
    ) {
        msg_send![self.get_ptr(), presentDrawable:drawable.get_ptr() afterMinimumDuration:duration]
    }
    pub unsafe fn present_drawable_at_time<T: MTLDrawable>(&self, drawable: T, time: f64) {
        msg_send![self.get_ptr(), presentDrawable:drawable.get_ptr() atTime:time]
    }
    pub unsafe fn get_status(&self) -> MTLCommandBufferStatus {
        msg_send![self.get_ptr(), status]
    }
    pub unsafe fn get_error(&self) -> Option<NSError> {
        let err = ObjectPointer(msg_send![self.get_ptr(), error]);
        if err.0.is_null() {
            None
        } else {
            Some(NSError::from_ptr(err))
        }
    }
    pub unsafe fn get_kernel_start_time(&self) -> f64 {
        msg_send![self.get_ptr(), kernelStartTime]
    }
    pub unsafe fn get_kernel_end_time(&self) -> f64 {
        msg_send![self.get_ptr(), kernelEndTime]
    }
}

impl Object for MTLCommandBuffer {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLCommandBuffer(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}

impl DeviceCreated for MTLCommandBuffer {}
