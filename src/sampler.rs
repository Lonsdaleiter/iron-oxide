use crate::import_objc_macros::*;
use crate::{handle, MTLCompareFunction, NSUInteger, Object, ObjectPointer};

#[repr(u64)]
pub enum MTLSamplerAddressMode {
    ClampToEdge = 0,
    MirrorClampToEdge = 1,
    Repeat = 2,
    MirrorRepeat = 3,
    ClampToZero = 4,
    ClampToBorderColor = 5,
}

#[repr(u64)]
pub enum MTLSamplerBorderColor {
    /// 0, 0, 0, 0,
    TransparentBlack = 0,
    /// 0, 0, 0, 1,
    OpaqueBlack = 1,
    /// 1, 1, 1, 1,
    OpaqueWhite = 2,
}

#[repr(u64)]
pub enum MTLSamplerMinMagFilter {
    Nearest = 0,
    Linear = 1,
}

pub struct MTLSamplerDescriptor(ObjectPointer);
handle!(MTLSamplerDescriptor);

impl MTLSamplerDescriptor {
    pub unsafe fn new() -> MTLSamplerDescriptor {
        MTLSamplerDescriptor::from_ptr(msg_send![class!(MTLSamplerDescriptor), new])
    }
    pub unsafe fn set_normalized_coords(&self, normalized: bool) {
        msg_send![self.get_ptr(), setNormalizedCoordinates: normalized]
    }
    pub unsafe fn set_r_address_mode(&self, mode: MTLSamplerAddressMode) {
        msg_send![self.get_ptr(), setRAddressMode: mode]
    }
    pub unsafe fn set_s_address_mode(&self, mode: MTLSamplerAddressMode) {
        msg_send![self.get_ptr(), setSAddressMode: mode]
    }
    pub unsafe fn set_t_address_mode(&self, mode: MTLSamplerAddressMode) {
        msg_send![self.get_ptr(), setTAddressMode: mode]
    }
    pub unsafe fn set_border_color(&self, color: MTLSamplerBorderColor) {
        msg_send![self.get_ptr(), setBorderColor: color]
    }
    pub unsafe fn set_min_filter(&self, filter: MTLSamplerMinMagFilter) {
        msg_send![self.get_ptr(), setMinFilter: filter]
    }
    pub unsafe fn set_mag_filter(&self, filter: MTLSamplerMinMagFilter) {
        msg_send![self.get_ptr(), setMagFilter: filter]
    }
    pub unsafe fn set_map_filter(&self, filter: MTLSamplerMinMagFilter) {
        msg_send![self.get_ptr(), setMipFilter: filter]
    }
    pub unsafe fn set_lod_min_clamp(&self, clamp: f32) {
        msg_send![self.get_ptr(), setLodMinClamp: clamp]
    }
    pub unsafe fn set_lod_max_clamp(&self, clamp: f32) {
        msg_send![self.get_ptr(), setLodMaxClamp: clamp]
    }
    pub unsafe fn set_lod_average(&self, average: bool) {
        msg_send![self.get_ptr(), setLodAverage: average]
    }
    pub unsafe fn set_max_anisotropy(&self, max: NSUInteger) {
        msg_send![self.get_ptr(), setMaxAnisotropy: max]
    }
    pub unsafe fn set_compare_function(&self, function: MTLCompareFunction) {
        msg_send![self.get_ptr(), setCompareFunction: function]
    }
    pub unsafe fn set_supports_argument_buffers(&self, supports: bool) {
        msg_send![self.get_ptr(), setSupportArgumentBuffers: supports]
    }
}

impl Object for MTLSamplerDescriptor {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLSamplerDescriptor(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}

pub struct MTLSamplerState(ObjectPointer);
handle!(MTLSamplerState);

impl Object for MTLSamplerState {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLSamplerState(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}
