use crate::import_objc_macros::*;
use crate::{DeviceCreated, NSUInteger, Object};

mod buffer;
mod texture;
pub use buffer::*;
pub use texture::*;

#[repr(u64)]
pub enum MTLCPUCacheMode {
    Default = 0,
    WriteCombined = 1,
}

#[repr(u64)]
pub enum MTLStorageMode {
    Shared = 0,
    Managed = 1,
    Private = 2,
    Memoryless = 3,
}

#[repr(u64)]
pub enum MTLPurgeableState {
    KeepCurrent = 1,
    NonVolatile = 2,
    Volatile = 3,
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
