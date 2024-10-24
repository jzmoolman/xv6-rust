// Port from xv6
// Mutual exclusion spin locks.
// JZ  Moolman

use core::arch::asm;
use crate::riscv::{intr_mget, intr_moff, intr_mon};

pub struct Spinlock {
    locked: u64,
    // for debugging
    // name: string,
    // cpu: *Cpu
}

impl Spinlock {
    pub fn  init() -> Self {
        Self {
            locked : 0,
            //name
            //cpu
        }
    }

    pub fn push_off()
    {
       let old =  intr_mget();
        intr_moff();
        // if(mycpu()->noff == 0)
        // mycpu()->intena = old;
        // mycpu()->noff += 1;
    }


    fn pop_off() {
        // struct cpu *c = mycpu();
        if intr_mget()  {
            // panic("pop_off - interruptible");
        }
        // if(c->noff < 1)
        // panic("pop_off");
        // c->noff -= 1;
        // if(c->noff == 0 && c->intena)
        intr_mon();

    }


    pub fn aquire(&mut self) {
        Self::push_off();    // disable interrupts to avoid deadlock.

        // if(holding(lk))
        // panic("acquire");k

        // On RISC-V, sync_lock_test_and_set turns into an atomic swap:
        //   a5 = 1
        //   s1 = &lk->locked
        //   amoswap.w.aq a5, a5, (s1)
        let x : *mut u64 = &mut self.locked;
        let mut y : u64 = 1;
        loop {
            unsafe {
                asm!("amoswap.w.aq {0}, {0}, ({1})", inout(reg) y, in(reg) x)
            }
            if y == 0  {
                break;
            }
        }
        // Tell the C compiler and the processor to not move loads or stores
        // past this point, to ensure that the critical section's memory
        // references happen strictly after the lock is acquired.
        // On RISC-V, this emits a fence instruction.
        unsafe {
            asm!("fence");
        }

        // Record info about lock acquisition for holding() and debugging.
        // lk->cpu = mycpu();

    }

    // Release the lock.
    pub fn release(&mut self) {
        // if(!holding(lk))
        // panic("release");

        // lk->cpu = 0;

        // Tell the C compiler and the CPU to not move loads or stores
        // past this point, to ensure that all the stores in the critical
        // section are visible to other CPUs before the lock is released,
        // and that loads in the critical section occur strictly before
        // the lock is released.
        // On RISC-V, this emits a fence instruction.
        //__sync_synchronize();
        unsafe {
            asm!("fence");
        }

        // Release the lock, equivalent to lk->locked = 0.
        // This code doesn't use a C assignment, since the C standard
        // implies that an assignment might be implemented with
        // multiple store instructions.
        // On RISC-V, sync_lock_release turns into an atomic swap:
        //   s1 = &lk->locked
        //   amoswap.w zero, zero, (s1)
        //__sync_lock_release(&lk->locked);

        let x : *mut u64 = &mut self.locked;
        unsafe {
            asm!("amoswap.w zero, zero, ({})",  in(reg) x)
        }

        Self::pop_off();
    }



}