#![feature(lang_items, asm, core_intrinsics)]
#![no_std]
use core::panic::PanicInfo;

pub mod gpio;
use gpio::PortFunction;

pub mod uart;

#[no_mangle]
pub extern "C" fn main() {
    let mut led_flag = false;

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
        for c in "Press L to flip LED state\r\n".as_bytes() {
            uart::write_data(*c);
        }
        loop {
            let input = uart::read_data();

            if input == 0x4C {
                led_flag = !led_flag;
                break;
            }
        }

        if led_flag {
            gpio::set_port(17);
            uart::write_bytes("LED ON\r\n".as_bytes());
        } else {
            gpio::reset_port(17);
            uart::write_bytes("LED OFF\r\n".as_bytes());
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
