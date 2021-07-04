//! Memory management library.

#![no_std]

use core::ops::Deref;

/// Represents a physical memory address.
#[derive(Debug, Copy, Clone)]
pub struct PhysAddr(u64);

impl From<u64> for PhysAddr {
    fn from(addr: u64) -> Self {
        PhysAddr(addr)
    }
}

impl Deref for PhysAddr {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Represents a virtual memory address.
#[derive(Debug, Copy, Clone)]
pub struct VirtAddr(u64);

impl From<u64> for VirtAddr {
    fn from(addr: u64) -> Self {
        VirtAddr(addr)
    }
}

impl Deref for VirtAddr {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
