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
    system_table_ptr: *const uefi::EfiSystemTable,
) -> ! {
    // Initialize serial.
    serial::init_serial();

    let system_table =
        unsafe { uefi::SystemTable::new(system_table_ptr).unwrap() };
    let boot_services = system_table.boot_services().unwrap();

    println!("image_handle: {:#x?}", image_handle);

    let (memory_map, map_key) = boot_services.get_available_memory().unwrap();
    println!("memory_map: {:#x?} {:#x}", memory_map.ranges(), map_key);
    println!("available memory: {}", memory_map.size());
    println!(
        "exit_boot_services: {:?}",
        boot_services.exit_boot_services(image_handle, map_key)
    );

    panic!("end");
}
