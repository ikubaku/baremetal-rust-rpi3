#![feature(lang_items, asm, core_intrinsics)]
#![no_std]
use core::panic::PanicInfo;

pub mod gpio;
use gpio::PortFunction;

pub mod uart;

#[no_mangle]
pub extern "C" fn main() {
    // set GPIO17 as Out(LED)
    gpio::set_port_function(17, &PortFunction::Output);
    gpio::set_port(17);
    // set GPIO14 as Alt0(TXD0)
    gpio::set_port_function(14, &PortFunction::Alt0);
    // set GPIO15 as Alt0(RXD0)
    gpio::set_port_function(15, &PortFunction::Alt0);

    uart::init();
    uart::set_baudrate(115200);

    loop {
        for c in "Hello, world!\r\n".as_bytes() {
            uart::write_data(*c);
        }
        for _i in 0..1000000 {
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
