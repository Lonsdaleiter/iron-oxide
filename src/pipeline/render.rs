use crate::import_objc_macros::*;
use crate::{handle, Array, MTLFunction, Object, ObjectPointer, MTLPixelFormat, NSUInteger};
use enumflags2::BitFlags;

/// Describes how buffers at specified indices are mapped to attributes at specified indices.
///
/// Irrelevant if there is no [[stage_in]] in the vertex shader and buffers given to the
/// vertex shader are specified therein as buffers, not attributes.
///
/// Will send to its pointer only messages specified in the MTLVertexDescriptor interface
/// linked [here](https://developer.apple.com/documentation/metal/mtlvertexdescriptor?language=objc).
pub struct MTLVertexDescriptor(ObjectPointer);
handle!(MTLVertexDescriptor);

impl MTLVertexDescriptor {
    //
}

impl Object for MTLVertexDescriptor {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLVertexDescriptor(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}

/// An array of MTLRenderPipelineColorAttachmentDescriptors.
pub struct MTLRenderPipelineColorAttachmentDescriptorArray(ObjectPointer);
handle!(MTLRenderPipelineColorAttachmentDescriptorArray);

impl Array<MTLRenderPipelineColorAttachmentDescriptor>
    for MTLRenderPipelineColorAttachmentDescriptorArray
{
}

impl Object for MTLRenderPipelineColorAttachmentDescriptorArray {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLRenderPipelineColorAttachmentDescriptorArray(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}

#[derive(BitFlags, Copy, Clone, Debug, PartialEq)]
#[repr(u64)]
/// Bitflags describing color channels, and which of them are enabled.
///
/// Analogous to [this](https://developer.apple.com/documentation/metal/mtlcolorwritemask?language=objc).
pub enum MTLColorWriteMask {
    Red = 0x1 << 3,
    Green = 0x1 << 2,
    Blue = 0x1 << 1,
    Alpha = 0x1,
}

#[repr(u64)]
/// Describes an operation to use in a blend equation.
///
/// More variant specific details can be found [here](https://developer.apple.com/documentation/metal/mtlblendoperation?language=objc).
pub enum MTLBlendOperation {
    Add = 0,
    Subtract = 1,
    ReverseSubtract = 2,
    Min = 3,
    Max = 4,
}

#[repr(u64)]
/// Factors in a blend equation.
///
/// More invariant specific details can be found [here](https://developer.apple.com/documentation/metal/mtlblendfactor?language=objc).
pub enum MTLBlendFactor {
    Zero = 0,
    One = 1,
    SourceColor = 2,
    OneMinusSourceColor = 3,
    SourceAlpha = 4,
    OneMinusSourceAlpha = 5,
    DestinationColor = 6,
    OneMinusDestinationColor = 7,
    DestinationAlpha = 8,
    OneMinusDestinationAlpha = 9,
    SourceAlphaSaturated = 10,
    BlendColor = 11,
    OneMinusBlendColor = 12,
    BlendAlpha = 13,
    OneMinusBlendAlpha = 14,
    Source1Color = 15,
    OneMinusSource1Color = 16,
    Source1Alpha = 17,
    OneMinusSource1Alpha = 18,
}

/// Settings for the creation of an MTLRenderPipelineColorAttachmentDescriptorDescriptor.
///
/// Will send to its pointer only messages specified in the MTLRenderPipelineColorAttachmentDescriptorDescriptor interface
/// linked [here](https://developer.apple.com/documentation/metal/MTLRenderPipelineColorAttachmentDescriptordescriptor?language=objc).
pub struct MTLRenderPipelineColorAttachmentDescriptor(ObjectPointer);
handle!(MTLRenderPipelineColorAttachmentDescriptor);

impl MTLRenderPipelineColorAttachmentDescriptor {
    /// Creates a new MTLRenderPipelineColorAttachmentDescriptor with standard allocation and initialization.
    pub unsafe fn new() -> MTLRenderPipelineColorAttachmentDescriptor {
        MTLRenderPipelineColorAttachmentDescriptor::from_ptr(msg_send![
            class!(MTLRenderPipelineColorAttachmentDescriptor),
            new
        ])
    }
    /// Sets the [writeMask](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/1514619-writemask?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_write_mask(&self, mask: BitFlags<MTLColorWriteMask>) {
        msg_send![self.get_ptr(), setWriteMask:mask.bits()]
    }
    /// Sets the [pixelFormat](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/1514651-pixelformat?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_pixel_format(&self, format: MTLPixelFormat) {
        msg_send![self.get_ptr(), setPixelFormat:format]
    }
    /// Sets the [blendingEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/1514642-blendingenabled?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_blending_enabled(&self, enabled: bool) {
        msg_send![self.get_ptr(), setBlendingEnabled:enabled]
    }
    /// Sets the [alphaBlendOperation](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/1514666-alphablendoperation?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_alpha_blend_operation(&self, operation: MTLBlendOperation) {
        msg_send![self.get_ptr(), setAlphaBlendOperation:operation]
    }
    /// Sets the [rgbBlendOperation](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/1514659-rgbblendoperation?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_rgb_blend_operation(&self, operation: MTLBlendOperation) {
        msg_send![self.get_ptr(), setRgbBlendOperation:operation]
    }
    /// Sets the [destinationAlphaBlendFactor](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/1514657-destinationalphablendfactor?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_destination_alpha_blend_factor(&self, factor: MTLBlendFactor) {
        msg_send![self.get_ptr(), setDestinationAlphaBlendFactor:factor]
    }
    /// Sets the [destinationRGBBlendFactor](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/1514626-destinationrgbblendfactor?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_destination_rgb_blend_factor(&self, factor: MTLBlendFactor) {
        msg_send![self.get_ptr(), setDestinationRGBBlendFactor:factor]
    }
    /// Sets the [sourceAlphaBlendFactor](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/1514660-sourcealphablendfactor?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_source_alpha_blend_factor(&self, factor: MTLBlendFactor) {
        msg_send![self.get_ptr(), setSourceAlphaBlendFactor:factor]
    }
    /// Sets the [sourceRGBBlendFactor](https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor/1514615-sourcergbblendfactor?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_source_rgb_blend_factor(&self, factor: MTLBlendFactor) {
        msg_send![self.get_ptr(), setSourceRGBBlendFactor:factor]
    }
}

