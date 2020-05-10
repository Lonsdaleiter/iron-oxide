use crate::import_objc_macros::*;
use crate::{handle, Array, MTLTexture, NSUInteger, Object, ObjectPointer};

#[repr(u64)]
pub enum MTLLoadAction {
    DontCare = 0,
    Load = 1,
    Clear = 2,
}

#[repr(u64)]
pub enum MTLStoreAction {
    DontCare = 0,
    Store = 1,
    MultisampleResolve = 2,
    StoreAndMultisampleResolve = 3,
    Unknown = 4,
    CustomSampleDepthStore = 5,
}

#[repr(u64)]
pub enum MTLStoreActionOptions {
    None = 0,
    CustomSamplePositions = 1,
}

pub trait MTLRenderPassAttachmentDescriptor: Object {
    unsafe fn set_texture(&self, texture: &MTLTexture) {
        msg_send![self.get_ptr(), setTexture:texture.get_ptr()]
    }
    unsafe fn set_level(&self, level: NSUInteger) {
        msg_send![self.get_ptr(), setLevel: level]
    }
    unsafe fn set_slice(&self, slice: NSUInteger) {
        msg_send![self.get_ptr(), setSlice: slice]
    }
    unsafe fn set_depth_plane(&self, plane: NSUInteger) {
        msg_send![self.get_ptr(), setDepthPlane: plane]
    }
    unsafe fn set_load_action(&self, action: MTLLoadAction) {
        msg_send![self.get_ptr(), setLoadAction: action]
    }
    unsafe fn set_store_action(&self, action: MTLStoreAction) {
        msg_send![self.get_ptr(), setStoreAction: action]
    }
    unsafe fn set_store_action_options(&self, options: MTLStoreActionOptions) {
        msg_send![self.get_ptr(), setStoreActionOptions: options]
    }
    unsafe fn set_resolve_texture(&self, texture: &MTLTexture) {
        msg_send![self.get_ptr(), setResolveTexture:texture.get_ptr()]
    }
    unsafe fn set_resolve_level(&self, level: NSUInteger) {
        msg_send![self.get_ptr(), setResolveLevel: level]
    }
    unsafe fn set_resolve_slice(&self, slice: NSUInteger) {
        msg_send![self.get_ptr(), setResolveSlice: slice]
    }
    unsafe fn set_resolve_depth_plane(&self, plane: NSUInteger) {
        msg_send![self.get_ptr(), setResolveDepthPlane: plane]
    }
}

#[repr(C)]
pub struct MTLClearColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

pub struct MTLRenderPassColorAttachmentDescriptor(ObjectPointer);
handle!(MTLRenderPassColorAttachmentDescriptor);

impl MTLRenderPassColorAttachmentDescriptor {
    pub unsafe fn new() -> MTLRenderPassColorAttachmentDescriptor {
        MTLRenderPassColorAttachmentDescriptor::from_ptr(msg_send![
            class!(MTLRenderPassColorAttachmentDescriptor),
            new
        ])
    }
    pub unsafe fn set_clear_color(&self, color: MTLClearColor) {
        msg_send![self.get_ptr(), setClearColor: color]
    }
}

impl MTLRenderPassAttachmentDescriptor for MTLRenderPassColorAttachmentDescriptor {}

impl Object for MTLRenderPassColorAttachmentDescriptor {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLRenderPassColorAttachmentDescriptor(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}

pub struct MTLRenderPassColorAttachmentDescriptorArray(ObjectPointer);
handle!(MTLRenderPassColorAttachmentDescriptorArray);

impl Array<MTLRenderPassColorAttachmentDescriptor> for MTLRenderPassColorAttachmentDescriptorArray {}

impl Object for MTLRenderPassColorAttachmentDescriptorArray {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLRenderPassColorAttachmentDescriptorArray(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}

#[repr(u64)]
pub enum MTLMultisampleDepthResolveFilter {
    Sample0 = 0,
    FilterMin = 1,
    FilterMax = 2,
}

pub struct MTLRenderPassDepthAttachmentDescriptor(ObjectPointer);
handle!(MTLRenderPassDepthAttachmentDescriptor);

impl MTLRenderPassDepthAttachmentDescriptor {
    pub unsafe fn set_clear_depth(&self, depth: f64) {
        msg_send![self.get_ptr(), setClearDepth:depth]
    }
    pub unsafe fn set_depth_resolve_filter(&self, filter: MTLMultisampleDepthResolveFilter) {
        msg_send![self.get_ptr(), setDepthResolveFilter:filter]
    }
}

impl Object for MTLRenderPassDepthAttachmentDescriptor {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLRenderPassDepthAttachmentDescriptor(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}

pub struct MTLRenderPassDescriptor(ObjectPointer);
handle!(MTLRenderPassDescriptor);

impl MTLRenderPassDescriptor {
    pub unsafe fn new() -> MTLRenderPassDescriptor {
        MTLRenderPassDescriptor::from_ptr(msg_send![
            class!(MTLRenderPassDescriptor),
            renderPassDescriptor
        ])
    }
    pub unsafe fn get_color_attachments(&self) -> MTLRenderPassColorAttachmentDescriptorArray {
        MTLRenderPassColorAttachmentDescriptorArray::from_ptr({
            let k = ObjectPointer(msg_send![self.get_ptr(), colorAttachments]);
            msg_send![k, retain]
        })
    }
    pub unsafe fn get_depth_attachment(&self) -> MTLRenderPassDepthAttachmentDescriptor {
        MTLRenderPassDepthAttachmentDescriptor::from_ptr({
            let k = ObjectPointer(msg_send![self.get_ptr(), depthAttachment]);
            msg_send![k, retain]
        })
    }
}

impl Object for MTLRenderPassDescriptor {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLRenderPassDescriptor(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}
