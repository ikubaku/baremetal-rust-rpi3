//#![no_std]
use core::intrinsics::{volatile_store, volatile_load};

use crate::common;

const UART_BASE_ADDR: u32 = common::PERIF_BASE_ADDR + 0x00201000;

const UART_DR_ADDR: u32 = UART_BASE_ADDR + 0x00;
const UART_FR_ADDR: u32 = UART_BASE_ADDR + 0x18;
const UART_IBRD_ADDR: u32 = UART_BASE_ADDR + 0x24;
const UART_FBRD_ADDR: u32 = UART_BASE_ADDR + 0x28;
const UART_LCRH_ADDR: u32 = UART_BASE_ADDR + 0x2C;
const UART_CR_ADDR: u32 = UART_BASE_ADDR + 0x30;
const UART_IMSC_ADDR: u32 = UART_BASE_ADDR + 0x38;
const UART_MIS_ADDR: u32 = UART_BASE_ADDR + 0x40;
const UART_ICR_ADDR: u32 = UART_BASE_ADDR + 0x44;

const UART_FREQ: u32 = 48_000_000;

fn enable() {
    let uart_cr = UART_CR_ADDR as *mut u32;

    unsafe {
        volatile_store(uart_cr, common::set_bit(*uart_cr, 0));
    }
}

fn disable() {
    let uart_cr = UART_CR_ADDR as *mut u32;

    unsafe {
        volatile_store(uart_cr, common::clear_bit(*uart_cr, 0));
    }
}

fn wait_for_transmission() {
    loop {
        let uart_fr_val = unsafe { volatile_load(UART_FR_ADDR as *mut u32) };
        // break loop if BUSY is not set
        if !common::test_bit(uart_fr_val, 3) {
            break;
        }
    }
}

fn is_rx_empty() -> bool {
    let uart_fr_val = unsafe { volatile_load(UART_FR_ADDR as *mut u32) };

    // return state of RXFE
    return common::test_bit(uart_fr_val, 4);
}

fn is_tx_full() -> bool {
    let uart_fr_val = unsafe { volatile_load(UART_FR_ADDR as *mut u32) };

    // return state of TXFF
    return common::test_bit(uart_fr_val, 5);
}

fn disable_fifo() {
    let uart_lcrh = UART_LCRH_ADDR as *mut u32;

    unsafe {
        volatile_store(uart_lcrh, common::clear_bit(*uart_lcrh, 4));
    }
}

fn enable_fifo() {
    let uart_lcrh = UART_LCRH_ADDR as *mut u32;

    unsafe {
        volatile_store(uart_lcrh, common::set_bit(*uart_lcrh, 4));
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

    let uart_lcrh = UART_LCRH_ADDR as *mut u32;

    // word size=8bit, with start/stop bit, no parity
    unsafe {
        volatile_store(uart_lcrh, 0x00000070);
    }

    // enable TX and RX, disable flow control
    let uart_cr = UART_CR_ADDR as *mut u32;

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
    let uart_ibrd = UART_IBRD_ADDR as *mut u32;
    let uart_fbrd = UART_FBRD_ADDR as *mut u32;
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
    let uart_dr_data = UART_DR_ADDR as *mut u8;

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
    let uart_dr_data = UART_DR_ADDR as *mut u8;

    while is_rx_empty() {};

    return unsafe { volatile_load(uart_dr_data) }
}

pub fn read_data_nonblock() -> Option<u8> {
    let uart_dr_data = UART_DR_ADDR as *mut u8;

    if is_rx_empty() {
        return None;
    } else {
        return Some(unsafe { volatile_load(uart_dr_data) });
    }
}

pub fn init_interrupt() {
    let uart_imsc = UART_IMSC_ADDR as *mut u32;

    // mask all UART interrupt
    unsafe {
        volatile_store(uart_imsc, 0x00000000);
    }
}

pub fn enable_rx_interrupt() {
    let uart_imsc = UART_IMSC_ADDR as *mut u32;

    unsafe {
        volatile_store(uart_imsc, common::set_bit(*uart_imsc, 4));
    }
}

pub fn is_receive_masked_interrupt() -> bool {
    let uart_mis_val = unsafe { volatile_load(UART_MIS_ADDR as *mut u32) };

    if common::test_bit(uart_mis_val, 4) {
        return true;
    } else {
        return false;
    }
}

pub fn clear_receive_masked_interrupt() {
    let uart_icr = UART_ICR_ADDR as *mut u32;

    unsafe {
        volatile_store(uart_icr, common::set_bit(*uart_icr, 4));
    }
}
