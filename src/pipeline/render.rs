use crate::import_objc_macros::*;
use crate::{handle, Array, MTLFunction, Object, ObjectPointer};

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
