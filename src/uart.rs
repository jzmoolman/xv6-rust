// Port from xv6
//
// low-level driver routines for 16550a UART.
//
// JZ  Moolman


const RHR: isize = 0;                   // receive holding register (for input bytes)
const THR: isize = 0;                   // transmit holding register (for output bytes)
const IER: isize = 1;                   // interrupt enable register
const IER_RX_ENABLE: u8 =  1<<0;
const IER_TX_ENABLE: u8 =  1<<1;
const FCR: isize = 2;                   // FIFO control register
const FCR_FIFO_ENABLE: u8 = 1<<0;
const FCR_FIFO_CLEAR: u8 = 3<<1;        // clear the content of the two FIFOs
const ISR: isize = 2;                   // interrupt status register
const LCR: isize = 3;                   // line control register
const LCR_EIGHT_BITS: u8 = 3<<0;
const LCR_BAUD_LATCH: u8  = 1<<7;       // special mode to set baud rate
const LSR: isize = 5;                   // line status register
const LSR_RX_READY: u8 = 1<<0;          // input is waiting to be read from RHR
const LSR_TX_IDLE: u8 = 1<<5;           // THR can accept another character to send

const UART_TX_BUF_SIZE: usize = 32;

pub struct Uart {
    base: usize,
    tx_buffer: [u8; UART_TX_BUF_SIZE],
    tx_w: usize,                        // write next to uart_tx_buf[uart_tx_w % UART_TX_BUF_SIZE]
    tx_r: usize,                        // read next from uart_tx_buf[uart_tx_r % UART_TX_BUF_SIZE]
}

impl Uart {
    pub fn new(addr: usize) -> Self {
        Self {
            base: addr,
            tx_buffer: [0;UART_TX_BUF_SIZE],
            tx_r: 0,
            tx_w: 0,
        }
    }

    pub fn init(&self)  {
        let csr= self.base as *mut u8;
        unsafe  {
            // disable interrupts.
            csr.offset(IER as isize).write_volatile(0x00);

            // special mode to set baud rate.
            csr.offset(LCR as isize).write_volatile(LCR_BAUD_LATCH);


            // LSB for baud rate of 38.4K.
            csr.offset(0).write_volatile(0x03);

            // MSB for baud rate of 38.4K.
            csr.offset(1).write_volatile(0x00);

            // leave set-baud mode,
            // and set word length to 8 bits, no parity.
            csr.offset(LCR as isize).write_volatile(LCR_EIGHT_BITS);

            // reset and enable FIFOs.
            csr.offset(FCR as isize).write_volatile(FCR_FIFO_ENABLE | FCR_FIFO_CLEAR);

            // enable transmit and receive interrupts.
            csr.offset(IER as isize).write_volatile(IER_TX_ENABLE | IER_RX_ENABLE);
        }

    }

    pub fn putc(&mut self, char: u8) {
        while self.tx_w == self.tx_r + UART_TX_BUF_SIZE {
            //sleep;
        }
        self.tx_buffer[self.tx_w % UART_TX_BUF_SIZE] = char;
        self.tx_w += 1;

        self.start();
    }

    fn start(&mut self) {
    let csr = self.base as *mut u8;
        loop {
            if self.tx_w == self.tx_r {
                // transmit buffer is empty.
                unsafe {
                    csr.offset(IER as isize).read_volatile();
                }
                return;
            }
            unsafe {
                if csr.offset(LSR).read_volatile() & LSR_TX_IDLE == 0 {
                    // the UART transmit holding register is full,
                    // so we cannot give it another byte.
                    // it will interrupt when it's ready for a new byte.
                    return;
                }
            }
            let char = self.tx_buffer[self.tx_r];
            self.tx_r += 1;

            // maybe uartputc() is waiting for space in the buffer.
            // wakeup(&uart_tx_r);
            unsafe {
                csr.offset(THR as isize).write_volatile(char);
            }
        }
    }
}
