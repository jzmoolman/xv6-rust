// Port from xv6
//
// JZ  Moolman

pub fn memset(dst: u64, c: u8, n: isize) {
   let p = dst as *mut u8;
    for i in  0..n  {
        unsafe {
            p.offset(i).write_volatile(c);
        }
   }
}