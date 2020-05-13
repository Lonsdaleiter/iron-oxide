use crate::import_objc_macros::*;
use crate::{
    handle, MTLBuffer, MTLCommandQueue, MTLCompileOptions, MTLComputePipelineState,
    MTLDepthStencilDescriptor, MTLDepthStencilState, MTLFunction, MTLLibrary,
    MTLRenderPipelineDescriptor, MTLRenderPipelineState, MTLResourceOptions, MTLSamplePosition,
    MTLSamplerDescriptor, MTLSamplerState, MTLSize, MTLTexture, MTLTextureDescriptor, NSError,
    NSUInteger, Object, ObjectPointer,
};
use std::os::raw::c_void;

mod externs {
    use crate::ObjectPointer;
    use std::os::raw::c_void;

    #[link(name = "Metal", kind = "framework")]
    extern "C" {
        pub fn MTLCreateSystemDefaultDevice() -> ObjectPointer;
        pub fn MTLCopyAllDevices() -> ObjectPointer;
    }
    #[link(name = "CoreGraphics", kind = "framework")]
    extern "C" {
        pub fn CGDirectDisplayCopyCurrentMetalDevice(display_id: u32) -> ObjectPointer;
    }

    #[cfg_attr(
        any(target_os = "macos", target_os = "ios"),
        link(name = "System", kind = "dylib")
    )]
    #[cfg_attr(
        not(any(target_os = "macos", target_os = "ios")),
        link(name = "dispatch", kind = "dylib")
    )]
    #[allow(improper_ctypes)]
    extern "C" {
        pub static _dispatch_main_q: *mut objc::runtime::Object;

        pub fn dispatch_data_create(
            buffer: *const c_void,
            size: usize,
            queue: *mut objc::runtime::Object,
            destructor: *const c_void,
        ) -> *mut objc::runtime::Object;
        pub fn dispatch_release(object: *mut objc::runtime::Object);
    }
}

#[allow(non_snake_case)]
pub unsafe fn MTLCreateSystemDefaultDevice() -> MTLDevice {
    MTLDevice::from_ptr({
        let obj = externs::MTLCreateSystemDefaultDevice();
        msg_send![obj, retain]
    })
}

#[allow(non_snake_case)]
pub unsafe fn MTLCopyAllDevices() -> Vec<MTLDevice> {
    #[cfg(target_os = "macos")]
    {
        let devices = externs::MTLCopyAllDevices();
        let length: NSUInteger = msg_send![devices, count];
        (0..length)
            .map(|index| {
                let obj = ObjectPointer(msg_send![devices, objectAtIndex: index]);
                MTLDevice::from_ptr(msg_send![obj, retain])
            })
            .collect()
    }
    #[cfg(target_os = "ios")]
    {
        vec![MTLCreateSystemDefaultDevice()]
    }
}

#[allow(non_snake_case)]
pub unsafe fn CGDirectDisplayCopyCurrentMetalDevice(display_id: u32) -> MTLDevice {
    MTLDevice::from_ptr({
        let obj = externs::CGDirectDisplayCopyCurrentMetalDevice(display_id);
        msg_send![obj, retain]
    })
}

pub struct MTLDevice(ObjectPointer);
handle!(MTLDevice);

