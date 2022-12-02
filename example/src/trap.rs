
use core::arch::{asm, global_asm};
use riscv::register::time;
use crate::sbi::set_timer;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie, stval, stvec, sstatus, sscratch, sepc
};
use riscv::register::{
	sstatus::Sstatus,
	scause::Scause,
};

const TICKS_PER_SEC: usize = 100;
pub const CLOCK_FREQ: usize = 12500000;
pub static mut TICKS: usize = 0;

global_asm!(include_str!("trap.asm"));


#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32], 
    pub sstatus: usize,
    pub sepc: usize,
    pub stval: usize,
    pub scause: usize,
}


pub fn init() {
    unsafe {
        extern "C" {
            fn __alltraps();
        }
        sscratch::write(0);
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
        sstatus::set_sie();
        crate::plic::plic_init();

        sie::set_sext();
    }
    // timer_init();
    println!("++++ setup interrupt! ++++");
}


#[no_mangle]
pub unsafe fn trap_handler(tf: &mut TrapContext) {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            super_timer();
        }
        Trap::Interrupt(Interrupt::SupervisorExternal) => {
            println!("++++ supervisor external! ++++");
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
}

fn super_timer() {
    clock_set_next_event();
    unsafe {
        TICKS += 1;
        if TICKS == 100 {
            TICKS = 0;
            println!("* 100 ticks *");
        }
    }
}


fn breakpoint(sepc: &mut usize){
	println!("A breakpoint set @0x{:x} ", sepc);
	*sepc +=2
}

pub fn get_time() -> usize {
    time::read()
}

pub fn timer_init() {
    unsafe {
        TICKS = 0;
        sie::set_stimer();
    }
    clock_set_next_event();
    println!("++++ setup timer!     ++++");
}

pub  fn clock_set_next_event() {
    set_timer((get_time() + CLOCK_FREQ / TICKS_PER_SEC) as u64);
}