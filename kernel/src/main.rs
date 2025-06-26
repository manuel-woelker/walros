// disable std, not available on raw metal
#![cfg_attr(not(test), no_std)]
// disable default main entry point
#![cfg_attr(not(test), no_main)]

mod serial;
mod vga_buffer;

#[allow(dead_code)]
static HELLO: &[u8] = b"Hello Wild World!";

#[allow(clippy::empty_loop)]
#[cfg(not(test))]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // write X to COM1 using assembly
    sprintln!("Hello Serial Port!");
    sprintln!("Hello again!");
    panic!("Hello World{}", "!");
}

/// Install panic Handler, simply loop forever
#[allow(clippy::empty_loop)]
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("KERNEL PANIC: {}", info);
    loop {}
}
