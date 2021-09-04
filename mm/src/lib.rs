//! Memory management library.

#![no_std]

/// Represents a physical memory address.
#[derive(Debug, Copy, Clone)]
pub struct PhysAddr(pub u64);

/// Represents a virtual memory address.
#[derive(Debug, Copy, Clone)]
pub struct VirtAddr(pub u64);
