use crate::import_objc_macros::*;
use crate::{handle, MTLBuffer, MTLCommandEncoder, MTLDepthStencilState, MTLRenderPipelineState, MTLSamplerState, NSRange, NSUInteger, NSUIntegerRange, Object, ObjectPointer, MTLTexture};
use std::os::raw::c_void;

#[repr(u64)]
pub enum MTLTriangleFillMode {
    Fill = 0,
    Lines = 1,
}

#[repr(u64)]
pub enum MTLWinding {
    Clockwise = 0,
    CounterClockwise = 1,
}

#[repr(u64)]
pub enum MTLCullMode {
    None = 0,
    Front = 1,
    Back = 2,
}

#[repr(u64)]
pub enum MTLDepthClipMode {
    Clip = 0,
    Clamp = 1,
}

#[repr(C)]
pub struct MTLViewport {
    pub origin_x: f64,
    pub origin_y: f64,
    pub width: f64,
    pub height: f64,
    pub znear: f64,
    pub zfar: f64,
}

#[repr(C)]
pub struct MTLScissorRect {
    pub width: NSUInteger,
    pub height: NSUInteger,
    pub x: NSUInteger,
    pub y: NSUInteger,
}

pub struct MTLRenderCommandEncoder(ObjectPointer);
handle!(MTLRenderCommandEncoder);

