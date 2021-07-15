//! expOS is a tiny Operating System focused on experimentation.

#![no_std]
#![no_main]
#![feature(panic_info_message)]

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
    let boot_services = system_table.boot_services().unwrap();

    let (memory_map, map_key) =
        uefi::mem::get_available_memory(&boot_services).unwrap();
    println!("memory_map: {:#x?} {:#x}", memory_map.ranges(), map_key);
    println!("available memory: {}", memory_map.size());

    let config_tables = system_table.configuration_tables().unwrap();

    let rsdp20_ptr = config_tables.acpi_rsdp20_ptr().unwrap();
    let rsdp20 = unsafe { uefi::acpi::Rsdp20::new(rsdp20_ptr).unwrap() };

    let xsdt_ptr = rsdp20.xsdt_ptr().unwrap();
    let xsdt = unsafe { uefi::acpi::Xsdt::new(xsdt_ptr).unwrap() };

    let madt_ptr = xsdt.madt_ptr().unwrap();

    println!("{:#x?}", xsdt);
    println!("{:#x?}", madt_ptr);

    boot_services
        .exit_boot_services(image_handle, map_key)
        .unwrap();

    panic!("end");
}
