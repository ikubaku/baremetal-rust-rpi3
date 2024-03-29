//#![no_std]
//#![feature(asm)]

pub const PERIF_BASE_ADDR: u32 = 0x3F000000;

// BCM2836 local peripheral
pub const BCM2836_LP_BASE_ADDR: u32 = 0x40000000;

pub fn test_bit(value: u32, n: u32) -> bool {
    if n > 31 {
        return false;
    } else {
        return (value & (1 << n)) != 0;
    }
}

pub fn set_bit(value: u32, n: u32) -> u32 {
    if n > 31 {
        return value;
    } else {
        return value | (1 << n);
    }
}

pub fn clear_bit(value: u32, n: u32) -> u32 {
    if n > 31 {
        return value;
    } else {
        return value & !(1 << n);
    }
}

pub fn halt() {
    unsafe {
        asm!("wfe");
    }
}
