#![allow(unused)]

use core::arch::asm;

const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8;

#[inline(always)]
fn sbi_call(eid: usize, fid: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    unsafe {
        core::arch::asm!("ecall",
            in("a0") arg0,
            in("a1") arg1,
            in("a2") arg2,
            in("a6") fid,
            in("a7") eid,
            lateout("a0") ret,
        );
    }
    ret
}

pub fn console_putchar(ch: usize) -> usize {
    sbi_call(SBI_CONSOLE_PUTCHAR, 0, ch, 0, 0)
}

pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0, 0)
}

/// use sbi call to shutdown the kernel
pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0, 0);
    unreachable!();
}

pub fn set_timer(stime_value: u64) -> usize {
    sbi_call(SBI_SET_TIMER, 0, stime_value as usize, 0, 0)
}
