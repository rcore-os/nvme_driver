#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

use core::arch::global_asm;

#[macro_use]
mod console;

mod lang_items;
mod sbi;
mod pci_scan;
mod nvme;
mod trap;
mod plic;


use crate::pci_scan::pci_scan;

global_asm!(include_str!("entry.asm"));
pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

use buddy_system_allocator::LockedHeap;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:#x?}", layout);
}


static mut HEAP_SPACE: [u8; 0x10000] = [0; 0x10000];


pub fn init_heap() {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP_SPACE.as_ptr() as usize, 0x10000);
    }
}


#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    init_heap();
    println!("Hello, world!");


    // trap::init();
    nvme::nvme_test();
    core::hint::spin_loop(); 
    loop{
    }
    panic!("+++++++++ NVME test has been completed +++++++++");
}
