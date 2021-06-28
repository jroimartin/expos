//! expOS is a tiny Operating System focused on experimentation.

#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod panic;
mod serial;

/// UEFI entry point.
#[no_mangle]
extern "C" fn efi_main(
    image_handler: uefi::Handle,
    system_table_ptr: *const uefi::EfiSystemTable,
) -> ! {
    // Initialize serial.
    serial::init_serial();

    let system_table =
        unsafe { uefi::SystemTable::new(system_table_ptr).unwrap() };
    let boot_services = system_table.boot_services().unwrap();

    println!("image_handler: {:#x?}", image_handler);
    println!("count: {:#x?}", boot_services.get_next_monotonic_count());
    println!("count: {:#x?}", boot_services.get_next_monotonic_count());

    panic!("end");
}
