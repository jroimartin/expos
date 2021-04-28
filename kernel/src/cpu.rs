//! CPU primitives.

/// Reads an `u8` from the specified IO port address.
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
#[inline]
pub unsafe fn out8(port_addr: u16, val: u8) {
    asm!(
        "out dx, al",
        in("dx") port_addr,
        in("al") val,
    );
}

/// Stops instruction execution and places the processor in a HALT state.
#[inline]
pub unsafe fn hlt() {
    asm!("hlt");
}