impl Object for MTLRenderPipelineColorAttachmentDescriptor {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLRenderPipelineColorAttachmentDescriptor(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}

/// Settings for the creation of an MTLRenderPipelineState.
///
/// Will send to its pointer only messages specified in the MTLRenderPipelineDescriptor interface
/// linked [here](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor?language=objc).
pub struct MTLRenderPipelineDescriptor(ObjectPointer);
handle!(MTLRenderPipelineDescriptor);

impl MTLRenderPipelineDescriptor {
    /// Creates a new MTLRenderPipelineDescriptor with standard allocation and initialization.
    pub unsafe fn new() -> MTLRenderPipelineDescriptor {
        MTLRenderPipelineDescriptor::from_ptr({
            let cl = class!(MTLRenderPipelineDescriptor);
            msg_send![cl, new]
        })
    }
    /// Sets the [vertexFunction](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1514679-vertexfunction?language=objc)
    /// attribute of the descriptor.
    ///
    /// *Must* be set.
    pub unsafe fn set_vertex_function(&self, function: &MTLFunction) {
        msg_send![self.get_ptr(), setVertexFunction:function.get_ptr()]
    }
    /// Sets the [fragmentFunction](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1514600-fragmentfunction?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_fragment_function(&self, function: &MTLFunction) {
        msg_send![self.get_ptr(), setFragmentFunction:function.get_ptr()]
    }
    /// Sets the [vertexDescriptor](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1514681-vertexdescriptor?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_vertex_descriptor(&self, desc: &MTLVertexDescriptor) {
        msg_send![self.get_ptr(), setVertexDescriptor:desc.get_ptr()]
    }
    /// Returns the [colorAttachments](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1514712-colorattachments?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn get_color_attachments(&self) -> MTLRenderPipelineColorAttachmentDescriptorArray {
        MTLRenderPipelineColorAttachmentDescriptorArray::from_ptr({
            let ptr = ObjectPointer(msg_send![self.get_ptr(), colorAttachments]);
            // we retain the pointer so that drop doesn't overrelease the pointer
            let _: () = msg_send![ptr, retain];
            ptr
        })
    }
    /// Calls the [reset](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1514688-reset?language=objc)
    /// instance method.
    pub unsafe fn reset(&self) {
        msg_send![self.get_ptr(), reset]
    }
    /// Sets the [depthAttachmentPixelFormat](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1514608-depthattachmentpixelformat?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_depth_attachment_pixel_format(&self, format: MTLPixelFormat) {
        msg_send![self.get_ptr(), setDepthAttachmentPixelFormat]
    }
    /// Sets the [stencilAttachmentPixelFormat](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1514650-stencilattachmentpixelformat?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_stencil_attachment_pixel_format(&self, format: MTLPixelFormat) {
        msg_send![self.get_ptr(), setStencilAttachmentPixelFormat]
    }
    /// Sets the [sampleCount](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1514699-samplecount?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_sample_count(&self, count: NSUInteger) {
        msg_send![self.get_ptr(), setSampleCount]
    }
    /// Sets the [alphaToCoverageEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1514624-alphatocoverageenabled?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_alpha_to_coverage_enabled(&self, enabled: bool) {
        msg_send![self.get_ptr(), setAlphaToCoverageEnabled:enabled]
    }
    /// Sets the [alphaToOneEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1514697-alphatooneenabled?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_alpha_to_one_enabled(&self, enabled: bool) {
        msg_send![self.get_ptr(), setAlphaToOneEnabled:enabled]
    }
    /// Sets the [rasterizationEnabled](https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor/1514708-rasterizationenabled?language=objc)
    /// attribute of the descriptor.
    pub unsafe fn set_rasterization_enabled(&self, enabled: bool) {
        msg_send![self.get_ptr(), setRasteriationEnabled:enabled]
    }
}

impl Object for MTLRenderPipelineDescriptor {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLRenderPipelineDescriptor(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}
