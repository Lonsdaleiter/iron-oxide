use crate::import_macros::*;
use crate::{
    handle, Error, MTLCommandQueue, MTLCompileOptions, MTLLibrary, MTLSamplePosition, MTLSize,
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
/// Creates an MTLDevice representing your system's default GPU. Analogous to [this](https://developer.apple.com/documentation/metal/1433401-mtlcreatesystemdefaultdevice?language=objc).
pub unsafe fn MTLCreateSystemDefaultDevice() -> MTLDevice {
    MTLDevice::from_ptr({
        let obj = externs::MTLCreateSystemDefaultDevice();
        msg_send![obj, retain]
    })
}

#[allow(non_snake_case)]
/// Creates a vector of MTLDevices representing all of your system's GPUs. Analogous to [this](https://developer.apple.com/documentation/metal/1433367-mtlcopyalldevices?language=objc).
///
/// Note that the original function is not supported on iOS or tvOS, but this function is.
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
/// Creates the MTLDevice driving the display of the given id. Analogous to [this](https://developer.apple.com/documentation/coregraphics/1493900-cgdirectdisplaycopycurrentmetald?language=objc).
///
/// If the `display_id` provided is not a valid identifier, the behavior is undefined.
pub unsafe fn CGDirectDisplayCopyCurrentMetalDevice(display_id: u32) -> MTLDevice {
    MTLDevice::from_ptr({
        let obj = externs::CGDirectDisplayCopyCurrentMetalDevice(display_id);
        msg_send![obj, retain]
    })
}

/// Represents a physical device, or GPU.
///
/// Will send to its pointer only the messages specified in the MTLDevice protocol
/// linked [here](https://developer.apple.com/documentation/metal/mtldevice?language=objc).
pub struct MTLDevice(ObjectPointer);
handle!(MTLDevice);

impl MTLDevice {
    /// Returns the [name](https://developer.apple.com/documentation/metal/mtldevice/1433359-name?language=objc)
    /// property of the device.
    pub unsafe fn get_name(&self) -> &str {
        let string = ObjectPointer(msg_send![self.get_ptr(), name]);
        let bytes: *const u8 = msg_send![string, UTF8String];
        let len: NSUInteger = msg_send![string, length];
        let bytes = std::slice::from_raw_parts(bytes, len as usize);
        std::str::from_utf8(bytes).unwrap()
    }
    /// Returns the [headless](https://developer.apple.com/documentation/metal/mtldevice/1433377-headless?language=objc)
    /// property of the device.
    ///
    /// This property reflects whether the GPU is attached to a particular display.
    pub unsafe fn is_headless(&self) -> bool {
        msg_send![self.get_ptr(), isHeadless]
    }
    /// Returns the [lowPower](https://developer.apple.com/documentation/metal/mtldevice/1433409-lowpower?language=objc)
    /// property of the device.
    ///
    /// If the GPU is integrated, this returns true. If it is discrete, it returns false.
    pub unsafe fn is_low_power(&self) -> bool {
        msg_send![self.get_ptr(), isLowPower]
    }
    /// Returns the [removable](https://developer.apple.com/documentation/metal/mtldevice/2889851-removable?language=objc)
    /// property of the device.
    pub unsafe fn is_removable(&self) -> bool {
        msg_send![self.get_ptr(), isRemovable]
    }
    /// Returns the [registryID](https://developer.apple.com/documentation/metal/mtldevice/2915737-registryid?language=objc)
    /// property of the device.
    pub unsafe fn get_registry_id(&self) -> u64 {
        msg_send![self.get_ptr(), registryID]
    }
    /// Returns the [recommendedMaxWorkingSetSize](https://developer.apple.com/documentation/metal/mtldevice/2369280-recommendedmaxworkingsetsize?language=objc)
    /// property of the device.
    pub unsafe fn get_recommended_max_working_set_size(&self) -> u64 {
        msg_send![self.get_ptr(), recommendedMaxWorkingSetSize]
    }
    /// Returns the [currentAllocatedSize](https://developer.apple.com/documentation/metal/mtldevice/2915745-currentallocatedsize?language=objc)
    /// property of the device.
    pub unsafe fn get_current_allocated_size(&self) -> NSUInteger {
        msg_send![self.get_ptr(), currentAllocatedSize]
    }
    /// Returns the [maxThreadgroupMemoryLength](https://developer.apple.com/documentation/metal/mtldevice/2877429-maxthreadgroupmemorylength?language=objc)
    /// property of the device.
    pub unsafe fn get_max_threadgroup_memory_length(&self) -> NSUInteger {
        msg_send![self.get_ptr(), maxThreadgroupMemoryLength]
    }
    /// Returns the [maxThreadsPerThreadgroup](https://developer.apple.com/documentation/metal/mtldevice/1433393-maxthreadsperthreadgroup?language=objc)
    /// property of the device.
    pub unsafe fn get_max_threads_per_threadgroup(&self) -> MTLSize {
        msg_send![self.get_ptr(), maxThreadsPerThreadgroup]
    }
    /// Returns the [programmableSamplePositionsSupported](https://developer.apple.com/documentation/metal/mtldevice/2866117-programmablesamplepositionssuppo?language=objc)
    /// property of the device.
    pub unsafe fn are_programmable_sample_positions_supported(&self) -> bool {
        msg_send![self.get_ptr(), areProgrammableSamplePositionsSupported]
    }
    /// Calls the [getDefaultSamplePositions](https://developer.apple.com/documentation/metal/mtldevice/2866120-getdefaultsamplepositions?language=objc)
    /// method.
    pub unsafe fn get_default_sample_positions(&self, count: NSUInteger) -> MTLSamplePosition {
        let mut pos = MTLSamplePosition { x: 0.0, y: 0.0 };
        let _: () = msg_send![self.get_ptr(), getDefaultSamplePositions: &mut pos count: count];
        pos
    }
    /// Returns the [rasterOrderGroupsSupported](https://developer.apple.com/documentation/metal/mtldevice/2887285-rasterordergroupssupported?language=objc)
    /// property of the device.
    pub unsafe fn are_raster_order_groups_supported(&self) -> bool {
        msg_send![self.get_ptr(), areRasterOrderGroupsSupported]
    }
    /// Returns the [depth24Stencil8PixelFormatSupported](https://developer.apple.com/documentation/metal/mtldevice/1433371-depth24stencil8pixelformatsuppor?language=objc)
    /// property of the device.
    pub unsafe fn is_d24_s8_pixel_format_supported(&self) -> bool {
        msg_send![self.get_ptr(), isDepth24Stencil8PixelFormatSupported]
    }
    /// Creates a new [MTLCommandQueue](https://developer.apple.com/documentation/metal/mtlcommandqueue?language=objc)
    /// via [this method](https://developer.apple.com/documentation/metal/mtldevice/1433388-newcommandqueue?language=objc).
    pub unsafe fn new_command_queue(&self) -> MTLCommandQueue {
        MTLCommandQueue::from_ptr({
            let k = ObjectPointer(msg_send![self.get_ptr(), newCommandQueue]);
            msg_send![k, retain]
        })
    }
    /// Creates a new [MTLCommandQueue](https://developer.apple.com/documentation/metal/mtlcommandqueue?language=objc)
    /// via [this method](https://developer.apple.com/documentation/metal/mtldevice/1433433-newcommandqueuewithmaxcommandbuf?language=objc).
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
    /// Creates a new [MTLLibrary](https://developer.apple.com/documentation/metal/mtllibrary?language=objc)
    /// via [this method](https://developer.apple.com/documentation/metal/mtldevice/1433391-newlibrarywithdata?language=objc).
    pub unsafe fn new_library_with_data(&self, data: &[u8]) -> Error<MTLLibrary> {
        use externs::*;

        let mut err: *mut objc::runtime::Object = std::ptr::null_mut();

        let dispatch_data = dispatch_data_create(
            data.as_ptr() as *const c_void,
            data.len() as usize,
            &_dispatch_main_q as *const _ as *mut objc::runtime::Object,
            std::ptr::null(),
        );

        let lib = ObjectPointer(msg_send![self.0, newLibraryWithData:dispatch_data error:&mut err]);
        dispatch_release(dispatch_data);

        if !err.is_null() {
            let info = ObjectPointer(msg_send![err, localizedDescription]);
            let bytes: *const u8 = msg_send![info, UTF8String];
            let len: NSUInteger = msg_send![info, length];
            let bytes = std::slice::from_raw_parts(bytes, len as usize);
            let st = std::str::from_utf8(bytes).unwrap();

            if lib.0.is_null() {
                Error::Error(st)
            } else {
                Error::Warn(MTLLibrary::from_ptr(lib), st)
            }
        } else {
            Error::None(MTLLibrary::from_ptr(lib))
        }
    }
    /// Creates a new [MTLLibrary](https://developer.apple.com/documentation/metal/mtllibrary?language=objc)
    /// via [this method](https://developer.apple.com/documentation/metal/mtldevice/1433431-newlibrarywithsource?language=objc).
    pub unsafe fn new_library_with_source(
        &self,
        source: &str,
        options: &MTLCompileOptions,
    ) -> Error<MTLLibrary> {
        let cls = class!(NSString);
        let bytes = source.as_ptr();
        let st = ObjectPointer(msg_send![cls, alloc]);
        let st = ObjectPointer(msg_send![
           st,
           initWithBytes:bytes
           length:source.len()
           encoding:4 // UTF-8
        ]);
        let mut error: *mut objc::runtime::Object = std::ptr::null_mut();

        let lib = ObjectPointer(msg_send![
            self.get_ptr(),
            newLibraryWithSource:st
            options:options.get_ptr()
            error:&mut error
        ]);

        if !error.is_null() {
            let info = ObjectPointer(msg_send![error, localizedDescription]);
            let bytes: *const u8 = msg_send![info, UTF8String];
            let len: NSUInteger = msg_send![info, length];
            let bytes = std::slice::from_raw_parts(bytes, len as usize);
            let st = std::str::from_utf8(bytes).unwrap();

            if !lib.0.is_null() {
                Error::Warn(MTLLibrary::from_ptr(lib), st)
            } else {
                Error::Error(st)
            }
        } else {
            Error::None(MTLLibrary::from_ptr(lib))
        }
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

pub trait DeviceCreated {
    /// Returns a reference to the device which created this object.
    unsafe fn get_device(&self) -> MTLDevice;
}
