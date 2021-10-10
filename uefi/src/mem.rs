//! This module provides memory management primitives in the context of UEFI.

use crate::{BootServices, EfiMemoryDescriptor, Error, MemoryType, Status};
use range::{Range, RangeSet};

/// Returns a tuple with a `RangeSet` containing the available memory
/// blocks and the map key of the current memory map. This tuple has the
/// form `(available_memory, map_key)`.
pub fn get_available_memory(
    boot_services: &BootServices,
) -> Result<(RangeSet, usize), Error> {
    // Allocate the arguments of the boot service.
    const BUFFER_SIZE: usize = 1024 * 32;
    let mut memory_map_size = BUFFER_SIZE;
    let mut memory_map = [0u8; BUFFER_SIZE];
    let mut map_key = 0usize;
    let mut descriptor_size = 0usize;
    let mut descriptor_version = 0u32;

    // Call `EFI_BOOT_SERVICES.GetMemoryMap()`.
    let status = (boot_services.boot_services.get_memory_map)(
        &mut memory_map_size,
        memory_map.as_mut_ptr(),
        &mut map_key,
        &mut descriptor_size,
        &mut descriptor_version,
    );

    // Return with error in the case of warning and error status codes.
    match status.into() {
        Status::Success => {}
        Status::Warning(warn) => return Err(warn.into()),
        Status::Error(err) => return Err(err.into()),
    }

    // Fill the `RangeSet` to be returned.
    let mut ret = RangeSet::new();
    let mut idx = 0;
    while (idx + 1) * descriptor_size <= memory_map_size {
        // Read the `EfiMemoryDescriptor`.
        let descriptor = unsafe {
            let descriptor_ptr = memory_map.as_ptr().add(idx * descriptor_size)
                as *const EfiMemoryDescriptor;
            core::ptr::read(descriptor_ptr)
        };

        // Add the memory block into the `RangeSet` if the memory is
        // avaiable.
        match MemoryType::from(descriptor.memory_type) {
            MemoryType::BootServicesCode
            | MemoryType::BootServicesData
            | MemoryType::ConventionalMemory
            | MemoryType::ACPIReclaimMemory => {
                let start = descriptor.physical_start.0;
                let size = descriptor.number_of_pages * 0x1000;
                let end = start + size - 1;
                ret.insert(Range::new(start, end)?)?;
            }
            _ => {}
        }

        idx += 1;
    }

    Ok((ret, map_key))
}
