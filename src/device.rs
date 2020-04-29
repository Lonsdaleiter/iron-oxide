use crate::import_macros::*;
use crate::{handle, Object, ObjectPointer};
use block::{ConcreteBlock, Block, RcBlock};

mod externs {
    use crate::ObjectPointer;
    use block::{RcBlock, Block, ConcreteBlock};

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
}

// pub unsafe fn test() {
//     externs::MTLCopyAllDevicesWithObserver(
//         std::ptr::null_mut(),
//         ()
//         // *std::mem::transmute::<RcBlock<_>, *mut Block<*mut (), *mut ()>>(ConcreteBlock::new(|_a, _b|{}).copy())
//     );
// }

#[allow(non_snake_case)]
/// Creates a MTLDevice representing your system's default GPU. Analogous to [this](https://developer.apple.com/documentation/metal/1433401-mtlcreatesystemdefaultdevice?language=objc).
///
/// # Safety
///
/// Do *not* call this function if:
/// - iOS < 8.0
/// - macOS < 10.11
/// - Catalyst < 13.0
/// - tvOS < 9.0
pub unsafe fn MTLCreateSystemDefaultDevice() -> MTLDevice {
    MTLDevice::from_ptr({
        let obj = externs::MTLCreateSystemDefaultDevice();
        msg_send![obj, retain]
    })
}

#[allow(non_snake_case)]
/// Creates a vector of MTLDevices representing all of your system's GPUs. Analogous to [this](https://developer.apple.com/documentation/metal/1433367-mtlcopyalldevices?language=objc).
///
/// Note that the original function is not supported on iOS or tvOS. This function is.
///
/// # Safety
///
/// Do *not* call this function if:
/// - iOS < 8.0
/// - macOS < 10.11
/// - Catalyst < 13.0
/// - tvOS < 9.0
pub unsafe fn MTLCopyAllDevices() -> Vec<MTLDevice> {
    #[cfg(target_os = "macos")]
    {
        let devices = externs::MTLCopyAllDevices();
        let length: u64 = msg_send![devices, count];
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

pub struct MTLDevice(ObjectPointer);
handle!(MTLDevice);
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
    unsafe fn get_device(&self) -> MTLDevice;
}
