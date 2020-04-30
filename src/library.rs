use crate::import_macros::*;
use crate::{handle, DeviceCreated, MTLDevice, NSUInteger, Object, ObjectPointer};

/// A collection of shader functions.
/// Will send to its pointer only the messages specified in the MTLLibrary protocol
/// linked [here](https://developer.apple.com/documentation/metal/mtllibrary?language=objc).
pub struct MTLLibrary(ObjectPointer);
handle!(MTLLibrary);

impl MTLLibrary {
    /// Returns the names of all public functions (kernel, vertex, fragment) stored in the library
    /// via [this method](https://developer.apple.com/documentation/metal/mtllibrary/1515651-functionnames?language=objc).
    pub unsafe fn get_function_names(&self) -> Vec<&str> {
        let names = ObjectPointer(msg_send![self.get_ptr(), functionNames]);
        let length: NSUInteger = msg_send![names, count];
        (0..length)
            .map(|index| {
                let obj = ObjectPointer(msg_send![names, objectAtIndex: index]);
                let obj = ObjectPointer(msg_send![obj, retain]);
                let bytes: *const u8 = msg_send![obj, UTF8String];
                let len: NSUInteger = msg_send![obj, length];
                let bytes = std::slice::from_raw_parts(bytes, len as usize);
                std::str::from_utf8(bytes).unwrap()
            })
            .collect()
    }
}

impl Object for MTLLibrary {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLLibrary(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}

impl DeviceCreated for MTLLibrary {
    unsafe fn get_device(&self) -> MTLDevice {
        MTLDevice::from_ptr({
            let k = ObjectPointer(msg_send![self.get_ptr(), device]);
            msg_send![k, retain]
        })
    }
}
