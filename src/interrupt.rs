//#![no_std]
//#![feature(asm)]
use core::intrinsics::{volatile_store, volatile_load};

use crate::common;

const INTERRUPT_BASE_ADDR: u32 = common::PERIF_BASE_ADDR + 0x00008000;

const IRQ_1_PENDING_ADDR: u32 = INTERRUPT_BASE_ADDR + 0x204;
const IRQ_1_ENABLE_ADDR: u32 = INTERRUPT_BASE_ADDR + 0x210;
const IRQ_1_DISABLE_ADDR: u32 = INTERRUPT_BASE_ADDR + 0x21C;
const IRQ_2_PENDING_ADDR: u32 = INTERRUPT_BASE_ADDR + 0x208;
const IRQ_2_ENABLE_ADDR: u32 = INTERRUPT_BASE_ADDR + 0x214;
const IRQ_2_DISABLE_ADDR: u32 = INTERRUPT_BASE_ADDR + 0x220;
const IRQ_BASIC_DISABLE_ADDR: u32 = INTERRUPT_BASE_ADDR + 0x224;
const FIQ_CTRL_ADDR: u32 = INTERRUPT_BASE_ADDR + 0x20C;

const GPU_INTERRUPT_ROUTING_ADDR: u32 = common::BCM2836_LP_BASE_ADDR + 0x0C;


pub fn enable_irq(irq: u32) {
    if irq < 32 {
        let irq_enable = IRQ_1_ENABLE_ADDR as *mut u32;

        unsafe {
            volatile_store(irq_enable, 1 << irq);
        }
    } else if irq < 64 {
        let irq_enable = IRQ_2_ENABLE_ADDR as *mut u32;

        unsafe {
            volatile_store(irq_enable, 1 << (irq - 32));
        }
    }
}

pub fn disable_irq(irq: u32) {
    if irq < 32 {
        let irq_disable = IRQ_1_DISABLE_ADDR as *mut u32;

        unsafe {
            volatile_store(irq_disable, 1 << irq);
        }
    } else if irq < 64 {
        let irq_disable = IRQ_2_DISABLE_ADDR as *mut u32;

        unsafe {
            volatile_store(irq_disable, 1 << (irq - 32));
        }
    }
}

pub fn init() {
    let fiq_ctrl = FIQ_CTRL_ADDR as *mut u32;
    let irq_basic_disable = IRQ_BASIC_DISABLE_ADDR as *mut u32;
    let irq_1_disable = IRQ_1_DISABLE_ADDR as *mut u32;
    let irq_2_disable = IRQ_2_DISABLE_ADDR as *mut u32;
    let gpu_int_routing = GPU_INTERRUPT_ROUTING_ADDR as *mut u32;

    unsafe {
        // disable FIQ
        volatile_store(fiq_ctrl, 0x00000000);
        // disable all IRQs
        volatile_store(irq_basic_disable, 0x000000FF);
        volatile_store(irq_1_disable, 0xFFFFFFFF);
        volatile_store(irq_2_disable, 0xFFFFFFFF);

        // route GPU IRQ & FIQ to Core 0
        volatile_store(gpu_int_routing, 0x00000000);
    }
}
