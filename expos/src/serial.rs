//! Primitives to output and read data via serial port.

use core::fmt::{self, Write};

use serial::SerialPort;
use ticket_mutex::TicketMutex;

/// Static variable that provides access to the COM1 serial port.
static COM1: TicketMutex<Option<SerialPort>> = TicketMutex::new(None);

/// Typically, COM1's IO port address.
/// FIXME: Do not use a fixed address, get it from UEFI.
const COM1_ADDRESS: u16 = 0x3f8;

/// Initialize COM1 serial. It is used by `print!`.
pub fn init_serial() {
    let mut com = COM1.lock();
    unsafe {
        *com = SerialPort::new(COM1_ADDRESS).ok();
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

/// Prints to the serial port.
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

/// Prints to the serial port, with a newline.
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    }
}
