use crate::import_objc_macros::*;
use crate::{
    handle, DeviceCreated, MTLPixelFormat, MTLRegion, MTLResource, MTLResourceOptions, NSUInteger,
    NSUIntegerRange, Object, ObjectPointer,
};
use enumflags2::BitFlags;
use std::os::raw::c_void;

#[repr(u64)]
pub enum MTLTextureType {
    D1 = 0,
    D1Array = 1,
    D2 = 2,
    D2Array = 3,
    D2Multisample = 4,
    Cube = 5,
    CubeArray = 6,
    D3 = 7,
    D2MultisampleArray = 8,
    TextureBuffer = 9,
}

#[derive(BitFlags, Copy, Clone, Debug, PartialEq)]
#[repr(u64)]
pub enum MTLTextureUsage {
    ShaderRead = 0x0001,
    ShaderWrite = 0x0002,
    RenderTarget = 0x004,
    PixelFormatView = 0x0010,
}

pub struct MTLTextureDescriptor(ObjectPointer);
handle!(MTLTextureDescriptor);

impl MTLTextureDescriptor {
    pub unsafe fn new() -> MTLTextureDescriptor {
        MTLTextureDescriptor::from_ptr(msg_send![class!(MTLTextureDescriptor), new])
    }
    pub unsafe fn set_texture_type(&self, texture_type: MTLTextureType) {
        msg_send![self.get_ptr(), setTextureType: texture_type]
    }
    pub unsafe fn set_pixel_format(&self, format: MTLPixelFormat) {
        msg_send![self.get_ptr(), setPixelFormat: format]
    }
    pub unsafe fn set_width(&self, width: NSUInteger) {
        msg_send![self.get_ptr(), setWidth: width]
    }
    pub unsafe fn set_height(&self, height: NSUInteger) {
        msg_send![self.get_ptr(), setHeight: height]
    }
    pub unsafe fn set_depth(&self, depth: NSUInteger) {
        msg_send![self.get_ptr(), setDepth: depth]
    }
    pub unsafe fn set_mipmap_level_count(&self, count: NSUInteger) {
        msg_send![self.get_ptr(), setMipmapLevelCount: count]
    }
    pub unsafe fn set_sample_count(&self, count: NSUInteger) {
        msg_send![self.get_ptr(), setSampleCount: count]
    }
    pub unsafe fn set_array_length(&self, length: NSUInteger) {
        msg_send![self.get_ptr(), setArrayLength: length]
    }
    pub unsafe fn set_resource_options(&self, options: MTLResourceOptions) {
        msg_send![self.get_ptr(), setResourceOptions:options.bits]
    }
    pub unsafe fn set_allow_gpu_optimized_contents(&self, allow: bool) {
        msg_send![self.get_ptr(), setAllowGPUOptimizedContents: allow]
    }
    pub unsafe fn set_usage(&self, usage: BitFlags<MTLTextureUsage>) {
        msg_send![self.get_ptr(), setUsage: usage.bits()]
    }
}

impl Object for MTLTextureDescriptor {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLTextureDescriptor(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}

pub struct MTLTexture(ObjectPointer);
handle!(MTLTexture);

impl MTLTexture {
    pub unsafe fn replace_region(
        &self,
        region: MTLRegion,
        mipmap_level: NSUInteger,
        slice: NSUInteger,
        bytes: *const c_void,
        bytes_per_row: NSUInteger,
        bytes_per_image: NSUInteger,
    ) {
        msg_send![
            self.get_ptr(),
            replaceRegion:region
            mipmapLevel:mipmap_level
            slice:slice
            withBytes:bytes
            bytesPerRow:bytes_per_row
            bytesPerImage:bytes_per_image
        ]
    }
    pub unsafe fn get_bytes(
        &self,
        to_write: *mut c_void,
        bytes_per_row: NSUInteger,
        bytes_per_image: NSUInteger,
        region: MTLRegion,
        mipmap_level: NSUInteger,
        slice: NSUInteger,
    ) {
        msg_send![
            self.get_ptr(),
            getBytes:to_write
            bytesPerRow:bytes_per_row
            bytesPerImage:bytes_per_image
            fromRegion:region
            mipmapLevel:mipmap_level
            slice:slice
        ]
    }
    pub unsafe fn new_texture_view_with_pixel_format(&self, format: MTLPixelFormat) -> MTLTexture {
        MTLTexture::from_ptr(msg_send![
            self.get_ptr(),
            newTextureViewWithPixelFormat: format
        ])
    }
    pub unsafe fn new_texture_view_with_pixel_format_and_texture_type(
        &self,
        format: MTLPixelFormat,
        texture_type: MTLTextureType,
        levels: NSUIntegerRange,
        slices: NSUIntegerRange,
    ) -> MTLTexture {
        MTLTexture::from_ptr(msg_send![
            self.get_ptr(),
            newTextureViewWithPixelFormat:format
            textureType:texture_type
            levels:levels
            slices:slices
        ])
    }
}

impl MTLResource for MTLTexture {}

impl DeviceCreated for MTLTexture {}

impl Object for MTLTexture {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLTexture(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}
