#![feature(lang_items, asm, core_intrinsics)]
#![no_std]
use core::panic::PanicInfo;

pub mod gpio;
use gpio::PortFunction;

pub mod uart;

pub mod common;

const PORT_TXD: u32 = 14;
const PORT_RXD: u32 = 15;
const PORT_LED: u32 = 17;

#[no_mangle]
pub extern "C" fn main() {
    let mut led_flag = false;

    // set GPIO17 as Out(LED)
    gpio::set_port_function(PORT_LED, &PortFunction::Output);
    gpio::set_port(PORT_LED);
    // set GPIO14 as Alt0(TXD0)
    gpio::set_port_function(PORT_TXD, &PortFunction::Alt0);
    // set GPIO15 as Alt0(RXD0)
    gpio::set_port_function(PORT_RXD, &PortFunction::Alt0);

    // initialize UART and set baud rate to 115200bps
    uart::init();
    uart::set_baudrate(115200);

    uart::write_bytes("Press L to toggle LED\r\n".as_bytes());

    // main loop
    loop {
        loop {
            let input = uart::read_data();

            if input == 0x4C {
                led_flag = !led_flag;
                break;
            }
        }

        if led_flag {
            gpio::set_port(PORT_LED);
            uart::write_bytes("LED ON\r\n".as_bytes());
        } else {
            gpio::reset_port(PORT_LED);
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
