use crate::import_objc_macros::*;
use crate::{
    handle, MTLBuffer, MTLCommandEncoder, MTLSamplerState, MTLSize, MTLTexture, NSRange,
    NSUInteger, NSUIntegerRange, Object, ObjectPointer,
};
use std::os::raw::c_void;

pub struct MTLComputeCommandEncoder(ObjectPointer);
handle!(MTLComputeCommandEncoder);

impl MTLComputeCommandEncoder {
    pub unsafe fn set_buffer(&self, buffer: &MTLBuffer, offset: NSUInteger, index: NSUInteger) {
        msg_send![self.get_ptr(), setBuffer:buffer.get_ptr() offset:offset atIndex:index]
    }
    pub unsafe fn set_buffers(
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

        msg_send![self.get_ptr(), setBuffers:pointers offsets:offsets.as_ptr() withRange:range]
    }
    pub unsafe fn set_buffer_offset(&self, offset: NSUInteger, index: NSUInteger) {
        msg_send![self.get_ptr(), setBufferOffset:offset atIndex:index]
    }
    pub unsafe fn set_bytes(&self, bytes: *const c_void, length: NSUInteger, index: NSUInteger) {
        msg_send![self.get_ptr(), setBytes:bytes length:length atIndex:index]
    }
    pub unsafe fn set_sampler_state(&self, sampler: &MTLSamplerState, index: NSUInteger) {
        msg_send![self.get_ptr(), setSamplerState:sampler.get_ptr() atIndex:index]
    }
    pub unsafe fn set_sampler_state_clamp(
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
    pub unsafe fn set_sampler_states(&self, samplers: &[MTLSamplerState], range: NSUIntegerRange) {
        let pointers = samplers
            .iter()
            .map(|sampler| sampler.get_ptr())
            .collect::<Vec<_>>();
        let pointers = pointers.as_slice().as_ptr();
        let range = NSRange {
            location: range.start,
            length: range.end - range.start,
        };
        msg_send![self.get_ptr(), setSamplerStates:pointers withRange:range]
    }
    pub unsafe fn set_texture(&self, texture: &MTLTexture, index: NSUInteger) {
        msg_send![self.get_ptr(), setTexture:texture.get_ptr() atIndex:index]
    }
    pub unsafe fn set_textures(&self, textures: &[MTLTexture], range: NSUIntegerRange) {
        let pointers = textures
            .iter()
            .map(|texture| texture.get_ptr())
            .collect::<Vec<_>>();
        let pointers = pointers.as_slice().as_ptr();
        let range = NSRange {
            location: range.start,
            length: range.end - range.start,
        };
        msg_send![self.get_ptr(), setTextures:pointers withRange:range]
    }
    pub unsafe fn set_threadgroup_memory_length(&self, length: NSUInteger, index: NSUInteger) {
        msg_send![self.get_ptr(), setThreadgroupMemoryLength:length atIndex:index]
    }
    pub unsafe fn dispatch_threadgroups(
        &self,
        threadgroups_per_grid: MTLSize,
        threads_per_threadgroup: MTLSize,
    ) {
        msg_send![
            self.get_ptr(),
            dispatchThreadgroups:threadgroups_per_grid
            threadsPerThreadgroup:threads_per_threadgroup
        ]
    }
    pub unsafe fn dispatch_threads(
        &self,
        threads_per_grid: MTLSize,
        threads_per_threadgroup: MTLSize,
    ) {
        msg_send![
            self.get_ptr(),
            dispatchThreads:threads_per_grid
            threadsPerThreadgroup:threads_per_threadgroup
        ]
    }
}

impl MTLCommandEncoder for MTLComputeCommandEncoder {}

impl Object for MTLComputeCommandEncoder {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLComputeCommandEncoder(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}
