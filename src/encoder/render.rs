use crate::import_objc_macros::*;
use crate::{handle, MTLCommandEncoder, MTLRenderPipelineState, Object, ObjectPointer, MTLDepthStencilState};

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
        msg_send![self.get_ptr(), setFrontFaceWinding:winding]
    }
    pub unsafe fn set_cull_mode(&self, mode: MTLCullMode) {
        msg_send![self.get_ptr(), setCullMode:mode]
    }
    pub unsafe fn set_depth_stencil_state(&self, state: &MTLDepthStencilState) {
        msg_send![self.get_ptr(), setDepthStencilState:state.get_ptr()]
    }
    pub unsafe fn set_depth_clip_mode(&self, mode: MTLDepthClipMode) {
        msg_send![self.get_ptr(), setDepthClipMode:mode]
    }
    pub unsafe fn set_stencil_reference_values(&self, front: u32, back: u32) {
        msg_send![self.get_ptr(), setStencilFrontReferenceValue:front backReferenceValue:back]
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
