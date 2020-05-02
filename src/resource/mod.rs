use crate::{Object, DeviceCreated, NSUInteger};
use crate::import_objc_macros::*;

mod buffer;
pub use buffer::*;

#[repr(u64)]
/// Options for CPU cache mode defining the CPU mapping of the resource.
///
/// Analogous to [this](https://developer.apple.com/documentation/metal/mtlcpucachemode?language=objc).
pub enum MTLCPUCacheMode {
    Default = 0,
    /// Optimized for writes by the CPU. Reads may be inefficient.
    WriteCombined = 1,
}

#[repr(u64)]
/// Options for the memory location and access permissions for a resource.
///
/// Analogous to [this](https://developer.apple.com/documentation/metal/mtlstoragemode?language=objc).
pub enum MTLStorageMode {
    /// System memory is used. Both the GPU and CPU may access it.
    Shared = 0,
    /// The GPU and CPU maintain separate copies which must be explicitly synchronized. macOS only.
    Managed = 1,
    /// Only the GPU may access the resource.
    Private = 2,
    /// Only the GPU may access the resource and only during a single render pass. iOS only.
    Memoryless = 3,
}

#[repr(u64)]
/// The purgeable state of a resource.
///
/// Analogous to [this](https://developer.apple.com/documentation/metal/mtlpurgeablestate?language=objc).
pub enum MTLPurgeableState {
    /// The state is queried but doesn't change.
    KeepCurrent = 1,
    /// The contents of the resource may not be discarded.
    NonVolatile = 2,
    /// The resource may be discarded.
    Volatile = 3,
    /// The contents of the resource may be or are discarded.
    Empty = 4,
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
    /// Sets the [storageMode](https://developer.apple.com/documentation/metal/mtlresource/1515477-storagemode?language=objc)
    /// property of the resource.
    unsafe fn set_storage_mode(&self, mode: MTLStorageMode) {
        msg_send![self.get_ptr(), setStorageMode:mode]
    }
    /// Sets the purgeable state of the resource via the
    /// [setPurgeableState](https://developer.apple.com/documentation/metal/mtlresource/1515898-setpurgeablestate?language=objc)
    /// method. Also returns the prior purgeable state of the resource.
    unsafe fn set_purgeable_state(&self, state: MTLPurgeableState) -> MTLPurgeableState {
        msg_send![self.get_ptr(), setPurgeableState:state]
    }
    /// Returns the [allocatedSize](https://developer.apple.com/documentation/metal/mtlresource/2915287-allocatedsize?language=objc)
    /// property of the resource; the current allocated size in bytes.
    unsafe fn get_allocated_size(&self) -> NSUInteger {
        msg_send![self.get_ptr(), allocatedSize]
    }
}
