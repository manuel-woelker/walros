// disable std, not available on raw metal
#![cfg_attr(not(test), no_std)]
// disable default main entry point
#![cfg_attr(not(test), no_main)]

#[allow(clippy::empty_loop)]
#[cfg(not(test))]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}

/// Install panic Handler, simply loop forever
#[allow(clippy::empty_loop)]
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
