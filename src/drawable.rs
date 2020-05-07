use crate::import_objc_macros::*;
use crate::{handle, CAMetalLayer, MTLTexture, NSUInteger, Object, ObjectPointer};

pub trait MTLDrawable: Object {
    unsafe fn get_id(&self) -> NSUInteger {
        msg_send![self.get_ptr(), drawableID]
    }
    unsafe fn present(&self) {
        msg_send![self.get_ptr(), present]
    }
    unsafe fn present_after_minimum_duration(&self, duration: f64) {
        msg_send![self.get_ptr(), presentAfterMinimumDuration: duration]
    }
    unsafe fn present_at_time(&self, time: f64) {
        msg_send![self.get_ptr(), presentAtTime: time]
    }
    unsafe fn get_presented_time(&self) -> f64 {
        msg_send![self.get_ptr(), presentedTime]
    }
}

pub struct CAMetalDrawable(ObjectPointer);
handle!(CAMetalDrawable);

impl CAMetalDrawable {
    pub unsafe fn get_texture(&self) -> MTLTexture {
        MTLTexture::from_ptr(msg_send![self.get_ptr(), texture])
    }
    pub unsafe fn get_layer(&self) -> CAMetalLayer {
        CAMetalLayer::from_ptr(msg_send![self.get_ptr(), layer])
    }
}

impl MTLDrawable for CAMetalDrawable {}

impl Object for CAMetalDrawable {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        CAMetalDrawable(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}
