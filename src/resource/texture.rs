use crate::import_objc_macros::*;
use crate::{
    handle, DeviceCreated, MTLDevice, MTLPixelFormat, MTLRegion, NSUInteger, NSUIntegerRange,
    Object, ObjectPointer,
};
use std::os::raw::c_void;

#[repr(u64)]
/// The dimension of each image, including whether multiple images
/// are arranged into an array or a cube.
///
/// Analogous to [this](https://developer.apple.com/documentation/metal/mtltexturetype?language=objc).
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

/// Describes a texture.
///
/// Will send to its pointer only the messages specified in the MTLTextureDescriptor
/// interface linked [here](https://developer.apple.com/documentation/metal/mtltexturedescriptor?language=objc).
pub struct MTLTextureDescriptor(ObjectPointer);
handle!(MTLTextureDescriptor);

impl MTLTextureDescriptor {
    /// Sets the [textureType](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1516228-texturetype?language=objc)
    /// property of the descriptor.
    pub unsafe fn set_texture_type(&self, texture_type: MTLTextureType) {
        msg_send![self.get_ptr(), setTextureType:texture_type]
    }
    /// Sets the [pixelFormat](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1515450-pixelformat?language=objc)
    /// property of the descriptor.
    pub unsafe fn set_pixel_format(&self, format: MTLPixelFormat) {
        msg_send![self.get_ptr(), setPixelFormat:format]
    }
    /// Sets the [width](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1515649-width?language=objc)
    /// property of the descriptor.
    pub unsafe fn set_width(&self, width: NSUInteger) {
        msg_send![self.get_ptr() setWidth:width]
    }
    /// Sets the [height](https://developer.apple.com/documentation/metal/mtltexturedescriptor/1516000-height?language=objc)
    /// property of the descriptor.
    pub unsafe fn set_height(&self, height: NSUInteger) {
        msg_send![self.get_ptr(), setHeight:height]
    }
}

impl Object for MTLTextureDescriptor {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self where
        Self: Sized {
        MTLTextureDescriptor(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}

/// A resource which stores formatted image data.
///
/// Will send to its pointer only the messages specified in the MTLTexture protocol
/// linked [here](https://developer.apple.com/documentation/metal/mtltexture?language=objc).
pub struct MTLTexture(ObjectPointer);
handle!(MTLTexture);

impl MTLTexture {
    /// Copies the specified pixel data into the specified section of a texture slice via
    /// the [replaceRegion](https://developer.apple.com/documentation/metal/mtltexture/1515679-replaceregion?language=objc)
    /// instance method.
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
    /// Copies the pixel data from the specified region of the texture to a specified
    /// part of RAM via the [getBytes](https://developer.apple.com/documentation/metal/mtltexture/1516318-getbytes?language=objc)
    /// instance method.
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
    /// Reinterprets this texture's data using a new pixel format but sharing the same
    /// storage allocation via the [newTextureViewWithPixelFormat](https://developer.apple.com/documentation/metal/mtltexture/1515598-newtextureviewwithpixelformat?language=objc)
    /// instance method.
    pub unsafe fn new_texture_view_with_pixel_format(&self, format: MTLPixelFormat) -> MTLTexture {
        MTLTexture::from_ptr(msg_send![
            self.get_ptr(),
            newTextureViewWithPixelFormat: format
        ])
    }
    /// Reinterprets this texture's data using a new pixel format and type but sharing the same
    /// storage allocation via the [newTextureViewWithPixelFormat](https://developer.apple.com/documentation/metal/mtltexture/1515409-newtextureviewwithpixelformat?language=objc)
    /// instance method.
    pub unsafe fn new_texture_view_with_pixel_format_and_texture_type(
        &self,
        format: MTLPixelFormat,
        texture_type: MTLTextureType,
        levels: NSUIntegerRange,
        slices: NSUIntegerRange,
    ) -> MTLTexture {
        MTLTexture::from_ptr(
            msg_send![
                self.get_ptr(),
                newTextureViewWithPixelFormat:format
                textureType:texture_type
                levels:levels
                slices:slices
            ]
        )
    }
}

impl DeviceCreated for MTLTexture {
    unsafe fn get_device(&self) -> MTLDevice {
        MTLDevice::from_ptr(msg_send![self.get_ptr(), device])
    }
}

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
