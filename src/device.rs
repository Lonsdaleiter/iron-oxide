use crate::{ObjectPointer, Object};
use crate::import_macros::*;

mod externs {
    use crate::ObjectPointer;

    #[link(name = "Metal", kind = "framework")]
    extern "C" {
        pub fn MTLCreateSystemDefaultDevice() -> ObjectPointer;
        pub fn MTLCopyAllDevices() -> ObjectPointer;
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
    let devices = externs::MTLCopyAllDevices();
    let length: u64 = msg_send![devices, count];
    (0..length).map(|index|{
        let obj: ObjectPointer = msg_send![devices, objectAtIndex:index];
        MTLDevice::from_ptr(msg_send![obj, retain])
    }).collect()
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
