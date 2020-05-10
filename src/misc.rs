use crate::import_objc_macros::*;
use crate::{handle, CGFloat, NSInteger, NSUInteger, Object, ObjectPointer};
use std::fmt::{Display, Formatter, Debug, Error};

/// Takes an implementor of `Object` and logs its description and retain count.
///
/// Assumes that the implementation of `get_ptr` given by `T` unconditionally
/// returns a pointer to a valid Objective-C object inheriting from `NSObject`.
///
/// # Example
///
/// ```
/// fn main() {
///     use iron_oxide::{MTLCreateSystemDefaultDevice, debug};
///
///     let device = unsafe { MTLCreateSystemDefaultDevice() };
///     unsafe { debug(&device) };
/// }
/// ```
pub unsafe fn debug<T: Object>(obj: &T) {
    use crate::import_objc_macros::*;

    let count: NSUInteger = msg_send![obj.get_ptr(), retainCount];
    let description = ObjectPointer(msg_send![obj.get_ptr(), description]);
    let description = {
        let bytes: *const u8 = msg_send![description, UTF8String];
        let len: NSUInteger = msg_send![description, length];
        let bytes = std::slice::from_raw_parts(bytes, len as usize);
        std::str::from_utf8(bytes).unwrap()
    };

    log::log!(log::Level::Info, "{}", description);
    log::log!(log::Level::Info, "Retain count of {}", count);
}

#[repr(u64)]
pub enum MTLCompareFunction {
    Never = 0,
    Less = 1,
    Equal = 2,
    LessEqual = 3,
    Greater = 4,
    NotEqual = 5,
    GreaterEqual = 6,
    Always = 7,
}

#[allow(non_camel_case_types)]
#[repr(u64)]
pub enum MTLPixelFormat {
    Invalid = 0,
    A8Unorm = 1,
    R8Unorm = 10,
    R8Unorm_sRGB = 11,
    R8Snorm = 12,
    R8Uint = 13,
    R8Sint = 14,
    R16Unorm = 20,
    R16Snorm = 22,
    R16Uint = 23,
    R16Sint = 24,
    R16Float = 25,
    RG8Unorm = 30,
    RG8Unorm_sRGB = 31,
    RG8Snorm = 32,
    RG8Uint = 33,
    RG8Sint = 34,
    B5G6R5Unorm = 40,
    A1BGR5Unorm = 41,
    ABGR4Unorm = 42,
    BGR5A1Unorm = 43,
    R32Uint = 53,
    R32Sint = 54,
    R32Float = 55,
    RG16Unorm = 60,
    RG16Snorm = 62,
    RG16Uint = 63,
    RG16Sint = 64,
    RG16Float = 65,
    RGBA8Unorm = 70,
    RGBA8Unorm_sRGB = 71,
    RGBA8Snorm = 72,
    RGBA8Uint = 73,
    RGBA8Sint = 74,
    BGRA8Unorm = 80,
    BGRA8Unorm_sRGB = 81,
    RGB10A2Unorm = 90,
    RGB10A2Uint = 91,
    RG11B10Float = 92,
    RGB9E5Float = 93,
    BGR10A2Unorm = 94,
    RG32Uint = 103,
    RG32Sint = 104,
    RG32Float = 105,
    RGBA16Unorm = 110,
    RGBA16Snorm = 112,
    RGBA16Uint = 113,
    RGBA16Sint = 114,
    RGBA16Float = 115,
    RGBA32Uint = 123,
    RGBA32Sint = 124,
    RGBA32Float = 125,
    BC1_RGBA = 130,
    BC1_RGBA_sRGB = 131,
    BC2_RGBA = 132,
    BC2_RGBA_sRGB = 133,
    BC3_RGBA = 134,
    BC3_RGBA_sRGB = 135,
    BC4_RUnorm = 140,
    BC4_RSnorm = 141,
    BC5_RGUnorm = 142,
    BC5_RGSnorm = 143,
    BC6H_RGBFloat = 150,
    BC6H_RGBUfloat = 151,
    BC7_RGBAUnorm = 152,
    BC7_RGBAUnorm_sRGB = 153,
    PVRTC_RGB_2BPP = 160,
    PVRTC_RGB_2BPP_sRGB = 161,
    PVRTC_RGB_4BPP = 162,
    PVRTC_RGB_4BPP_sRGB = 163,
    PVRTC_RGBA_2BPP = 164,
    PVRTC_RGBA_2BPP_sRGB = 165,
    PVRTC_RGBA_4BPP = 166,
    PVRTC_RGBA_4BPP_sRGB = 167,
    EAC_R11Unorm = 170,
    EAC_R11Snorm = 172,
    EAC_RG11Unorm = 174,
    EAC_RG11Snorm = 176,
    EAC_RGBA8 = 178,
    EAC_RGBA8_sRGB = 179,
    ETC2_RGB8 = 180,
    ETC2_RGB8_sRGB = 181,
    ETC2_RGB8A1 = 182,
    ETC2_RGB8A1_sRGB = 183,
    ASTC_4x4_sRGB = 186,
    ASTC_5x4_sRGB = 187,
    ASTC_5x5_sRGB = 188,
    ASTC_6x5_sRGB = 189,
    ASTC_6x6_sRGB = 190,
    ASTC_8x5_sRGB = 192,
    ASTC_8x6_sRGB = 193,
    ASTC_8x8_sRGB = 194,
    ASTC_10x5_sRGB = 195,
    ASTC_10x6_sRGB = 196,
    ASTC_10x8_sRGB = 197,
    ASTC_10x10_sRGB = 198,
    ASTC_12x10_sRGB = 199,
    ASTC_12x12_sRGB = 200,
    ASTC_4x4_LDR = 204,
    ASTC_5x4_LDR = 205,
    ASTC_5x5_LDR = 206,
    ASTC_6x5_LDR = 207,
    ASTC_6x6_LDR = 208,
    ASTC_8x5_LDR = 210,
    ASTC_8x6_LDR = 211,
    ASTC_8x8_LDR = 212,
    ASTC_10x5_LDR = 213,
    ASTC_10x6_LDR = 214,
    ASTC_10x8_LDR = 215,
    ASTC_10x10_LDR = 216,
    ASTC_12x10_LDR = 217,
    ASTC_12x12_LDR = 218,
    GBGR422 = 240,
    BGRG422 = 241,
    Depth16Unorm = 250,
    Depth32Float = 252,
    Stencil8 = 253,
    Depth24Unorm_Stencil8 = 255,
    Depth32Float_Stencil8 = 260,
    X32_Stencil8 = 261,
    X24_Stencil8 = 262,
    BGRA10_XR = 552,
    BGRA10_XR_SRGB = 553,
    BGR10_XR = 554,
    BGR10_XR_SRGB = 555,
}

