//! CPU primitives.

#![no_std]
#![feature(asm)]

/// Reads an `u8` from the specified IO port address.
///
/// # Safety
///
/// This function executes an `in` instruction passing the provided
/// `port_addr`. Thus, it is considered unsafe.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline]
pub unsafe fn in8(port_addr: u16) -> u8 {
    let retval: u8;

    asm!(
        "in al, dx",
        out("al") retval,
        in("dx") port_addr,
    );

    retval
}

/// Writes an `u8` to the specified IO port address.
///
/// # Safety
///
/// This function executes an `out` instruction passing the provided
/// `port_addr`. Thus, it is considered unsafe.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline]
pub unsafe fn out8(port_addr: u16, val: u8) {
    asm!(
        "out dx, al",
        in("dx") port_addr,
        in("al") val,
    );
}

/// Stops instruction execution and places the processor in a HALT state.
///
/// # Safety
///
/// This function executes a `hlt` instruction. Thus, it is considered unsafe.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline]
pub unsafe fn hlt() {
    asm!("hlt");
}
