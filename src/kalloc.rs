// Port from xv6
// kernel stacks, page-table pages,
// and pipe buffers. Allocates whole 4096-byte pages.
// JZ  Moolman

use crate::memlayout::PHYSTOP;
use crate::riscv::PGSIZE;

use crate::spinlock::Spinlock;
use crate::string::memset;

extern "C" {
    static end: u64;
}

//#[ah] Should I say something like it a  C Structure
pub struct Run {
    next: u64,
}

pub struct Kmem {
    lock: Spinlock,
    freelist: u64,

}

impl Kmem {
    pub(crate) fn init() -> Self {
        let mut result = Self {
            lock: Spinlock::init(),
            freelist: 0,
        };
        let _end =  unsafe {  end };
        result.freerange(_end, PHYSTOP );
        result
    }

    fn freerange(&mut self, pa_start: u64, pa_end: u64) {
        let p = page_roundup(pa_start);
        for p in (p..=pa_end).step_by((PGSIZE - 1) as usize) {
            self.kfree(p);
        }
    }


    // Free the page of physical memory pointed at by pa,
    // which normally should have been returned by a
    // call to kalloc().  (The exception is when
    // initializing the allocator; see kinit above.)
    fn kfree(&mut self, pa: u64) {
        let _end = unsafe { end };
        if (pa % PGSIZE) != 0 || pa < _end || pa >= PHYSTOP {
            // panic("kfree");
        }

        // Fill with junk to catch dangling refs.
        memset(pa, 1, PGSIZE as isize);

        let r = pa as *mut Run;

        self.lock.aquire();
        unsafe  {
            (*r).next = self.freelist;
        }
        self.freelist = r as u64;

        self.lock.release();
    }

    fn kalloc(&mut self)-> u64 {
        self.lock.aquire();
        let r=   self.freelist as *mut Run;
        if r as u64 != 0 {
           unsafe  {
               self.freelist  = (*r).next;
           }
        }
        self.lock.release();
        if r as u64 != 0 {
            memset(r as u64, 5, PGSIZE as isize);
        }
        return r as u64;
    }
}

#[inline(always)]
fn page_roundup(pa: u64) -> u64 {
    (pa + PGSIZE-1) & !(PGSIZE-1)
}



