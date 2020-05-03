use crate::import_objc_macros::*;
use crate::{handle, DeviceCreated, MTLDevice, MTLRegion, NSUInteger, Object, ObjectPointer};
use std::os::raw::c_void;

/// A resource which stores data.
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
        mipmap_level: NSUinteger,
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
