use crate::import_objc_macros::*;
use crate::{DeviceCreated, NSUInteger, Object};

mod buffer;
mod texture;
pub use buffer::*;
pub use texture::*;

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

#[repr(C)]
pub struct MTLResourceOptions {
    pub bits: NSUInteger,
}

impl MTLResourceOptions {
    pub fn new() -> MTLResourceOptions {
        MTLResourceOptions { bits: 0 }
    }
    pub fn set_cpu_cache_mode(&self, mode: MTLCPUCacheMode) -> MTLResourceOptions {
        MTLResourceOptions {
            bits: self.bits | mode as NSUInteger,
        }
    }
    pub fn set_storage_mode(&self, mode: MTLStorageMode) -> MTLResourceOptions {
        MTLResourceOptions {
            bits: self.bits | ((mode as NSUInteger) << 4),
        }
    }
}

pub trait MTLResource: Object + DeviceCreated {
    unsafe fn get_cpu_cache_mode(&self) -> MTLCPUCacheMode {
        msg_send![self.get_ptr(), cpuCacheMode]
    }
    unsafe fn get_storage_mode(&self) -> MTLStorageMode {
        msg_send![self.get_ptr(), storageMode]
    }
    unsafe fn get_resource_options(&self) -> MTLResourceOptions {
        MTLResourceOptions {
            bits: msg_send![self.get_ptr(), resourceOptions],
        }
    }
    unsafe fn set_purgeable_state(&self, state: MTLPurgeableState) -> MTLPurgeableState {
        msg_send![self.get_ptr(), setPurgeableState: state]
    }
    unsafe fn get_allocated_size(&self) -> NSUInteger {
        msg_send![self.get_ptr(), allocatedSize]
    }
}
