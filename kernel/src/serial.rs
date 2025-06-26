use core::arch::asm;
use core::fmt;
use core::fmt::Write;
use lazy_static::lazy_static;
use spin::mutex::Mutex;

static COM1: u16 = 0x3F8;

fn is_transmit_empty(port: u16) -> bool {
    let status: u8;
    unsafe {
        asm!(
        "in al, dx",
        in("dx") (port + 5), // Line Status Register
        out("al") status,
        options(nomem, nostack, preserves_flags),
        );
    }
    status & 0x20 != 0 // Transmit Holding Register Empty
}

fn write_serial_port(port: u16, data: u8) {
    unsafe {
        asm!(
        "out dx, al",
        in("dx") port,
        in("al") data,
        options(nomem, nostack, preserves_flags),
        );
    }
}

pub fn serial_write(port: u16, string: &str) {
    while !is_transmit_empty(0x3fd) {}
    for byte in string.bytes() {
        let byte_to_write = match byte {
            // printable ASCII byte or newline
            0x20..=0x7e | b'\n' => byte,
            // not part of printable ASCII range
            _ => 0xfe,
        };
        write_serial_port(port, byte_to_write);
    }
}

pub struct SerialWriter {
    port: u16,
}

impl Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        serial_write(self.port, s);
        Ok(())
    }
}

lazy_static! {
    pub static ref COM1_WRITER: Mutex<SerialWriter> = Mutex::new(SerialWriter { port: COM1 });
}

#[macro_export]
macro_rules! sprint {
    ($($arg:tt)*) => ($crate::serial::_sprint(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! sprintln {
    () => ($crate::sprint!("\n"));
    ($($arg:tt)*) => ($crate::sprint!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _sprint(args: fmt::Arguments) {
    use core::fmt::Write;
    COM1_WRITER.lock().write_fmt(args).unwrap();
}
