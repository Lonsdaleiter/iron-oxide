use crate::import_macros::*;
use crate::{handle, NSUInteger, Object, ObjectPointer};

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
    /// Returns the [name](https://developer.apple.com/documentation/metal/mtldevice/1433359-name?language=objc) of the device.
    pub unsafe fn get_name(&self) -> &str {
        let string = ObjectPointer(msg_send![self.get_ptr(), name]);
        let bytes: *const u8 = msg_send![string, UTF8String];
        let len: NSUInteger = msg_send![string, length];
        let bytes = std::slice::from_raw_parts(bytes, len as usize);
        std::str::from_utf8(bytes).unwrap()
    }
    /// Returns the [headless](https://developer.apple.com/documentation/metal/mtldevice/1433377-headless?language=objc) property of the device.
    ///
    /// This property reflects whether the GPU is attached to a particular display.
    pub unsafe fn is_headless(&self) -> bool {
        msg_send![self.get_ptr(), isHeadless]
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
    ///
    /// # Safety
    ///
    /// Do *not* call this function if:
    /// - iOS < 8.0
    /// - macOS < 10.11
    /// - Catalyst < 13.0
    /// - tvOS < 9.0
    /// - you run any other OS
    unsafe fn get_device(&self) -> MTLDevice;
}
