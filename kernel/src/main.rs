// disable std, not available on raw metal
#![cfg_attr(not(test), no_std)]
// disable default main entry point
#![cfg_attr(not(test), no_main)]

static HELLO: &[u8] = b"Hello Wild World!";

#[allow(clippy::empty_loop)]
#[cfg(not(test))]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
/// Install panic Handler, simply loop forever
#[allow(clippy::empty_loop)]
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
