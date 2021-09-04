//! expOS is a tiny Operating System focused on experimentation.

#![no_std]
#![no_main]
#![feature(panic_info_message)]

use uefi::acpi;

mod panic;
mod serial;

/// UEFI entry point.
#[no_mangle]
extern "C" fn efi_main(
    image_handle: uefi::Handle,
    system_table_ptr: uefi::Ptr,
) -> ! {
    // Initialize serial.
    serial::init_serial();

    let system_table =
        unsafe { uefi::SystemTable::new(system_table_ptr).unwrap() };

    println!("{:#x?}", system_table);

    let boot_services = system_table.boot_services().unwrap();

    println!("{:#x?}", boot_services);

    let (memory_map, map_key) =
        uefi::mem::get_available_memory(&boot_services).unwrap();
    println!("memory_map: {:#x?} {:#x}", memory_map.ranges(), map_key);
    println!("available memory: {}", memory_map.size());

    let config_tables = system_table.configuration_tables().unwrap();

    println!("{:#x?}", config_tables);

    let rsdp20_ptr = config_tables.acpi_rsdp20_ptr().unwrap();
    let rsdp20 = unsafe { acpi::Rsdp20::new(rsdp20_ptr).unwrap() };

    println!("{:#x?}", rsdp20);

    let xsdt = rsdp20.xsdt().unwrap();

    println!("{:#x?}", xsdt);

    let madt = xsdt.madt().unwrap();

    println!("{:#x?}", madt);

    let lapic = madt.lapic();

    println!("{:#x?}", lapic);

    boot_services
        .exit_boot_services(image_handle, map_key)
        .unwrap();

    panic!("end");
}
