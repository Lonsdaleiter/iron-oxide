use crate::{Object, DeviceCreated};
use crate::import_objc_macros::*;

#[repr(u64)]
/// Options for CPU cache mode defining the CPU mapping of the resource.
///
/// Analogous to [this](https://developer.apple.com/documentation/metal/mtlcpucachemode?language=objc).
pub enum MTLCPUCacheMode {
    Default = 0,
    WriteCombined = 1,
}

/// An allocation of GPU accessible memory. Implemented only on device created objects.
///
/// Will send to its pointer only the messages specified in the MTLResource protocol
/// linked [here](https://developer.apple.com/documentation/metal/mtlresource?language=objc).
pub trait MTLResource: Object + DeviceCreated {
    /// Sets the [cpuCacheMode](https://developer.apple.com/documentation/metal/mtlresource/1516127-cpucachemode?language=objc)
    /// property of the resource.
    unsafe fn set_cpu_cache_mode(&self, mode: MTLCPUCacheMode) {
        msg_send![self.get_ptr(), setCpuCacheMode:mode]
    }
}
