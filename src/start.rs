use core::arch::asm;
use crate::main;
use crate::riscv::{r_mhartid, w_mepc, w_tp};

#[no_mangle]
pub extern "C" fn _start () -> ! {

    // // set M Previous Privilege mode to Supervisor, for mret.
    // unsigned long x = r_mstatus();
    // x &= ~MSTATUS_MPP_MASK;
    // x |= MSTATUS_MPP_S;
    // w_mstatus(x);


    // set M Exception Program Counter to main, for mret.
    // requires gcc -mcmodel=medany
    w_mepc(main as u64);

    // keep each CPU's hartid in its tp register, for cpuid().
    let id = r_mhartid();
    w_tp(id);

    // switch to supervisor mode and jump to main().
    unsafe  {
        asm!("j main");
    }
    loop {}
}