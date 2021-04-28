//! Panic handling.

use core::panic::PanicInfo;

use crate::cpu::hlt;
use crate::println;

/// Panic handler.
#[panic_handler]
fn panic_handler(panic_info: &PanicInfo) -> ! {
    println!("====== PANIC ======");

    if let Some(message) = panic_info.message() {
        println!("{}", message);
    }

    if let Some(payload) = panic_info.payload().downcast_ref::<&str>() {
        println!("{}", payload);
    }

    if let Some(location) = panic_info.location() {
        println!("Panic ocurred in {}", location);
    }

    loop {
        unsafe { hlt() };
    }
}