pub struct NSError(ObjectPointer);
handle!(NSError);

impl NSError {
    pub unsafe fn get_code(&self) -> NSInteger {
        msg_send![self.get_ptr(), code]
    }
    pub unsafe fn get_domain(&self) -> &str {
        let domain = ObjectPointer(msg_send![self.get_ptr(), domain]);
        let bytes: *const u8 = msg_send![domain, UTF8String];
        let len: NSUInteger = msg_send![domain, length];
        let bytes = std::slice::from_raw_parts(bytes, len as usize);
        std::str::from_utf8(bytes).unwrap()
    }
    pub unsafe fn get_localized_description(&self) -> &str {
        let desc = ObjectPointer(msg_send![self.get_ptr(), localizedDescription]);
        let bytes: *const u8 = msg_send![desc, UTF8String];
        let len: NSUInteger = msg_send![desc, length];
        let bytes = std::slice::from_raw_parts(bytes, len as usize);
        std::str::from_utf8(bytes).unwrap()
    }
    pub unsafe fn get_localized_failure_reason(&self) -> &str {
        let reason = ObjectPointer(msg_send![self.get_ptr(), localizedFailureReason]);
        let bytes: *const u8 = msg_send![reason, UTF8String];
        let len: NSUInteger = msg_send![reason, length];
        let bytes = std::slice::from_raw_parts(bytes, len as usize);
        std::str::from_utf8(bytes).unwrap()
    }
}

impl Debug for NSError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(unsafe { self.get_localized_description() })
    }
}

impl Object for NSError {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        NSError(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}

/// A Rust range of NSUIntegers.
pub type NSUIntegerRange = std::ops::Range<NSUInteger>;

#[repr(C)]
pub struct MTLRegion {
    pub origin: MTLSize,
    pub size: MTLSize,
}

#[repr(C)]
pub struct NSRange {
    pub location: NSUInteger,
    pub length: NSUInteger,
}

#[repr(C)]
pub struct MTLSize {
    pub width: NSUInteger,
    pub height: NSUInteger,
    pub depth: NSUInteger,
}

#[repr(C)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

impl Display for MTLSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("W: {}, H: {}, D: {}", self.width, self.height, self.depth).as_str())
    }
}

#[repr(C)]
pub struct MTLSamplePosition {
    pub x: f32,
    pub y: f32,
}

impl Display for MTLSamplePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("X: {}, Y: {}", self.x, self.y).as_str())
    }
}
