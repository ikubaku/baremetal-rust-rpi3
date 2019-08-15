#![feature(lang_items, asm, core_intrinsics)]
#![no_std]
use core::panic::PanicInfo;

pub mod gpio;
use gpio::PortFunction;

pub mod uart;

pub mod common;

pub mod interrupt;

const PORT_TXD: u32 = 14;
const PORT_RXD: u32 = 15;
const PORT_LED: u32 = 17;

#[no_mangle]
pub extern "C" fn main() {
    // initialize interrupts
    interrupt::init();
    interrupt::bcm2835_enable_irq(57);

    uart::init_interrupt();    // UART
    uart::enable_rx_interrupt();

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

    // unmask IRQ
    interrupt::enable_irq();

    uart::write_bytes("Init complete\r\n".as_bytes());

    // main loop
    loop {
        gpio::reset_port(PORT_LED);

        for _i in 0..1000000 {
            unsafe {
                asm!("");
            }
        }

        gpio::set_port(PORT_LED);

        for _i in 0..1000000 {
            unsafe {
                asm!("");
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn current_el_spx_sync_el2_handler() {}

#[no_mangle]
pub extern "C" fn current_el_spx_irq_el2_handler() {
    //uart::write_bytes("IRQ!!\r\n".as_bytes());
    if uart::is_receive_masked_interrupt() {
        while let Some(b) = uart::read_data_nonblock() {
            uart::write_data(b);
        }
        uart::clear_receive_masked_interrupt();
    }
}

#[no_mangle]
pub extern "C" fn current_el_spx_fiq_el2_handler() {}

#[no_mangle]
pub extern "C" fn current_el_spx_serror_el2_handler() {}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // do nothing for now
    // TODO: rename _info to info after implementing panic process
    loop {}
}
