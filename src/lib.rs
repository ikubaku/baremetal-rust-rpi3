#![feature(lang_items, asm, core_intrinsics)]
#![no_std]
use core::panic::PanicInfo;

pub mod gpio;
use gpio::PortFunction;

#[no_mangle]
pub extern "C" fn main() {
    // set GPIO17 to OUTPUT
    gpio::set_port_function(17, &PortFunction::Output);

    loop {
        // set GPIO17 to HIGH
        gpio::set_port(17);

        // busy wait
        for _ in 1..10000000 {
            unsafe {
                asm!("");
            }
        }

        // set GPIO17 to LOW
        gpio::reset_port(17);

        // busy wait
        for _ in 1..10000000 {
            unsafe {
                asm!("");
            }
        }
    }
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // do nothing for now
    // TODO: rename _info to info after implementing panic process
    loop {}
}
