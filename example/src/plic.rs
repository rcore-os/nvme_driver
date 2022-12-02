use core::ptr;
use core::ptr::{read_volatile, write_volatile};
use core::arch::asm;

use riscv::register::sie;

pub fn irq_handler() {
    // if let Some(irq) = plic_claim() {
    //     match irq {
    //         _ => panic!("unsupported IRQ {}", irq),
    //     }
        
    //     plic_complete(irq);
    // }
}


pub fn plic_init(){

    let irq = 33;
    let ie_words = irq / 32 + 1;
    plic_set_thresh(0, 0);
    plic_set_priority(irq, 7);
    plic_set_ie(0, ie_words, 1);

    println!("plic init done");
}
fn write(addr: usize, val: u32) {
    unsafe {
        ptr::write_volatile(addr as *mut u32, val);
    }
}

fn read(addr: usize) -> u32 {
    unsafe {
        ptr::read_volatile(addr as *const u32)
    }
}

pub fn hart_id() -> usize {
    0
}


pub const PLIC_BASE: usize = 0xc000000;
pub const PLIC_PRIORITY_BASE   :usize = 0x4;
pub const PLIC_PENDING_BASE    :usize = 0x1000;
pub const PLIC_ENABLE_BASE     :usize = 0x2000;
pub const PLIC_ENABLE_STRIDE   :usize = 0x80;
pub const PLIC_CONTEXT_BASE    :usize = 0x200000;
pub const PLIC_CONTEXT_STRIDE  :usize = 0x1000;

pub fn plic_set_priority(source: u32, value: u32) {
    let mut addr = PLIC_BASE + PLIC_PRIORITY_BASE as usize + source as usize * 4;
    write(addr as usize, value);
}

pub fn plic_set_thresh(cntxid: u32, value: u32) {
    let addr = PLIC_BASE + PLIC_CONTEXT_BASE +PLIC_CONTEXT_STRIDE * cntxid as usize;
    write(addr as usize, value);
}

pub fn plic_set_ie(cntxid: u32, word_index: u32, val: u32) {
    let mut addr = PLIC_BASE + PLIC_ENABLE_BASE as usize +PLIC_ENABLE_STRIDE * cntxid as usize + word_index as usize * 4;
    write(addr as usize, val);
}
