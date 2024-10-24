// Port from xv6
//
// Console input and output, to the uart.
// Reads are line at a time.
// Implements special input characters:
//   newline -- end of line
//   control-h -- backspace
//   control-u -- kill line
//   control-d -- end of file
//   control-p -- print process list
//
// JZ  Moolman
use crate::spinlock::Spinlock;
use crate::uart::Uart;

const INPUT_BUF_SIZE: usize = 128;

pub struct Cons {
    lock: Spinlock,
    pub(crate) uart: Uart,
    buf: [u8; INPUT_BUF_SIZE],
    r: usize,      // Read index
    w: usize,     // Write index
    e: usize,     // Edit index
}

impl Cons {
    pub(crate) fn new() -> Self {
        Self {
            lock: Spinlock::init(),
            uart: Uart::new(0x10000000),
            buf: [0; INPUT_BUF_SIZE],
            r: 0,
            w: 0,
            e: 0,
        }
    }


    fn init(&mut self) {
        self.uart.init();
        // devsw[CONSOLE].read = consoleread;
        // devsw[CONSOLE].write = consolewrite;
    }


    //
    // user read()s from the console go here.
    // copy (up to) a whole input line to dst.
    // user_dist indicates whether dst is a user
    // or kernel address.
    //
    // pub fn read(int user_dst, uint64 dst, int n)
    pub fn read(&mut self, n: usize) -> usize {
        let target: u64;
        let mut c: u8;
        let mut cbuf: u8;

        // target = n;
        target = 0;
        self.lock.aquire();

        while n > 0 {
            // wait until interrupt handler has put some
            // input into cons.buffer.
            while (self.r == self.w) {
                // if(killed(myproc())){
                // release(&cons.lock);
                //return -1;
            }
            // sleep(&cons.r, &cons.lock);

            c = self.buf[self.r % INPUT_BUF_SIZE];
            self.r += 1;

            // if(c == C('D')){  // end-of-file

            if c == (b'D' - b'@') {  // ^D = end-of-file
                // if(n < target){
                // // Save ^D for next time, to make sure
                // // caller gets a 0-byte result.
                // cons.r--;
                // }
                // break;
                // }
            }

            // copy the input byte to the user-space buffer.
            cbuf = c;
            // if(either_copyout(user_dst, dst, &cbuf, 1) == -1)
            // break;
            //
            // dst++;
            // --n;

            if (c == b'\n') {
                // a whole line has arrived, return to
                // the user-level read().
                break;
            }
        }
        self.lock.release();
        // return target - n;
        return 0;
    }
}