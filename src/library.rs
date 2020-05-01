use crate::import_objc_macros::*;
use crate::{handle, DeviceCreated, MTLDevice, NSUInteger, Object, ObjectPointer};

/// A collection of MSL shader functions.
///
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
    /// Returns a new MTLFunction representing the function of the name given via
    /// [this method](https://developer.apple.com/documentation/metal/mtllibrary/1515524-newfunctionwithname?language=objc).
    ///
    /// Returns `None` if there is no function of the provided name.
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

impl DeviceCreated for MTLLibrary {
    unsafe fn get_device(&self) -> MTLDevice {
        MTLDevice::from_ptr({
            let k = ObjectPointer(msg_send![self.get_ptr(), device]);
            msg_send![k, retain]
        })
    }
}

#[repr(u64)]
/// The type of an MSL shader function.
///
/// Analogous to [this](https://developer.apple.com/documentation/metal/mtlfunctiontype?language=objc).
pub enum MTLFunctionType {
    /// A vertex function for use in an `MTLRenderPipelineState`.
    Vertex = 1,
    /// A fragment function for use in an `MTLRenderPipelineState`.
    Fragment = 2,
    /// A kernel function for use in an `MTLComputePipelineState`.
    Kernel = 3,
}

/// A single MSL shader function.
///
/// Will send to its pointer only the messages specified in the MTLFunction protocol
/// linked [here](https://developer.apple.com/documentation/metal/mtlfunction?language=objc).
pub struct MTLFunction(ObjectPointer);
handle!(MTLFunction);

impl MTLFunction {
    /// Returns the function's associated MTLFunctionType via
    /// [this method](https://developer.apple.com/documentation/metal/mtlfunction/1516042-functiontype?language=objc).
    pub unsafe fn get_function_type(&self) -> MTLFunctionType {
        msg_send![self.get_ptr(), functionType]
    }
    /// Returns the [name](https://developer.apple.com/documentation/metal/mtlfunction/1515424-name?language=objc)
    /// property of the function.
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

impl DeviceCreated for MTLFunction {
    unsafe fn get_device(&self) -> MTLDevice {
        MTLDevice::from_ptr(msg_send![self.get_ptr(), device])
    }
}

#[repr(u64)]
/// MSL versions.
///
/// Analogous to [this](https://developer.apple.com/documentation/metal/mtllanguageversion?language=objc).
pub enum MTLLanguageVersion {
    V10 = 1 << 16,
    V11 = (1 << 16) + 1,
    V12 = (1 << 16) + 2,
    V20 = 2 << 16,
    V21 = (2 << 16) + 1,
    V22 = (2 << 16) + 2,
}

/// Settings for the compilation of an MSL shader library.
///
/// Will send to its pointer only messages specified in the MTLCompileOptions interface linked
/// [here](https://developer.apple.com/documentation/metal/mtlcompileoptions?language=objc).
pub struct MTLCompileOptions(ObjectPointer);
handle!(MTLCompileOptions);

impl MTLCompileOptions {
    /// Creates a new MTLCompileOptions with standard allocation and initialization.
    pub unsafe fn new() -> MTLCompileOptions {
        MTLCompileOptions({
            let c = class!(MTLCompileOptions);
            msg_send![c, new]
        })
    }
    /// Sets the [fastMathEnabled](https://developer.apple.com/documentation/metal/mtlcompileoptions/1515914-fastmathenabled?language=objc)
    /// property.
    pub unsafe fn set_fast_math_enabled(&self, enabled: bool) {
        msg_send![self.get_ptr(), setFastMathEnabled: enabled]
    }
    /// Sets the [languageVersion](https://developer.apple.com/documentation/metal/mtlcompileoptions/1515494-languageversion?language=objc)
    /// propery.
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
