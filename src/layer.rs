use crate::import_objc_macros::*;
use crate::{
    handle, CAMetalDrawable, CGSize, MTLDevice, MTLPixelFormat, Object, ObjectPointer,
    ObjectPointerMarker,
};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

pub trait CreateCAMetalLayer {
    unsafe fn set_layer(&self, layer: &CAMetalLayer);
}

impl<T: HasRawWindowHandle> CreateCAMetalLayer for T {
    unsafe fn set_layer(&self, layer: &CAMetalLayer) {
        let ns_view = match self.raw_window_handle() {
            RawWindowHandle::MacOS(handle) => {
                ObjectPointer(handle.ns_view as *mut ObjectPointerMarker)
            }
            _ => unimplemented!(
                "CAMetalLayers can only be set for macOS RawWindowHandles right now."
            ),
        };

        msg_send![ns_view, setLayer:layer.get_ptr()]
    }
}

pub struct CAMetalLayer(ObjectPointer);
handle!(CAMetalLayer);

impl CAMetalLayer {
    pub unsafe fn new() -> CAMetalLayer {
        CAMetalLayer::from_ptr(msg_send![class!(CAMetalLayer), new])
    }
    pub unsafe fn set_device(&self, device: &MTLDevice) {
        msg_send![self.get_ptr(), setDevice:device.get_ptr()]
    }
    pub unsafe fn set_pixel_format(&self, format: MTLPixelFormat) {
        msg_send![self.get_ptr(), setPixelFormat: format]
    }
    pub unsafe fn set_framebuffer_only(&self, only: bool) {
        msg_send![self.get_ptr(), setFramebufferOnly: only]
    }
    pub unsafe fn set_drawable_size(&self, size: CGSize) {
        msg_send![self.get_ptr(), setDrawableSize: size]
    }
    pub unsafe fn set_vsync(&self, enabled: bool) {
        msg_send![self.get_ptr(), setDisplaySyncEnabled: enabled]
    }
    pub unsafe fn wants_extended_dynamic_range_content(&self, wants: bool) {
        msg_send![self.get_ptr(), setWantsExtendedDynamicRangeContent: wants]
    }
    pub unsafe fn next_drawable(&self) -> Option<CAMetalDrawable> {
        let dw = ObjectPointer(msg_send![self.get_ptr(), nextDrawable]);
        if dw.0.is_null() {
            None
        } else {
            Some(CAMetalDrawable::from_ptr(dw))
        }
    }
}

impl Object for CAMetalLayer {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        CAMetalLayer(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}
