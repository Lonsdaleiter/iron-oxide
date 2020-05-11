use crate::import_objc_macros::*;
use crate::{
    handle, Array, DeviceCreated, MTLFunction, MTLPixelFormat, NSUInteger, Object, ObjectPointer,
};
use enumflags2::BitFlags;

pub struct MTLVertexDescriptor(ObjectPointer);
handle!(MTLVertexDescriptor);

impl MTLVertexDescriptor {
    pub unsafe fn new() -> MTLVertexDescriptor {
        MTLVertexDescriptor::from_ptr(msg_send![class!(MTLVertexDescriptor), new])
    }
    // TODO add my behavior
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
pub enum MTLColorWriteMask {
    Red = 0x1 << 3,
    Green = 0x1 << 2,
    Blue = 0x1 << 1,
    Alpha = 0x1,
}

#[repr(u64)]
pub enum MTLBlendOperation {
    Add = 0,
    Subtract = 1,
    ReverseSubtract = 2,
    Min = 3,
    Max = 4,
}

#[repr(u64)]
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

pub struct MTLRenderPipelineColorAttachmentDescriptor(ObjectPointer);
handle!(MTLRenderPipelineColorAttachmentDescriptor);

impl MTLRenderPipelineColorAttachmentDescriptor {
    pub unsafe fn new() -> MTLRenderPipelineColorAttachmentDescriptor {
        MTLRenderPipelineColorAttachmentDescriptor::from_ptr(msg_send![
            class!(MTLRenderPipelineColorAttachmentDescriptor),
            new
        ])
    }
    pub unsafe fn set_write_mask(&self, mask: BitFlags<MTLColorWriteMask>) {
        msg_send![self.get_ptr(), setWriteMask:mask.bits()]
    }
    pub unsafe fn set_pixel_format(&self, format: MTLPixelFormat) {
        msg_send![self.get_ptr(), setPixelFormat: format]
    }
    pub unsafe fn set_blending_enabled(&self, enabled: bool) {
        msg_send![self.get_ptr(), setBlendingEnabled: enabled]
    }
    pub unsafe fn set_alpha_blend_operation(&self, operation: MTLBlendOperation) {
        msg_send![self.get_ptr(), setAlphaBlendOperation: operation]
    }
    pub unsafe fn set_rgb_blend_operation(&self, operation: MTLBlendOperation) {
        msg_send![self.get_ptr(), setRgbBlendOperation: operation]
    }
    pub unsafe fn set_destination_alpha_blend_factor(&self, factor: MTLBlendFactor) {
        msg_send![self.get_ptr(), setDestinationAlphaBlendFactor: factor]
    }
    pub unsafe fn set_destination_rgb_blend_factor(&self, factor: MTLBlendFactor) {
        msg_send![self.get_ptr(), setDestinationRGBBlendFactor: factor]
    }
    pub unsafe fn set_source_alpha_blend_factor(&self, factor: MTLBlendFactor) {
        msg_send![self.get_ptr(), setSourceAlphaBlendFactor: factor]
    }
    pub unsafe fn set_source_rgb_blend_factor(&self, factor: MTLBlendFactor) {
        msg_send![self.get_ptr(), setSourceRGBBlendFactor: factor]
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

#[repr(u64)]
pub enum MTLPrimitiveTopologyClass {
    Unspecified = 0,
    Point = 1,
    Line = 2,
    Triangle = 3,
}

pub struct MTLRenderPipelineDescriptor(ObjectPointer);
handle!(MTLRenderPipelineDescriptor);

impl MTLRenderPipelineDescriptor {
    pub unsafe fn new() -> MTLRenderPipelineDescriptor {
        MTLRenderPipelineDescriptor::from_ptr({
            let cl = class!(MTLRenderPipelineDescriptor);
            msg_send![cl, new]
        })
    }
    pub unsafe fn set_vertex_function(&self, function: &MTLFunction) {
        msg_send![self.get_ptr(), setVertexFunction:function.get_ptr()]
    }
    pub unsafe fn set_fragment_function(&self, function: &MTLFunction) {
        msg_send![self.get_ptr(), setFragmentFunction:function.get_ptr()]
    }
    pub unsafe fn set_vertex_descriptor(&self, desc: &MTLVertexDescriptor) {
        msg_send![self.get_ptr(), setVertexDescriptor:desc.get_ptr()]
    }
    pub unsafe fn get_color_attachments(&self) -> MTLRenderPipelineColorAttachmentDescriptorArray {
        MTLRenderPipelineColorAttachmentDescriptorArray::from_ptr({
            let ptr = ObjectPointer(msg_send![self.get_ptr(), colorAttachments]);
            // we should not release this array ourselves, Metal does it for us
            // we retain the pointer so that drop doesn't overrelease the pointer and
            // the reference count remains at at least one until the parent dies
            let _: () = msg_send![ptr, retain];
            ptr
        })
    }
    pub unsafe fn reset(&self) {
        msg_send![self.get_ptr(), reset]
    }
    pub unsafe fn set_depth_attachment_pixel_format(&self, format: MTLPixelFormat) {
        msg_send![self.get_ptr(), setDepthAttachmentPixelFormat: format]
    }
    pub unsafe fn set_stencil_attachment_pixel_format(&self, format: MTLPixelFormat) {
        msg_send![self.get_ptr(), setStencilAttachmentPixelFormat: format]
    }
    pub unsafe fn set_sample_count(&self, count: NSUInteger) {
        msg_send![self.get_ptr(), setSampleCount: count]
    }
    pub unsafe fn set_alpha_to_coverage_enabled(&self, enabled: bool) {
        msg_send![self.get_ptr(), setAlphaToCoverageEnabled: enabled]
    }
    pub unsafe fn set_alpha_to_one_enabled(&self, enabled: bool) {
        msg_send![self.get_ptr(), setAlphaToOneEnabled: enabled]
    }
    pub unsafe fn set_rasterization_enabled(&self, enabled: bool) {
        msg_send![self.get_ptr(), setRasteriationEnabled: enabled]
    }
    pub unsafe fn set_input_primitive_topology(&self, topology: MTLPrimitiveTopologyClass) {
        msg_send![self.get_ptr(), setInputPrimitiveTopology: topology]
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

pub struct MTLRenderPipelineState(ObjectPointer);
handle!(MTLRenderPipelineState);

impl MTLRenderPipelineState {
    //
}

impl DeviceCreated for MTLRenderPipelineState {}

impl Object for MTLRenderPipelineState {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLRenderPipelineState(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}
