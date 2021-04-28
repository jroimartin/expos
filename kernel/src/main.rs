//! expOS is a tiny Operating System focused on experimentation.

#![no_std]
#![no_main]
#![feature(asm, panic_info_message)]

mod cpu;
mod panic;
mod serial;
mod uefi;

/// UEFI entry point.
#[no_mangle]
extern "C" fn efi_main(
    image_handler: uefi::EfiHandle,
    system_table: uefi::EfiSystemTable,
) -> ! {
    // Initialize serial.
    serial::init_serial();

    println!("image_handler: {:#x?}", image_handler);
    println!("system_table: {:#x?}", system_table);

    panic!("end");
}