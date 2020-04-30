use crate::import_macros::*;
use crate::{handle, MTLSamplePosition, MTLSize, NSUInteger, Object, ObjectPointer, MTLCommandQueue};

mod externs {
    use crate::ObjectPointer;

    #[link(name = "Metal", kind = "framework")]
    extern "C" {
        pub fn MTLCreateSystemDefaultDevice() -> ObjectPointer;
        pub fn MTLCopyAllDevices() -> ObjectPointer;
    // TODO find a way to pass blocks into these guys that won't segfault
    // pub fn MTLCopyAllDevicesWithObserver(
    //     observer: *mut (),
    //     handler: Block<(*mut (), *mut ()), ()>,
    // ) -> *mut ();
    // pub fn MTLRemoveDeviceObserver(observer: ObjectPointer);
    }
    #[link(name = "CoreGraphics", kind = "framework")]
    extern "C" {
        pub fn CGDirectDisplayCopyCurrentMetalDevice(display_id: u32) -> ObjectPointer;
    }
}

#[allow(non_snake_case)]
/// Creates a MTLDevice representing your system's default GPU. Analogous to [this](https://developer.apple.com/documentation/metal/1433401-mtlcreatesystemdefaultdevice?language=objc).
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
                let obj: ObjectPointer = msg_send![devices, objectAtIndex: index];
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
pub unsafe fn CGDirectDisplayCopyCurrentMetalDevice(monitor_id: u32) -> MTLDevice {
    MTLDevice::from_ptr({
        let obj = externs::CGDirectDisplayCopyCurrentMetalDevice(monitor_id);
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