impl MTLRenderCommandEncoder {
    pub unsafe fn set_render_pipeline_state(&self, state: &MTLRenderPipelineState) {
        msg_send![self.get_ptr(), setRenderPipelineState:state.get_ptr()]
    }
    pub unsafe fn set_triangle_fill_mode(&self, mode: MTLTriangleFillMode) {
        msg_send![self.get_ptr(), setTriangleFillMode: mode]
    }
    pub unsafe fn set_front_face_winding(&self, winding: MTLWinding) {
        msg_send![self.get_ptr(), setFrontFaceWinding: winding]
    }
    pub unsafe fn set_cull_mode(&self, mode: MTLCullMode) {
        msg_send![self.get_ptr(), setCullMode: mode]
    }
    pub unsafe fn set_depth_stencil_state(&self, state: &MTLDepthStencilState) {
        msg_send![self.get_ptr(), setDepthStencilState:state.get_ptr()]
    }
    pub unsafe fn set_depth_clip_mode(&self, mode: MTLDepthClipMode) {
        msg_send![self.get_ptr(), setDepthClipMode: mode]
    }
    pub unsafe fn set_stencil_reference_values(&self, front: u32, back: u32) {
        msg_send![self.get_ptr(), setStencilFrontReferenceValue:front backReferenceValue:back]
    }
    pub unsafe fn set_viewport(&self, viewport: MTLViewport) {
        msg_send![self.get_ptr(), setViewport: viewport]
    }
    pub unsafe fn set_viewports(&self, viewports: &[MTLViewport]) {
        msg_send![self.get_ptr(), setViewports:viewports.as_ptr() count:viewports.len()]
    }
    pub unsafe fn set_scissor_rect(&self, rect: MTLScissorRect) {
        msg_send![self.get_ptr(), setScissorRect: rect]
    }
    pub unsafe fn set_scissor_rects(&self, rects: &[MTLScissorRect]) {
        msg_send![self.get_ptr(), setScissorRects:rects.as_ptr() count:rects.len()]
    }
    pub unsafe fn set_blend_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        msg_send![self.get_ptr(), setBlendColorRed:red green:green blue:blue alpha:alpha]
    }
    pub unsafe fn set_vertex_buffer(
        &self,
        buffer: &MTLBuffer,
        offset: NSUInteger,
        index: NSUInteger,
    ) {
        msg_send![self.get_ptr(), setVertexBuffer:buffer.get_ptr() offset:offset atIndex:index]
    }
    pub unsafe fn set_vertex_buffers(
        &self,
        buffers: &[MTLBuffer],
        offsets: &[NSUInteger],
        range: NSUIntegerRange,
    ) {
        let range = NSRange {
            location: range.start,
            length: range.end - range.start,
        };
        let pointers = buffers
            .iter()
            .map(|buffer| buffer.get_ptr())
            .collect::<Vec<ObjectPointer>>();
        let pointers = pointers.as_slice().as_ptr();

        msg_send![self.get_ptr(), setVertexBuffers:pointers offsets:offsets.as_ptr() withRange:range]
    }
    pub unsafe fn set_vertex_buffer_offset(&self, offset: NSUInteger, index: NSUInteger) {
        msg_send![self.get_ptr(), setVertexBufferOffset:offset atIndex:index]
    }
    pub unsafe fn set_vertex_bytes(
        &self,
        bytes: *const c_void,
        length: NSUInteger,
        index: NSUInteger,
    ) {
        msg_send![self.get_ptr(), setVertexBytes:bytes length:length atIndex:index]
    }
    pub unsafe fn set_vertex_sampler_state(&self, sampler: &MTLSamplerState, index: NSUInteger) {
        msg_send![self.get_ptr(), setVertexSamplerState:sampler.get_ptr() atIndex:index]
    }
    pub unsafe fn set_vertex_sampler_state_clamp(
        &self,
        sampler: &MTLSamplerState,
        lod_min_clamp: f32,
        lod_max_clamp: f32,
        index: NSUInteger,
    ) {
        msg_send![
            self.get_ptr(),
            setVertexSamplerState:sampler.get_ptr()
            lodMinClamp:lod_min_clamp
            lodMaxClamp:lod_max_clamp
            atIndex:index
        ]
    }
    pub unsafe fn set_vertex_sampler_states(
        &self,
        samplers: &[MTLSamplerState],
        range: NSUIntegerRange,
    ) {
        let pointers = samplers
            .iter()
            .map(|sampler| sampler.get_ptr())
            .collect::<Vec<_>>();
        let pointers = pointers.as_slice().as_ptr();
        let range = NSRange {
            location: range.start,
            length: range.end - range.start,
        };
        msg_send![self.get_ptr(), setVertexSamplerStates:pointers withRange:range]
    }
    pub unsafe fn set_vertex_texture(&self, texture: &MTLTexture, index: NSUInteger) {
        msg_send![self.get_ptr(), setVertexTexture:texture.get_ptr() atIndex:index]
    }
    pub unsafe fn set_vertex_textures(&self, textures: &[MTLTexture], range: NSUIntegerRange) {
        let pointers = textures.iter().map(|texture|texture.get_ptr()).collect::<Vec<_>>();
        let pointers = pointers.as_slice().as_ptr();
        let range = NSRange {
            location: range.start,
            length: range.end - range.start,
        };
        msg_send![self.get_ptr(), setVertexTextures:pointers withRange:range]
    }

    pub unsafe fn set_fragment_buffer(
        &self,
        buffer: &MTLBuffer,
        offset: NSUInteger,
        index: NSUInteger,
    ) {
        msg_send![self.get_ptr(), setFragmentBuffer:buffer.get_ptr() offset:offset atIndex:index]
    }
    pub unsafe fn set_fragment_buffers(
        &self,
        buffers: &[MTLBuffer],
        offsets: &[NSUInteger],
        range: NSUIntegerRange,
    ) {
        let range = NSRange {
            location: range.start,
            length: range.end - range.start,
        };
        let pointers = buffers
            .iter()
            .map(|buffer| buffer.get_ptr())
            .collect::<Vec<ObjectPointer>>();
        let pointers = pointers.as_slice().as_ptr();

        msg_send![self.get_ptr(), setFragmentBuffers:pointers offsets:offsets.as_ptr() withRange:range]
    }
    pub unsafe fn set_fragment_buffer_offset(&self, offset: NSUInteger, index: NSUInteger) {
        msg_send![self.get_ptr(), setFragmentBufferOffset:offset atIndex:index]
    }
    pub unsafe fn set_fragment_bytes(
        &self,
        bytes: *const c_void,
        length: NSUInteger,
        index: NSUInteger,
    ) {
        msg_send![self.get_ptr(), setFragmentBytes:bytes length:length atIndex:index]
    }
    pub unsafe fn set_fragment_sampler_state(&self, sampler: &MTLSamplerState, index: NSUInteger) {
        msg_send![self.get_ptr(), setFragmentSamplerState:sampler.get_ptr() atIndex:index]
    }
    pub unsafe fn set_fragment_sampler_state_clamp(
        &self,
        sampler: &MTLSamplerState,
        lod_min_clamp: f32,
        lod_max_clamp: f32,
        index: NSUInteger,
    ) {
        msg_send![
            self.get_ptr(),
            setFragmentSamplerState:sampler.get_ptr()
            lodMinClamp:lod_min_clamp
            lodMaxClamp:lod_max_clamp
            atIndex:index
        ]
    }
    pub unsafe fn set_fragment_sampler_states(
        &self,
        samplers: &[MTLSamplerState],
        range: NSUIntegerRange,
    ) {
        let pointers = samplers
            .iter()
            .map(|sampler| sampler.get_ptr())
            .collect::<Vec<_>>();
        let pointers = pointers.as_slice().as_ptr();
        let range = NSRange {
            location: range.start,
            length: range.end - range.start,
        };
        msg_send![self.get_ptr(), setFragmentSamplerStates:pointers withRange:range]
    }
    pub unsafe fn set_fragment_texture(&self, texture: &MTLTexture, index: NSUInteger) {
        msg_send![self.get_ptr(), setFragmentTexture:texture.get_ptr() atIndex:index]
    }
    pub unsafe fn set_fragment_textures(&self, textures: &[MTLTexture], range: NSUIntegerRange) {
        let pointers = textures.iter().map(|texture|texture.get_ptr()).collect::<Vec<_>>();
        let pointers = pointers.as_slice().as_ptr();
        let range = NSRange {
            location: range.start,
            length: range.end - range.start,
        };
        msg_send![self.get_ptr(), setFragmentTextures:pointers withRange:range]
    }
}

impl MTLCommandEncoder for MTLRenderCommandEncoder {}

impl Object for MTLRenderCommandEncoder {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLRenderCommandEncoder(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}
