//#![no_std]
use core::intrinsics::volatile_store;

const PERIF_BASE_ADDR: u32 = 0x3F000000;
const GPIO_BASE_ADDR: u32 = PERIF_BASE_ADDR + 0x00200000;

pub enum PortFunction {
    Input,
    Output,
    Alt0,
    Alt1,
    Alt2,
    Alt3,
    Alt4,
    Alt5,
}

fn get_gpfsel_reg_mut(port: u32) -> *mut u32 {
    return (GPIO_BASE_ADDR + 0x00 + 4 * (port / 10)) as *mut u32;
}

fn get_gpset_reg_mut(port: u32) -> *mut u32 {
    return (GPIO_BASE_ADDR + 0x1C + 4 * (port / 32)) as *mut u32;
}

fn get_gpclr_reg_mut(port: u32) -> *mut u32 {
    return (GPIO_BASE_ADDR + 0x28 + 4 * (port / 32)) as *mut u32;
}

fn get_fsel_lsb(port: u32) -> u32 {
    return 3 * (port % 10);
}

fn port_function_to_flag(func: &PortFunction) -> u32 {
    return match *func {
        PortFunction::Input => 0b000,
        PortFunction::Output => 0b001,
        PortFunction::Alt0 => 0b100,
        PortFunction::Alt1 => 0b101,
        PortFunction::Alt2 => 0b110,
        PortFunction::Alt3 => 0b111,
        PortFunction::Alt4 => 0b011,
        PortFunction::Alt5 => 0b010,
    }
}

fn get_fsel_flags(port: u32, func: &PortFunction) -> u32 {
    return port_function_to_flag(func) << get_fsel_lsb(port);
}

fn get_fsel_mask(port: u32) -> u32 {
    return (0b111 as u32) << get_fsel_lsb(port);
}

fn get_port_bit(port: u32) -> u32 {
    return 1 << (port % 32);
}

pub fn set_port_function(port: u32, func: &PortFunction) {
    let gpfsel = get_gpfsel_reg_mut(port);

    unsafe {
        volatile_store(gpfsel, *(gpfsel) & !get_fsel_mask(port) | get_fsel_flags(port, func));
    }
}

pub fn set_port(port: u32) {
    let gpset = get_gpset_reg_mut(port);

    unsafe {
        volatile_store(gpset, *(gpset) | get_port_bit(port));
    }
}
                     
pub fn reset_port(port: u32) {
    let gpclr = get_gpclr_reg_mut(port);

    unsafe {
        volatile_store(gpclr, *(gpclr) | get_port_bit(port));
    }
}
