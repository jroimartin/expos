#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_panic_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
fn efi_main() -> ! {
    unsafe {
        core::ptr::read(0x4142434445464748 as *const u8);
    }

    loop {}
}
