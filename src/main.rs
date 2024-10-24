#![no_std]
#![no_main]

use core::arch::asm;
use crate::console::Cons;
use crate::kalloc::Kmem;
use crate::proc::cpuid;
use crate::spinlock::Spinlock;
use crate::uart::{Uart};

mod assembly;
mod uart;
mod spinlock;
mod riscv;
mod console;
mod kalloc;
mod memlayout;
mod string;
mod proc;
mod start;

/// This function is called on panic.A
#[panic_handler]
#[no_mangle]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    abort();
}

// extern "C" {
//     fn uart8250_init(base: u64,  freq_in: u64, buadrate: u64 );
//     fn uart8250_putc(base: u64,  char: u8);
// }

#[no_mangle]
pub extern "C" fn main () -> ! {
    // unsafe  {
    //     uart8250_init(0x10000000, 10000000, 115200);
    //     uart8250_putc(0x10000000, 0x41 );
    // }
    if cpuid() == 0 {
        let mut cons = Cons::new();
        cons.uart.putc(0x42);
        let mut kmem = Kmem::init();
    }
    loop {}
}

#[no_mangle]
extern "C"
fn abort() -> ! {
    loop {
        unsafe {
            // The asm! syntax has changed in Rust.
            // For the old, you can use llvm_asm!, but the
            // new syntax kicks ass--when we actually get to use it.
            asm!("wfi");
        }
    }
}