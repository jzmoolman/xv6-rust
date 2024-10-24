use crate::riscv::r_tp;

// Must be called with interrupts disabled,
// to prevent race with process being moved
// to a different CPU.

pub fn cpuid() -> u64 {
    r_tp()
}