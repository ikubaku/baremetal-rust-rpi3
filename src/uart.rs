//#![no_std]
use core::intrinsics::{volatile_store, volatile_load};

const PERIF_BASE_ADDR: u32 = 0x3F000000;
const UART_BASE_ADDR: u32 = PERIF_BASE_ADDR + 0x00201000;
const UART_FREQ: u32 = 48_000_000;

fn enable() {
    let uart_cr = (UART_BASE_ADDR + 0x30) as *mut u32;

    unsafe {
        volatile_store(uart_cr, *(uart_cr) | 0x00000001);
    }
}

fn disable() {
    let uart_cr = (UART_BASE_ADDR + 0x30) as *mut u32;

    unsafe {
        volatile_store(uart_cr, *(uart_cr) & !0x00000001);
    }
}

fn wait_for_transmission() {
    loop {
        let uart_fr = unsafe { volatile_load((UART_BASE_ADDR + 0x18) as *mut u32) };
        if uart_fr & 0x00000008 == 0 {
            break;
        }
    }
}

fn wait_for_data() {
    loop {
        let uart_fr = unsafe { volatile_load((UART_BASE_ADDR + 0x18) as *mut u32) };
        if uart_fr & 0x00000040 == 0 {
            break;
        }
    }
}

fn is_rx_empty() -> bool {
    let uart_fr = unsafe { volatile_load((UART_BASE_ADDR + 0x18) as *mut u32) };
    if uart_fr & 0x00000010 == 0 {
        return false;
    } else {
        return true;
    }
}

fn wait_for_peer() {
    loop {
        let uart_fr = unsafe { volatile_load((UART_BASE_ADDR + 0x18) as *mut u32) };
        if uart_fr & 0x00000020 == 0 {
            break;
        }
    }
}

fn is_tx_full() -> bool {
    let uart_fr = unsafe { volatile_load((UART_BASE_ADDR + 0x18) as *mut u32) };
    if uart_fr & 0x00000020 == 0 {
        return false;
    } else {
        return true;
    }
}

fn disable_fifo() {
    let uart_lcrh = (UART_BASE_ADDR + 0x2C) as *mut u32;

    unsafe {
        volatile_store(uart_lcrh, *(uart_lcrh) & !0x00000010);
    }
}

fn enable_fifo() {
    let uart_lcrh = (UART_BASE_ADDR + 0x2C) as *mut u32;

    unsafe {
        volatile_store(uart_lcrh, *(uart_lcrh) | 0x00000010);
    }
}

fn get_divisor(rate: u32) -> (u32, u32) {
    let bdiv_i = UART_FREQ / (16 * rate);
    let temp = (1000 * UART_FREQ as u64 / (16 * rate as u64) - 1000 * bdiv_i as u64) * 64 + 500;
    let bdiv_f= (temp / 1000) as u32;

    return (bdiv_i, bdiv_f);
}

pub fn init() {
    disable();
    wait_for_transmission();
    disable_fifo();

    // word size=8bit, with start/stop bit, no parity
    let uart_lcrh = (UART_BASE_ADDR + 0x2C) as *mut u32;

    unsafe {
        volatile_store(uart_lcrh, *(uart_lcrh) | 0x00000070);
    }

    // enable TX and RX, disable flow control
    let uart_cr = (UART_BASE_ADDR + 0x30) as *mut u32;

    unsafe {
        volatile_store(uart_cr, 0x00000300);
    }

    enable_fifo();
    enable();
}

pub fn set_baudrate(rate: u32) {
    // stop transmission before changing UART_CR register
    disable();
    wait_for_transmission();
    // flush fifo
    disable_fifo();

    // set baudrate divisor
    let (bdiv_i, bdiv_f) = get_divisor(rate);
    let uart_ibrd = (UART_BASE_ADDR + 0x24) as *mut u32;
    let uart_fbrd = (UART_BASE_ADDR + 0x28) as *mut u32;
    unsafe {
        volatile_store(uart_ibrd, bdiv_i);
        volatile_store(uart_fbrd, bdiv_f);
    }

    // enable fifo
    enable_fifo();
    // enable UART
    enable();
}

pub fn write_data(data: u8) {
    let uart_dr_data = (UART_BASE_ADDR + 0x00) as *mut u8;

    while is_tx_full() {};

    unsafe {
        volatile_store(uart_dr_data, data);
    }
}

pub fn write_bytes(data: &[u8]) {
    for b in data {
        write_data(*b);
    }
}

pub fn read_data() -> u8 {
    let uart_dr_data = (UART_BASE_ADDR + 0x00) as *mut u8;

    while is_rx_empty() {};

    return unsafe { volatile_load(uart_dr_data) }
}
