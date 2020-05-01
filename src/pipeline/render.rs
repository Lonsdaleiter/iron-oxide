use crate::import_macros::*;
use crate::{ObjectPointer, handle, Object, MTLFunction};

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
    pub unsafe fn set_vertex_function(&self, function: MTLFunction) {
        msg_send![self.get_ptr(), setVertexFunction:function.get_ptr()]
    }
}

impl Object for MTLRenderPipelineDescriptor {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self where
        Self: Sized {
        MTLRenderPipelineDescriptor(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}
