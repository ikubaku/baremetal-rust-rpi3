#![feature(lang_items, asm, core_intrinsics)]
#![no_std]
use core::intrinsics::volatile_store;
use core::panic::PanicInfo;

#[no_mangle]
pub extern fn main() {
    // GPIOSEL1
    let gpfsel1 = 0x3F200004 as *mut u32;
    // GPSET0
    let gpset0 = 0x3F20001C as *mut u32;
    // GPCLR0
    let gpclr0 = 0x3F200028 as *mut u32;

    // set GPIO17 to OUTPUT
    unsafe {
        volatile_store(gpfsel1, *(gpfsel1) & !(((0x07 as u32) << (7*3 as u32))));
        volatile_store(gpfsel1, *(gpfsel1) | ((0x01 as u32) << (7*3 as u32)));
    }

    loop {
        // GPSET0: set GPIO17 to HIGH
        unsafe {
            volatile_store(gpset0, 1 << 17 as u32);
        }
        
        // busy wait
        for _ in 1..1000000 {
            unsafe { asm!(""); }
        }

        // GPCLR0: set GPIO17 to LOW
        unsafe {
            volatile_store(gpclr0, 1 << 17 as u32);
        }

        // busy wait
        for _ in 1..1000000 {
            unsafe { asm!(""); }
        }
    }
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // do nothing for now
    // TODO: rename _info to info after implementing panic process
    loop {}
}
