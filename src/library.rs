use crate::import_objc_macros::*;
use crate::{handle, DeviceCreated, NSUInteger, Object, ObjectPointer};

pub struct MTLLibrary(ObjectPointer);
handle!(MTLLibrary);

impl MTLLibrary {
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
    pub unsafe fn new_function_with_name(&self, name: &str) -> Option<MTLFunction> {
        let cls = class!(NSString);
        let bytes = name.as_ptr();
        let st = ObjectPointer(msg_send![cls, alloc]);
        let st = ObjectPointer(msg_send![
           st,
           initWithBytes:bytes
           length:name.len()
           encoding:4 // UTF-8
        ]);
        let obj = ObjectPointer(msg_send![self.get_ptr(), newFunctionWithName: st]);
        if obj.0.is_null() {
            None
        } else {
            Some(MTLFunction::from_ptr(obj))
        }
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

impl DeviceCreated for MTLLibrary {}

#[repr(u64)]
pub enum MTLFunctionType {
    Vertex = 1,
    Fragment = 2,
    Kernel = 3,
}

pub struct MTLFunction(ObjectPointer);
handle!(MTLFunction);

impl MTLFunction {
    pub unsafe fn get_function_type(&self) -> MTLFunctionType {
        msg_send![self.get_ptr(), functionType]
    }
    pub unsafe fn get_name(&self) -> &str {
        let string = ObjectPointer(msg_send![self.get_ptr(), name]);
        let bytes: *const u8 = msg_send![string, UTF8String];
        let len: NSUInteger = msg_send![string, length];
        let bytes = std::slice::from_raw_parts(bytes, len as usize);
        std::str::from_utf8(bytes).unwrap()
    }
}

impl Object for MTLFunction {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLFunction(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}

impl DeviceCreated for MTLFunction {}

#[repr(u64)]
pub enum MTLLanguageVersion {
    V10 = 1 << 16,
    V11 = (1 << 16) + 1,
    V12 = (1 << 16) + 2,
    V20 = 2 << 16,
    V21 = (2 << 16) + 1,
    V22 = (2 << 16) + 2,
}

pub struct MTLCompileOptions(ObjectPointer);
handle!(MTLCompileOptions);

impl MTLCompileOptions {
    pub unsafe fn new() -> MTLCompileOptions {
        MTLCompileOptions({
            let c = class!(MTLCompileOptions);
            msg_send![c, new]
        })
    }
    pub unsafe fn set_fast_math_enabled(&self, enabled: bool) {
        msg_send![self.get_ptr(), setFastMathEnabled: enabled]
    }
    pub unsafe fn set_language_version(&self, version: MTLLanguageVersion) {
        msg_send![self.get_ptr(), setLanguageVersion: version]
    }
}

impl Object for MTLCompileOptions {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self
    where
        Self: Sized,
    {
        MTLCompileOptions(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}
