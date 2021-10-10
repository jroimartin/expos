//! expOS is a tiny Operating System focused on experimentation.

#![no_std]
#![cfg_attr(not(test), no_main)]
#![feature(panic_info_message)]

use range::RangeSet;
use uefi::acpi;

#[cfg(not(test))]
mod panic;

mod serial;

struct BootInfo {
    available_memory: RangeSet,
    acpi_madt: acpi::Madt,
}

/// UEFI entry point.
#[no_mangle]
extern "C" fn efi_main(
    image_handle: uefi::Handle,
    system_table_ptr: uefi::Ptr,
) -> ! {
    // Initialize serial.
    serial::init_serial();

    // Parse UEFI's system table.
    let system_table =
        unsafe { uefi::SystemTable::new(system_table_ptr).unwrap() };

    // Get LAPIC data.
    let config_tables = system_table.configuration_tables().unwrap();
    let rsdp20_ptr = config_tables.acpi_rsdp20_ptr().unwrap();
    let rsdp20 = unsafe { acpi::Rsdp20::new(rsdp20_ptr).unwrap() };
    let xsdt = rsdp20.xsdt().unwrap();
    let madt = xsdt.madt().unwrap();

    // Get available memory.
    let boot_services = system_table.boot_services().unwrap();
    let (available_memory, map_key) =
        uefi::mem::get_available_memory(&boot_services).unwrap();

    // Exit UEFI boot services.
    boot_services
        .exit_boot_services(image_handle, map_key)
        .unwrap();

    // Fill `BootInfo` structure and call kernel's entrypoint.
    let boot_info = BootInfo {
        available_memory,
        acpi_madt: madt,
    };
    os_main(boot_info)
}

/// Kernel entry point.
fn os_main(boot_info: BootInfo) -> ! {
    println!("lapic: {:#x?}", boot_info.acpi_madt.lapic());
    println!("memory map: {:#x?}", boot_info.available_memory.ranges());
    println!("memory size: {}", boot_info.available_memory.size());

    panic!("end");
}
