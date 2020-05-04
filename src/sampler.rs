use crate::{ObjectPointer, handle, Object};
use crate::import_objc_macros::*;

#[repr(u64)]
/// Determines the texture coordinate at each pixel when a query
/// falls outside of a texture's bounds.
///
/// Analogous to [this](https://developer.apple.com/documentation/metal/mtlsampleraddressmode?language=objc).
pub enum MTLSamplerAddressMode {
    ClampToEdge = 0,
    MirrorClampToEdge = 1,
    Repeat = 2,
    MirrorRepeat = 3,
    ClampToZero = 4,
    ClampToBorderColor = 5,
}

#[repr(u64)]
/// Describes colors for out-of-bounds texture values when the chosen MTLSamplerAddressMode
/// is ClampToBorderColor.
pub enum MTLSamplerBorderColor {
    /// 0, 0, 0, 0,
    TransparentBlack = 0,
    /// 0, 0, 0, 1,
    OpaqueBlack = 1,
    /// 1, 1, 1, 1,
    OpaqueWhite = 2,
}

#[repr(u64)]
/// Options for determining which pixel value is returned within a mipmap level.
pub enum MTLSamplerMinMagFilter {
    /// The nearest pixel to the sample point.
    Nearest = 0,
    /// Select 2 pixels per dimension and linearly interpolate.
    Linear = 1,
}

/// Describes a sampler which samples textures.
///
/// Will send to its pointer only messages specified in theMTLSamplerDescriptor interface
/// linked [here](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor?language=objc).
pub struct MTLSamplerDescriptor(ObjectPointer);
handle!(MTLSamplerDescriptor);

impl MTLSamplerDescriptor {
    /// Sets the [normalizedCoords](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/1516289-normalizedcoordinates?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_normalized_coords(&self, normalized: bool) {
        msg_send![self.get_ptr(), setNormalizedCoordinates:normalized]
    }
    /// Sets the [rAddressMode](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/1515466-raddressmode?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_r_address_mode(&self, mode: MTLSamplerAddressMode) {
        msg_send![self.get_ptr(), setRAddressMode:mode]
    }
    /// Sets the [sAddressMode](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/1515779-saddressmode?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_s_address_mode(&self, mode: MTLSamplerAddressMode) {
        msg_send![self.get_ptr(), setSAddressMode:mode]
    }
    /// Sets the [tAddressMode](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/1515900-taddressmode?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_t_address_mode(&self, mode: MTLSamplerAddressMode) {
        msg_send![self.get_ptr(), setTAddressMode:mode]
    }
    /// Sets the [borderColor](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/2092299-bordercolor?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_border_color(&self, color: MTLSamplerBorderColor) {
        msg_send![self.get_ptr(), setBorderColor:color]
    }
    /// Sets the [minFilter](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/1515792-minfilter?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_min_filter(&self, filter: MTLSamplerMinMagFilter) {
        msg_send![self.get_ptr(), setMinFilter:filter]
    }
    /// Sets the [magFilter](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/1515926-magfilter?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_mag_filter(&self, filter: MTLSamplerMinMagFilter) {
        msg_send![self.get_ptr(), setMagFilter:filter]
    }
    /// Sets the [mipFilter](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/1515553-mipfilter?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_map_filter(&self, filter: MTLSamplerMinMagFilter) {
        msg_send![self.get_ptr(), setMipFilter:filter]
    }
    /// Sets the [lodMinClamp](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/1515629-lodminclamp?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_lod_min_clamp(&self, clamp: f32) {
        msg_send![self.get_ptr(), setLodMinClamp:clamp]
    }
    /// Sets the [lodMaxClamp](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/1516234-lodmaxclamp?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_lod_max_clamp(&self, clamp: f32) {
        msg_send![self.get_ptr(), setLodMaxClamp:clamp]
    }
    /// Sets the [lodAverage](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor/1615844-lodaverage?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_lod_average(&self, average: bool) {
        msg_send![self.get_ptr(), setLodAverage:average]
    }
}

impl Object for MTLSamplerDescriptor {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self where
        Self: Sized {
        MTLSamplerDescriptor(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}
