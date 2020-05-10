use crate::import_objc_macros::*;
use crate::{handle, Object, ObjectPointer, MTLTexture, NSUInteger};

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
        msg_send![self.get_ptr(), setLevel:level]
    }
    unsafe fn set_slice(&self, slice: NSUInteger) {
        msg_send![self.get_ptr(), setSlice:slice]
    }
    unsafe fn set_depth_plane(&self, plane: NSUInteger) {
        msg_send![self.get_ptr(), setDepthPlane:plane]
    }
    unsafe fn set_load_action(&self, action: MTLLoadAction) {
        msg_send![self.get_ptr(), setLoadAction:action]
    }
    unsafe fn set_store_action(&self, action: MTLStoreAction) {
        msg_send![self.get_ptr(), setStoreAction:action]
    }
    unsafe fn set_store_action_options(&self, options: MTLStoreActionOptions) {
        msg_send![self.get_ptr(), setStoreActionOptions:options]
    }
    unsafe fn set_resolve_texture(&self, texture: &MTLTexture) {
        msg_send![self.get_ptr(), setResolveTexture:texture.get_ptr()]
    }
    unsafe fn set_resolve_level(&self, level: NSUInteger) {
        msg_send![self.get_ptr(), setResolveLevel:level]
    }
    unsafe fn set_resolve_slice(&self, slice: NSUInteger) {
        msg_send![self.get_ptr(), setResolveSlice:slice]
    }
    unsafe fn set_resolve_depth_plane(&self, plane: NSUInteger) {
        msg_send![self.get_ptr(), setResolveDepthPlane:plane]
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
