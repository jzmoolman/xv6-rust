use core::arch::global_asm;

global_asm!(include_str!("asm/entry.S"));
// global_asm!(include_str!("asm/uart8250.S"));
// global_asm!(include_str!("asm/trampoline.S"));
