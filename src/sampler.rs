use crate::{ObjectPointer, handle, Object};

/// Describes a sampler which samples textures.
///
/// Will send to its pointer only messages specified in theMTLSamplerDescriptor interface
/// linked [here](https://developer.apple.com/documentation/metal/mtlsamplerdescriptor?language=objc).
pub struct MTLSamplerDescriptor(ObjectPointer);
handle!(MTLSamplerDescriptor);

impl Object for MTLSamplerDescriptor {
    unsafe fn from_ptr(ptr: ObjectPointer) -> Self where
        Self: Sized {
        MTLSamplerDescriptor(ptr)
    }

    fn get_ptr(&self) -> ObjectPointer {
        self.0
    }
}
