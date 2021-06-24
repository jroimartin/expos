//! Serial port support for the 8250 UART used in the IBM PC.
//!
//! Reference:
//! - [Wikipedia article](https://en.wikipedia.org/wiki/8250_UART)
//! - [Datasheet](https://web.archive.org/web/20160503070506/http://archive.pcjs.org/pubs/pc/datasheets/8250A-UART.pdf)

use core::fmt::{self, Write};

use crate::cpu::{in8, out8};
use crate::ticket_mutex::TicketMutex;

/// Static variable that provides access to the COM1 serial port.
static COM1: TicketMutex<Option<SerialPort>> = TicketMutex::new(None);

/// Typically, COM1's IO port address.
/// FIXME: Do not use a fixed address, get it from UEFI.
const COM1_ADDRESS: u16 = 0x3f8;

/// Error representing that the serial port is not operating normally.
#[derive(Debug)]
struct Error;

/// Represents a serial port.
struct SerialPort(u16);

/// Initialize COM1 serial. It is used by `print!`.
pub fn init_serial() {
    let mut com = COM1.lock();
    unsafe {
        *com = Some(SerialPort::new(COM1_ADDRESS).unwrap());
    }
}

impl SerialPort {
    /// Constructs a new `SerialPort`.
    ///
    /// # Errors
    ///
    /// This function performs a loopback test of the serial port. If it fails,
    /// an `Error` is returned.
    ///
    /// # Safety
    ///
    /// The port address is provided by the user, so creating a new
    /// `SerialPort` is considered unsafe. However, a `SerialPort` is only
    /// returned if the loopback test succeeded. Thus, we consider its methods
    /// to be safe.
    unsafe fn new(port_addr: u16) -> Result<SerialPort, Error> {
        // Disable DLAB.
        out8(port_addr + 3, 0x00);

        // Disable all interrupts.
        out8(port_addr + 1, 0x00);

        // Enable DLAB.
        out8(port_addr + 3, 0x80);

        // Set divisor latch to 3 (38400 bps for a 1.8432 MHz Crystal).
        // LSB.
        out8(port_addr, 0x03);
        // MSB.
        out8(port_addr + 1, 0x00);

        // Disable DLAB. Set 8N1 mode.
        out8(port_addr + 3, 0x03);

        // Enable loop mode for loopback test.
        out8(port_addr + 4, 0x10);

        // Check that we received the same byte we sent. If that is not the
        // case, then return an error because the serial is faulty.
        out8(port_addr, 0xae);
        if in8(port_addr) != 0xae {
            return Err(Error);
        }

        // If the serial is working properly, set it in normal operation mode.
        // Modem Control Register: Disable loop mode.
        out8(port_addr + 4, 0x00);

        Ok(SerialPort(port_addr))
    }

    /// Returns `true` if the Transmitter Holding Register (THR) is empty,
    /// indicating that the UART is ready to accept a new character for
    /// transmission.
    fn is_thr_empty(&self) -> bool {
        // Check the "Transmitter Holding Register Empty" indicator.
        unsafe { in8(self.0 + 5) & 0x20 != 0 }
    }

    /// Writes a single `u8` to the serial port.
    fn write_u8(&self, b: u8) {
        while !self.is_thr_empty() {}

        unsafe { out8(self.0, b) };
    }

    /// Writes the buffer `buf` to the serial port.
    fn write<B: AsRef<[u8]>>(&self, buf: B) {
        let buf = buf.as_ref();

        for b in buf.iter() {
            self.write_u8(*b);
        }
    }

    /// Returns `true` if a complete incoming character has been received and
    /// transferred into the Receiver Buffer Register.
    fn is_data_ready(&self) -> bool {
        // Check the "Data Ready" indicator.
        unsafe { in8(self.0 + 5) & 0x1 != 0 }
    }

    /// Reads a single `u8` from the serial port.
    pub fn read_u8(&self) -> u8 {
        while !self.is_data_ready() {}

        unsafe { in8(self.0) }
    }
}

/// The type `SerialWriter` implements the `Write` trait for serial.
pub struct SerialWriter;

impl Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let com = COM1.lock();
        if let Some(serial) = com.as_ref() {
            serial.write(s);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        // In the case of a `SeriaWriter`, `write_str` cannot fail, so
        // we can safely unwrap the returned result.
        core::fmt::Write::write_fmt(
            &mut $crate::serial::SerialWriter,
            format_args!($($arg)*)
        ).unwrap()
    }
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    }
}