impl MTLDevice {
    pub unsafe fn get_name(&self) -> &str {
        let string = ObjectPointer(msg_send![self.get_ptr(), name]);
        let bytes: *const u8 = msg_send![string, UTF8String];
        let len: NSUInteger = msg_send![string, length];
        let bytes = std::slice::from_raw_parts(bytes, len as usize);
        std::str::from_utf8(bytes).unwrap()
    }
    pub unsafe fn is_headless(&self) -> bool {
        msg_send![self.get_ptr(), isHeadless]
    }
    pub unsafe fn is_low_power(&self) -> bool {
        msg_send![self.get_ptr(), isLowPower]
    }
    pub unsafe fn is_removable(&self) -> bool {
        msg_send![self.get_ptr(), isRemovable]
    }
    pub unsafe fn get_registry_id(&self) -> u64 {
        msg_send![self.get_ptr(), registryID]
    }
    pub unsafe fn get_recommended_max_working_set_size(&self) -> u64 {
        msg_send![self.get_ptr(), recommendedMaxWorkingSetSize]
    }
    pub unsafe fn get_current_allocated_size(&self) -> NSUInteger {
        msg_send![self.get_ptr(), currentAllocatedSize]
    }
    pub unsafe fn get_max_threadgroup_memory_length(&self) -> NSUInteger {
        msg_send![self.get_ptr(), maxThreadgroupMemoryLength]
    }
    pub unsafe fn get_max_threads_per_threadgroup(&self) -> MTLSize {
        msg_send![self.get_ptr(), maxThreadsPerThreadgroup]
    }
    pub unsafe fn are_programmable_sample_positions_supported(&self) -> bool {
        msg_send![self.get_ptr(), areProgrammableSamplePositionsSupported]
    }
    pub unsafe fn get_default_sample_positions(&self, count: NSUInteger) -> MTLSamplePosition {
        let mut pos = MTLSamplePosition { x: 0.0, y: 0.0 };
        let _: () = msg_send![self.get_ptr(), getDefaultSamplePositions: &mut pos count: count];
        pos
    }
    pub unsafe fn are_raster_order_groups_supported(&self) -> bool {
        msg_send![self.get_ptr(), areRasterOrderGroupsSupported]
    }
    pub unsafe fn is_d24_s8_pixel_format_supported(&self) -> bool {
        msg_send![self.get_ptr(), isDepth24Stencil8PixelFormatSupported]
    }
    pub unsafe fn new_command_queue(&self) -> MTLCommandQueue {
        MTLCommandQueue::from_ptr({
            let k = ObjectPointer(msg_send![self.get_ptr(), newCommandQueue]);
            msg_send![k, retain]
        })
    }
    pub unsafe fn new_command_queue_with_max_command_buffer_count(
        &self,
        count: NSUInteger,
    ) -> MTLCommandQueue {
        MTLCommandQueue::from_ptr({
            let k = ObjectPointer(msg_send![
                self.get_ptr(),
                newCommandQueueWithMaxCommandBufferCount: count
            ]);
            msg_send![k, retain]
        })
    }
    pub unsafe fn new_library_with_data(&self, data: &[u8]) -> Result<MTLLibrary, NSError> {
        use externs::*;

        let mut err = ObjectPointer(std::ptr::null_mut());

        let dispatch_data = dispatch_data_create(
            data.as_ptr() as *const c_void,
            data.len() as usize,
            &_dispatch_main_q as *const _ as *mut objc::runtime::Object,
            std::ptr::null(),
        );

        let lib = ObjectPointer(msg_send![self.0, newLibraryWithData:dispatch_data error:&mut err]);
        dispatch_release(dispatch_data);

        if !err.0.is_null() {
            Err(NSError::from_ptr(err))
        } else {
            Ok(MTLLibrary::from_ptr(lib))
        }
    }
    pub unsafe fn new_library_with_source(
        &self,
        source: &str,
        options: &MTLCompileOptions,
    ) -> Result<MTLLibrary, NSError> {
        let cls = class!(NSString);
        let bytes = source.as_ptr();
        let st = ObjectPointer(msg_send![cls, alloc]);
        let st = ObjectPointer(msg_send![
           st,
           initWithBytes:bytes
           length:source.len()
           encoding:4 // UTF-8
        ]);
        let mut err = ObjectPointer(std::ptr::null_mut());

        let lib = ObjectPointer(msg_send![
            self.get_ptr(),
            newLibraryWithSource:st
            options:options.get_ptr()
            error:&mut err
        ]);

        if !err.0.is_null() {
            Err(NSError::from_ptr(err))
        } else {
            Ok(MTLLibrary::from_ptr(lib))
        }
    }
    pub unsafe fn new_render_pipeline_state_with_descriptor(
        &self,
        desc: &MTLRenderPipelineDescriptor,
    ) -> Result<MTLRenderPipelineState, NSError> {
        let mut err = ObjectPointer(std::ptr::null_mut());
        let b = ObjectPointer(msg_send![
            self.get_ptr(),
            newRenderPipelineStateWithDescriptor:desc.get_ptr()
            error:&mut err
        ]);
        if !err.0.is_null() {
            Err(NSError::from_ptr(err))
        } else {
            Ok(MTLRenderPipelineState::from_ptr(b))
        }
    }
    pub unsafe fn new_compute_pipeline_state_with_function(
        &self,
        function: &MTLFunction,
    ) -> Result<MTLComputePipelineState, NSError> {
        let mut err = ObjectPointer(std::ptr::null_mut());
        let b = ObjectPointer(msg_send![
            self.get_ptr(),
            newComputePipelineStateWithFunction:function.get_ptr()
            error:&mut err
        ]);
        if err.0.is_null() {
            Ok(MTLComputePipelineState::from_ptr(b))
        } else {
            Err(NSError::from_ptr(err))
        }
    }
    pub unsafe fn get_max_buffer_length(&self) -> NSUInteger {
        msg_send![self.get_ptr(), maxBufferLength]
    }
    pub unsafe fn new_buffer_with_length(
        &self,
        length: NSUInteger,
        options: MTLResourceOptions,
    ) -> MTLBuffer {
        MTLBuffer::from_ptr(
            msg_send![self.get_ptr(), newBufferWithLength:length options:options.bits],
        )
    }
    pub unsafe fn new_buffer_with_bytes(
        &self,
        bytes: *const c_void,
        length: NSUInteger,
        options: MTLResourceOptions,
    ) -> MTLBuffer {
        MTLBuffer::from_ptr(
            msg_send![self.get_ptr(), newBufferWithBytes:bytes length:length options:options.bits],
        )
    }
    pub unsafe fn supports_texture_sample_count(&self, count: NSUInteger) -> bool {
        msg_send![self.get_ptr(), supportsTextureSampleCount: count]
    }
    pub unsafe fn new_texture_with_descriptor(
        &self,
        descriptor: &MTLTextureDescriptor,
    ) -> MTLTexture {
        MTLTexture::from_ptr(
            msg_send![self.get_ptr(), newTextureWithDescriptor:descriptor.get_ptr()],
        )
    }
    pub unsafe fn new_sampler_state_with_descriptor(
        &self,
        desc: &MTLSamplerDescriptor,
    ) -> MTLSamplerState {
        MTLSamplerState::from_ptr(
            msg_send![self.get_ptr(), newSamplerStateWithDescriptor:desc.get_ptr()],
        )
    }
    pub unsafe fn new_depth_stencil_state_with_descriptor(
        &self,
        desc: &MTLDepthStencilDescriptor,
    ) -> MTLDepthStencilState {
        MTLDepthStencilState::from_ptr(
            msg_send![self.get_ptr(), newDepthStencilStateWithDescriptor:desc.get_ptr()],
        )
    }
}

impl Object for MTLDevice {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self {
        MTLDevice(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}

pub trait DeviceCreated: Object {
    unsafe fn get_device(&self) -> Option<MTLDevice> {
        use crate::import_objc_macros::*;
        let d = ObjectPointer(msg_send![self.get_ptr(), device]);
        if d.0.is_null() {
            None
        } else {
            Some(MTLDevice::from_ptr(d))
        }
    }
}
