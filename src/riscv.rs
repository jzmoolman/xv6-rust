// Port from xv6
// JZ  Moolman

use core::arch::asm;


// which hart (core) is this?
pub fn r_mhartid() -> u64 {
    let x: u64;
    unsafe {
        asm!("csrr {}, mhartid", out(reg) x);
    }
    x
}

// Machine Status Register, mstatu

const MSTATUS_MPP_MASK: usize = 3 << 11; // previous mode.
const MSTATUS_MPP_M: usize = 3 << 11;
const MSTATUS_MPP_S: usize = 1 << 11;
const MSTATUS_MPP_U: usize = 0 << 11;
const MSTATUS_MIE: usize = 1 << 3;    // machine-mode interrupt enable.


// Supervisor Status Register, sstatus

const SSTATUS_SPP: usize = 1 << 8;  // Previous mode, 1=Supervisor, 0=User
const SSTATUS_SPIE: usize = 1 << 5; // Supervisor Previous Interrupt Enable
const SSTATUS_UPIE: usize = 1 << 4; // User Previous Interrupt Enable
const SSTATUS_SIE: usize = 1 << 1;  // Supervisor Interrupt Enable
const SSTATUS_UIE: usize = 1 << 0;  // User Interrupt Enable


fn r_mstatus() -> usize {
    let x: usize;
    unsafe {
        asm!("csrr {}, mstatus",  out(reg) x);
    }
    x
}

fn r_sstatus() -> usize {
    let x: usize;
    unsafe {
        asm!("csrr {}, sstatus",  out(reg) x);
    }
    x
}

fn w_xstatus(x:usize) {
    // w_xstatus where x is machine or supervisor mode
    unsafe  {
        asm!("csrw sstatus, {}",  in(reg) x);
    }
}

// machine exception program counter, holds the
// instruction address to which a return from
// exception will go.
pub fn w_mepc(x : u64) {
    unsafe {
        asm!("csrw mepc, {}", in(reg) x);
    }
}



pub fn intr_mget() -> bool {
    let x = r_mstatus(); //test in m mode
    if x & MSTATUS_MIE != 0 {
        true
    } else {
        false
    }
}

fn intr_sget() -> bool {
     let x = r_sstatus();
    if x & SSTATUS_SIE != 0 {
        true
    } else {
        false
    }
}

// disable device interrupts
pub fn intr_moff() {
    w_xstatus(r_mstatus() & !MSTATUS_MIE);
}

fn intr_soff() {
    w_xstatus(r_sstatus() & !SSTATUS_SIE);
}

// enable device interrupts
pub fn intr_mon() {
    w_xstatus(r_mstatus() | MSTATUS_MIE)
}

pub fn intr_son() {
    w_xstatus(r_sstatus() | SSTATUS_SIE)
}

pub const PGSIZE: u64 = 4096; // bytes per page

// read and write tp, the thread pointer, which xv6 uses to hold
// this core's hartid (core number), the index into cpus[].
pub(crate) fn r_tp() -> u64 {
    let x: u64;
    unsafe {
        asm!("mv {}, tp", out(reg) x);
    }
    x
}

pub fn w_tp(x: u64) {
    unsafe {
        asm!("mv tp, {}", in(reg) x);
    }
}