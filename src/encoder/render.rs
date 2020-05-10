use crate::import_objc_macros::*;
use crate::{handle, MTLCommandEncoder, MTLRenderPipelineState, Object, ObjectPointer};

#[repr(u64)]
pub enum MTLTriangleFillMode {
    Fill = 0,
    Lines = 1,
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
